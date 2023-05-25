mod stealer;
mod profile;
mod websocket;
mod message;

use anyhow::Result;
use axum::{
    response::Html,
    Router,
    Server,
    routing::get,
};
use axum::extract::WebSocketUpgrade;
use paris::{info, success};
use serde::{Deserialize, Serialize};
use crate::websocket::websocket;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/ws",
               get(|ws: WebSocketUpgrade| async { ws.on_upgrade(websocket) }
               ),
        );

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
