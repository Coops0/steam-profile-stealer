mod message;
mod profile;
mod stealer;
mod websocket;

use std::path::{Path, PathBuf};
use crate::websocket::websocket;
use anyhow::{bail, Result};
use axum::{
    extract::WebSocketUpgrade,
    response::Html,
    routing::get,
    Router,
    Server,
};
use chromiumoxide::{Browser, BrowserConfig, BrowserFetcher, BrowserFetcherOptions};
use paris::{info, success};
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

static CHROME_EXECUTABLE: OnceCell<PathBuf> = OnceCell::const_new();

#[tokio::main]
async fn main() -> Result<()> {
    CHROME_EXECUTABLE.get_or_init(|| async {
        download_chrome_oxide().await.unwrap()
    }).await;

    let app = Router::new().route("/", get(root)).route(
        "/ws",
        get(|ws: WebSocketUpgrade| async { ws.on_upgrade(websocket) }),
    );

    info!("Attempting to bind server...");
    let builder = Server::bind(&"0.0.0.0:8000".parse()?);

    success!("Successfully bound to port 8000");
    builder.serve(app.into_make_service()).await?;

    Ok(())
}

async fn download_chrome_oxide() -> Result<PathBuf> {
    let download_path = Path::new("./download");

    info!("Fetching chrome oxide install...");
    // if !download_path.exists() {
    //     info!("Download path did not exist, most likely downloading again...");
    tokio::fs::create_dir_all(download_path).await?;
    // }

    let fetcher = BrowserFetcher::new(
        BrowserFetcherOptions::builder()
            .with_path(download_path)
            .build()?,
    );

    let info = fetcher.fetch().await?;
    if !download_path.exists() {
        bail!("Download path still doesn't exist?");
    }

    let exe_path = info.executable_path;

    success!("Successfully fetched chrome oxide. Testing with browser launch. Path={} Size={}", exe_path.display(), exe_path.metadata()?.len());
    // test
    let (mut browser, _) = Browser::launch(
        BrowserConfig::builder()
            .chrome_executable(&exe_path)
            .build()
            .unwrap(),
    ).await?;

    browser.close().await?;
    success!("Successfully tested browser!");

    Ok(exe_path)
}

async fn root() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    pub name: String,
    pub image_url: String,
    pub url: String,
}
