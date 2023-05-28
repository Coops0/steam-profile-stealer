use crate::message::{SteamMessageOut, WebsocketWrapper};
use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose, Engine};
use chromiumoxide::{cdp::browser_protocol::network::CookieParam, Browser, BrowserConfig};
use futures::StreamExt;
use reqwest::get;

pub async fn image_to_base64(wrapper: &mut WebsocketWrapper, image_url: &str) -> Result<String> {
    wrapper.log("Requesting image from url").await;
    let image = get(image_url).await?.bytes().await?;

    wrapper.log("Requested image, encoding base64...").await;
    Ok(general_purpose::STANDARD.encode(image))
}

pub async fn headless_steam(
    wrapper: &mut WebsocketWrapper,
    name: &str,
    base64_image: &str,
) -> Result<()> {
    wrapper
        .log("Launching new headless chrome instance...")
        .await;

    let config =
        BrowserConfig::builder()
            .build()
            .map_err(|e| anyhow!(e))?;

    let (mut browser, mut handler) = Browser::launch(config)
        .await
        .context("failure to launch browser")?;

    let handle = tokio::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    wrapper
        .log("Launched headless chrome instance, setting up cookie...")
        .await;

    let page = browser.new_page("about:blank").await?;
    let cookie = CookieParam::builder()
        .name("steamLoginSecure")
        .value(&wrapper.cookie)
        .domain("steamcommunity.com")
        .url("https://steamcomminity.com")
        .build()
        .map_err(|e| anyhow!(e))?;

    page.set_cookie(cookie).await?;

    wrapper
        .log("Set cookie, navigating to edit info page...")
        .await;
    page.goto(format!("{}/edit/info", wrapper.profile_url))
        .await?;

    wrapper.log("Navigated to edit info page, loading...").await;
    // wait to load
    let _ = page.wait_for_navigation_response().await?;

    wrapper
        .log("Page successfully loaded, clearing input...")
        .await;

    // page.save_screenshot(
    //     ScreenshotParams::builder()
    //         .full_page(true)
    //         .build(),
    //     "test.png",
    // ).await?;

    // clear old name
    page.evaluate("() => {document.querySelector('input[name=personaName]').clear()}")
        .await
        .context("Failed to execute clear input script (couldn't find personaName)")?;

    wrapper
        .log("Cleared input, now typing into personaName")
        .await;

    page.find_element("input[name=personaName]")
        .await?
        .click()
        .await?
        .type_str(name)
        .await?;

    wrapper
        .log("Successfully typed into input, submitting...")
        .await;

    page.find_element("button[type=submit]")
        .await?
        .click()
        .await?;

    page.wait_for_navigation().await?;

    wrapper
        .log("Finished changing name, navigating to edit avatar.")
        .await;
    wrapper
        .sm(SteamMessageOut::NameChange {
            name: name.to_owned(),
        })
        .await;

    page.goto(format!("{}/edit/avatar", wrapper.profile_url))
        .await?;

    wrapper
        .log("Navigating to avatar page, running script to update image.")
        .await;

    page.evaluate(include_str!("../image_stealer.js").replace("{image_base64}", base64_image))
        .await
        .context("Failed to execute image stealer script")?;

    wrapper.log("Finished updating image.").await;

    let _ = page.wait_for_navigation().await?;

    wrapper
        .log("Navigating to profile to clear aliases...")
        .await;
    page.goto(&wrapper.profile_url).await?;
    let _ = page.wait_for_navigation_response().await?;

    wrapper.log("Got to profile, opening dialog...").await;

    page.evaluate("ShowClearAliasDialog()")
        .await
        .context("Failed to run ShowClearAliasDialog() function")?;
    wrapper
        .log("Ran ShowClearAliasDialog() function, clicking button")
        .await;

    let _ = page.wait_for_navigation_response().await?;

    page.find_element(".btn_green_steamui")
        .await?
        .click()
        .await?;

    wrapper.log("Clicked clear aliases button.").await;
    let _ = page.wait_for_navigation_response().await?;

    wrapper
        .log("Page finished navigation, closing browser.")
        .await;

    browser.close().await?;
    let _ = handle.await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::message::WebsocketWrapper;
    use crate::profile::get_self_profile;
    use crate::stealer::{headless_steam, image_to_base64};
    use anyhow::Result;

    const AUTH_COOKIE: &str = "76561198286609782%7C%7CeyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyAiaXNzIjogInI6MTFEMF8yMjMxODEzRl9DRkNDRCIsICJzdWIiOiAiNzY1NjExOTgyODY2MDk3ODIiLCAiYXVkIjogWyAid2ViIiBdLCAiZXhwIjogMTY4NDQzODUyNywgIm5iZiI6IDE2NzU3MTE2MjMsICJpYXQiOiAxNjg0MzUxNjIzLCAianRpIjogIjBEMUVfMjI4REE3MDhfQzAxQ0EiLCAib2F0IjogMTY3ODMwMTkwMywgInJ0X2V4cCI6IDE2OTYzOTQwOTksICJwZXIiOiAwLCAiaXBfc3ViamVjdCI6ICI3MS4xOTEuODQuMjgiLCAiaXBfY29uZmlybWVyIjogIjcxLjE5MS44NC4yOCIgfQ.95-Oc8Q02HRQKEv2z82CV7M2KZ-BHwGU4pxQzb17_qKffoxM67WTb0vqTgwgv6s8F9PQXDwvpNsE9AL0JkM8BA";

    #[tokio::test]
    async fn headless_steam_test() -> Result<()> {
        let mut wrapper = WebsocketWrapper::new(None);
        wrapper.cookie = AUTH_COOKIE.to_owned();
        wrapper.profile_url = "https://steamcommunity.com/id/coops_".to_owned();

        let img = image_to_base64(&mut wrapper, "https://avatars.cloudflare.steamstatic.com/3c783af9215d49b3daa150d95444489aede2f855_full.jpg").await?;

        headless_steam(&mut wrapper, "your friend", &img).await?;

        let profile = get_self_profile(&mut wrapper).await?;
        assert_eq!(&profile.name, "your friend");
        assert_eq!(profile.url, wrapper.profile_url);

        Ok(())
    }
}
