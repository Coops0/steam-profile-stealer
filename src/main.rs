mod stealer;
mod profile;
mod websocket;

use anyhow::Result;

use axum::response::{Html};
use axum::{Router, Server};

use axum::routing::{get};

use paris::{info, success};



use serde::{Deserialize, Serialize};


#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(websocket::websocket_handler));

    info!("Attempting to bind server...");
    let builder = Server::bind(&"0.0.0.0:8000".parse()?);

    success!("Successfully bound to port 8000");
    builder.serve(app.into_make_service()).await?;

    Ok(())
}

async fn root() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub icon_url: String,
    pub url: String,
}