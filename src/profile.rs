use std::future::Future;
use anyhow::{bail, Context};
use reqwest::Client;
use scraper::Selector;
use axum::Json;
use axum::response::{IntoResponse, Response};
use reqwest::redirect::Policy;
use paris::error;
use axum::http::StatusCode;
use serde::Deserialize;
use axum::extract::Query;
use crate::Profile;
use anyhow::Result;

#[derive(Deserialize)]
pub struct TestBody {
    cookie: String,
}

pub async fn get_self_profile<S: ToString>(cookie: S) -> Result<Profile> {
    let cookie = cookie.to_string();

    let client = Client::builder()
        .redirect(Policy::none())
        .build()
        .unwrap();

    // let cookie = "76561198286609782%7C%7CeyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyAiaXNzIjogInI6MTFEMF8yMjMxODEzRl9DRkNDRCIsICJzdWIiOiAiNzY1NjExOTgyODY2MDk3ODIiLCAiYXVkIjogWyAid2ViIiBdLCAiZXhwIjogMTY4NDAzNDk2NywgIm5iZiI6IDE2NzUzMDc2NTcsICJpYXQiOiAxNjgzOTQ3NjU3LCAianRpIjogIjE0M0JfMjI4NUNBOUJfRUJGNjgiLCAib2F0IjogMTY3ODMwMTkwMywgInJ0X2V4cCI6IDE2OTYzOTQwOTksICJwZXIiOiAwLCAiaXBfc3ViamVjdCI6ICI3MS4xOTEuODQuMjgiLCAiaXBfY29uZmlybWVyIjogIjcxLjE5MS44NC4yOCIgfQ.71ndyfohgopVd81ccE2I6snOSnz1uCokQYYe6e6FMT94YWeELAY5eszlgpMWIvp0QI4ANDFF6VzKIJo22-kiAg";

    let res = client.get("https://steamcommunity.com/my/profile")
        .header("cookie", format!("steamLoginSecure={cookie}"))
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .send()
        .await
        .context("failed to req (server error)")?;

    let location = res.headers()
        .get("Location")
        .map(|l| l.to_str().ok())
        .flatten()
        .unwrap_or_default();

    if location.is_empty() || location == "https://steamcommunity.com/login/home/?goto=%2Fmy%2Fprofile" {
        bail!("redirect is invalid (bad cookie)");
    }

    parse_profile(location)
        .await
        .context("couldn't parse profile (server error)")
}

pub async fn mine(Json(TestBody { cookie }): Json<TestBody>) -> Response {
    match get_self_profile(cookie).await {
        Ok(o) => Json(o).into_response(),
        Err(e) => {
            error!("failed to fetch profile -> {e:?}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct ProfileParam {
    profile_url: String,
}

pub async fn profile(Query(ProfileParam { mut profile_url }): Query<ProfileParam>) -> Response {
    if !profile_url.starts_with("https://steamcommunity.com/id/") {
        profile_url = format!("https://steamcommunity.com/id/{profile_url}");
    }

    match parse_profile(&profile_url).await {
        Ok(o) => Json(o).into_response(),
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn parse_profile(url: &str) -> anyhow::Result<Profile> {
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
            url: url.to_string(),
        }
    )
}
