use crate::{message::WebsocketWrapper, Profile};
use anyhow::{bail, Context, Result};
use reqwest::{header, redirect::Policy, Client};
use scraper::Selector;
use std::ops::Deref;


pub async fn get_self_profile(wrapper: &mut WebsocketWrapper) -> Result<Profile> {
    let client = Client::builder().redirect(Policy::none()).build().unwrap();

    // let cookie = "76561198286609782%7C%7CeyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyAiaXNzIjogInI6MTFEMF8yMjMxODEzRl9DRkNDRCIsICJzdWIiOiAiNzY1NjExOTgyODY2MDk3ODIiLCAiYXVkIjogWyAid2ViIiBdLCAiZXhwIjogMTY4NDAzNDk2NywgIm5iZiI6IDE2NzUzMDc2NTcsICJpYXQiOiAxNjgzOTQ3NjU3LCAianRpIjogIjE0M0JfMjI4NUNBOUJfRUJGNjgiLCAib2F0IjogMTY3ODMwMTkwMywgInJ0X2V4cCI6IDE2OTYzOTQwOTksICJwZXIiOiAwLCAiaXBfc3ViamVjdCI6ICI3MS4xOTEuODQuMjgiLCAiaXBfY29uZmlybWVyIjogIjcxLjE5MS44NC4yOCIgfQ.71ndyfohgopVd81ccE2I6snOSnz1uCokQYYe6e6FMT94YWeELAY5eszlgpMWIvp0QI4ANDFF6VzKIJo22-kiAg";

    wrapper
        .log("Sending initial request to get redirect...")
        .await;

    let res = client.get("https://steamcommunity.com/my/profile")
        .header(header::COOKIE, format!("steamLoginSecure={}", wrapper.cookie))
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .send()
        .await
        .context("failed to req (server error)")?;

    wrapper
        .log("Successfully received response, Parsing location header...")
        .await;

    let mut location = res
        .headers()
        .get("Location")
        .and_then(|l| l.to_str().ok())
        .unwrap_or_default();

    if location.is_empty() || !(location.starts_with("https://steamcommunity.com/id/") || location.starts_with("https://steamcommunity.com/profiles/")) {
        bail!("Redirect is not a profile!");
    }

    wrapper
        .log(format!(
            "Found location as {location}, forwarding to parse profile function..."
        ))
        .await;

    location = location.strip_suffix("/profile").unwrap_or(location);

    parse_profile(wrapper, location)
        .await
        .context("couldn't parse profile (server error)")
}

pub async fn parse_profile(wrapper: &mut WebsocketWrapper, url: &str) -> Result<Profile> {
    wrapper
        .log(format!("Sending request to profile {url}..."))
        .await;

    let resp = Client::default().get(url)
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .send()
        .await?;

    wrapper
        .log("Successfully got profile response, parsing HTML...")
        .await;

    let text = resp.text().await?;
    let (name, image_url) = {
        let document = scraper::Html::parse_document(&text);
        // .actual_persona_name
        // .playerAvatarAutoSizeInner > img
        let name = document
            .select(&Selector::parse(".actual_persona_name").unwrap())
            .flat_map(|e| e.text())
            // for some fucking reason there are 2 elements,
            // one w/ the name and the other is " "
            .filter(|e| !e.is_empty())
            .collect::<Vec<&str>>()
            .first()
            .context("Couldn't find '.actual_persona_name' element")?
            .deref()
            .to_owned();

        let image_url = document
            .select(&Selector::parse(".playerAvatarAutoSizeInner > img").unwrap())
            .filter_map(|e| e.value().attr("src"))
            .map(str::to_owned)
            .collect::<String>();

        (name, image_url)
    };

    let start_bytes = text.find(r#""steamid":"#)
        .context("no start steam id found in html")? + 11;
    let end_bytes = text.find(r#","personaname":""#)
        .context("no end quote? this shouldn't happen")? - 1;

    let id = text.get(start_bytes..end_bytes)
        .context("failed to get start bytes to end bytes index")?
        .to_owned();

    wrapper
        .log(format!(
            "Successfully parsed profile with name of {name} and image url of {image_url}"
        ))
        .await;

    Ok(Profile {
        name,
        image_url,
        url: url.to_owned(),
        id,
    })
}

#[cfg(test)]
mod tests {
    use crate::message::WebsocketWrapper;
    use crate::profile::{get_self_profile, parse_profile};
    use anyhow::Result;

    const AUTH_COOKIE: &str = "76561198286609782%7C%7CeyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyAiaXNzIjogInI6MTFEMF8yMjMxODEzRl9DRkNDRCIsICJzdWIiOiAiNzY1NjExOTgyODY2MDk3ODIiLCAiYXVkIjogWyAid2ViIiBdLCAiZXhwIjogMTY4NDQzODUyNywgIm5iZiI6IDE2NzU3MTE2MjMsICJpYXQiOiAxNjg0MzUxNjIzLCAianRpIjogIjBEMUVfMjI4REE3MDhfQzAxQ0EiLCAib2F0IjogMTY3ODMwMTkwMywgInJ0X2V4cCI6IDE2OTYzOTQwOTksICJwZXIiOiAwLCAiaXBfc3ViamVjdCI6ICI3MS4xOTEuODQuMjgiLCAiaXBfY29uZmlybWVyIjogIjcxLjE5MS44NC4yOCIgfQ.95-Oc8Q02HRQKEv2z82CV7M2KZ-BHwGU4pxQzb17_qKffoxM67WTb0vqTgwgv6s8F9PQXDwvpNsE9AL0JkM8BA";

    #[tokio::test]
    async fn get_self_profile_test() -> Result<()> {
        let mut wrapper = WebsocketWrapper::new(None);
        wrapper.cookie = AUTH_COOKIE.to_owned();

        let profile = get_self_profile(&mut wrapper).await?;
        assert_eq!(&profile.image_url, "https://avatars.akamai.steamstatic.com/244734af0daa603a8ae9f33f5dcb3a7c14c4d126_full.jpg");
        assert_eq!(&profile.name, "your friend");
        assert_eq!(&profile.url, "https://steamcommunity.com/id/coops_");

        Ok(())
    }

    #[tokio::test]
    async fn parse_profile_test() -> Result<()> {
        let mut wrapper = WebsocketWrapper::new(None);
        let profile = parse_profile(
            &mut wrapper,
            "https://steamcommunity.com/id/gabelogannewell",
        )
            .await?;

        assert!(&profile
            .image_url
            .ends_with("/c5d56249ee5d28a07db4ac9f7f60af961fab5426_full.jpg"));
        assert_eq!(&profile.name, "Rabscuttle");
        assert_eq!(
            &profile.url,
            "https://steamcommunity.com/id/gabelogannewell"
        );
        assert_eq!(&profile.id, "76561197960287930");

        Ok(())
    }
}
