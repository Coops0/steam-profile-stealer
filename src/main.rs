mod message;
mod profile;
mod stealer;
mod websocket;

use std::path::{Path, PathBuf};
use crate::websocket::websocket;
use anyhow::Result;
use axum::{
    extract::WebSocketUpgrade,
    response::Html,
    routing::get,
    Router,
    Server,
};
use chromiumoxide::{BrowserFetcher, BrowserFetcherOptions};
use once_cell::sync::Lazy;
use paris::{info, log, success};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::sync::OnceCell;

const CHROME_EXECUTABLE: OnceCell<PathBuf> = OnceCell::const_new();

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
    if !download_path.exists() {
        info!("Download path did not exist, most likely downloading again...");
    }

    let _ = tokio::fs::create_dir_all(download_path).await;
    let fetcher = BrowserFetcher::new(
        BrowserFetcherOptions::builder()
            .with_path(&download_path)
            .build()?,
    );

    let info = fetcher.fetch().await?;

    success!("Successfully fetched chrome oxide");
    Ok(info.executable_path)
}

async fn root() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Profile {
    pub name: String,
    pub image_url: String,
    pub url: String,
}
