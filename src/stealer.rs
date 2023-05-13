use crate::{Profile};
use anyhow::Result;
use chromiumoxide::{Browser, BrowserConfig};
use crate::websocket::WebsocketWrapper;
use chromiumoxide::cdp::browser_protocol::network::CookieParam;
use futures::StreamExt;


pub async fn headless_name_steal(wrapper: &mut WebsocketWrapper, our_url: &str, name: &str) -> Result<()> {
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

    wrapper.log("Page finished navigation, closing browser.");

    browser.close().await?;
    let _ = handle.await;
    Ok(())
}

async fn update_picture<S: ToString>(_cookie: S, _our_profile: &Profile, _target: &Profile) -> Result<()> {
    todo!()
}