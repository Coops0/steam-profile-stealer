mod stealer;
mod profile;

use anyhow::Result;
use axum::extract::{Query, RawQuery};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::{Json, Router, Server};
use axum::http::StatusCode;
use axum::routing::{get, post};
use once_cell::sync::Lazy;
use paris::{error, info, success};
use reqwest::Client;
use reqwest::redirect::Policy;
use scraper::{ElementRef, Selector};
use serde::{Deserialize, Serialize};


#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/mine", post(profile::mine))
        .route("/profile", get(profile::profile));

    info!("Attempting to bind server...");
    let builder = Server::bind(&"0.0.0.0:8000".parse()?);

    success!("Successfully bound to port 8000");
    builder.serve(app.into_make_service()).await?;

    Ok(())
}

async fn root() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub icon_url: String,
    pub url: String,
}