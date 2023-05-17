use anyhow::Result;
use base64::Engine;
use base64::engine::general_purpose;
use chromiumoxide::{Browser, BrowserConfig};
use crate::websocket::{Messager, SteamMessageOut, WebsocketWrapper};
use chromiumoxide::cdp::browser_protocol::network::CookieParam;
use futures::StreamExt;
use reqwest::get;

pub async fn image_to_base64(wrapper: &mut WebsocketWrapper, image_url: &str) -> Result<String> {
    wrapper.log("Requesting image from url").await;
    let image = get(image_url).await?.bytes().await?;

    wrapper.log("Requested image, encoding base64...").await;
    Ok(general_purpose::STANDARD.encode(image))
}

pub async fn headless_steam(wrapper: &mut WebsocketWrapper, our_url: &str, name: &str, base64_image: &str) -> Result<()> {
    wrapper.log("Launching new headless chrome instance...").await;
    let (mut browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build().unwrap()).await?;

    let handle = tokio::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });


    wrapper.log("Launched headless chrome instance, setting up cookie...").await;

    let page = browser.new_page("about:blank").await?;
    page.set_cookie(CookieParam::new("steamLoginSecure", &wrapper.cookie)).await?;

    let page = browser.new_page(format!("{our_url}/edit/info")).await?;

    // wait to load
    let _ = page.wait_for_navigation().await?;

    wrapper.log("Page successfully loaded, typing into input...").await;

    // clear old name
    page
        .evaluate("() => {document.querySelector('input[name=personaName]').clear()}")
        .await?;

    page.find_element("input[name=personaName]")
        .await?
        .click()
        .await?
        .type_str(name)
        .await?;

    wrapper.log("Successfully typed into input, submitting...").await;

    page.find_element("button[type=submit]")
        .await?
        .click()
        .await?;

    let _ = page.wait_for_navigation().await?.content().await?;

    wrapper.log("Finished changing name, navigating to edit avatar.").await;
    wrapper.sm(SteamMessageOut::NameChange { name: name.to_owned() }).await;

    let page = browser.new_page(format!("{our_url}/edit/avatar")).await?;

    wrapper.log("Navigating to avatar page, running script to update image.").await;

    page
        .evaluate(include_str!("../image_stealer.js").replace("{image_base64}", base64_image))
        .await?;

    wrapper.log("Finished updating image.").await;

    let _ = page.wait_for_navigation().await?.content().await?;

    wrapper.log("Page finished navigation, closing browser.").await;

    browser.close().await?;
    let _ = handle.await;
    Ok(())
}

#[cfg(test)]
#[async_trait]
mod tests {
    use axum::async_trait;
    use anyhow::Result;

    #[test]
    async fn test_image_shit() -> Result<()> {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
