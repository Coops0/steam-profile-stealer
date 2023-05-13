use anyhow::{bail, Context};
use reqwest::Client;
use scraper::Selector;


use reqwest::redirect::Policy;


use crate::Profile;
use anyhow::Result;
use crate::websocket::WebsocketWrapper;

pub async fn get_self_profile(wrapper: &mut WebsocketWrapper) -> Result<Profile> {
    let client = Client::builder()
        .redirect(Policy::none())
        .build()
        .unwrap();

    // let cookie = "76561198286609782%7C%7CeyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyAiaXNzIjogInI6MTFEMF8yMjMxODEzRl9DRkNDRCIsICJzdWIiOiAiNzY1NjExOTgyODY2MDk3ODIiLCAiYXVkIjogWyAid2ViIiBdLCAiZXhwIjogMTY4NDAzNDk2NywgIm5iZiI6IDE2NzUzMDc2NTcsICJpYXQiOiAxNjgzOTQ3NjU3LCAianRpIjogIjE0M0JfMjI4NUNBOUJfRUJGNjgiLCAib2F0IjogMTY3ODMwMTkwMywgInJ0X2V4cCI6IDE2OTYzOTQwOTksICJwZXIiOiAwLCAiaXBfc3ViamVjdCI6ICI3MS4xOTEuODQuMjgiLCAiaXBfY29uZmlybWVyIjogIjcxLjE5MS44NC4yOCIgfQ.71ndyfohgopVd81ccE2I6snOSnz1uCokQYYe6e6FMT94YWeELAY5eszlgpMWIvp0QI4ANDFF6VzKIJo22-kiAg";

    wrapper.log("Sending initial request to get redirect...").await;

    let res = client.get("https://steamcommunity.com/my/profile")
        .header("cookie", format!("steamLoginSecure={}", wrapper.cookie))
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .send()
        .await
        .context("failed to req (server error)")?;

    wrapper.log("Received response, Parsing location header...").await;

    let location = res.headers()
        .get("Location")
        .and_then(|l| l.to_str().ok())
        .unwrap_or_default();

    if location.is_empty() || location == "https://steamcommunity.com/login/home/?goto=%2Fmy%2Fprofile" {
        bail!("redirect is invalid (bad cookie)");
    }

    wrapper.log(format!("Found location as {location}, forwarding to parse profile function...")).await;

    parse_profile(wrapper, location)
        .await
        .context("couldn't parse profile (server error)")
}

pub async fn parse_profile(wrapper: &mut WebsocketWrapper, url: &str) -> anyhow::Result<Profile> {
    wrapper.log(format!("Sending request to your profile {url}...")).await;
    let resp = Client::default().get(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .send()
        .await?;

    wrapper.log("Got profile response, parsing HTML...").await;

    let text = resp.text().await?;
    let document = scraper::Html::parse_document(&text);
    // .actual_persona_name
    // .playerAvatarAutoSizeInner > img
    let name = document.select(&Selector::parse(".actual_persona_name").unwrap())
        .flat_map(|e| e.text())
        // for some fucking reason there are 2 elements,
        // one w/ the name and the other is " "
        .filter(|e| !e.is_empty())
        .collect::<String>();

    let icon_url = document.select(&Selector::parse(".playerAvatarAutoSizeInner > img").unwrap())
        .filter_map(|e| e.value().attr("src"))
        .map(|e| e.to_string())
        .collect::<String>();

    Ok(
        Profile {
            name,
            icon_url,
            url: url.to_string(),
        }
    )
}
