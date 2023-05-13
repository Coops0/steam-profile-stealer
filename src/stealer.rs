use serde::Deserialize;
use axum::{async_trait, Json};
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use crate::{profile, Profile};
use anyhow::Result;
use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, COOKIE, USER_AGENT};

#[derive(Deserialize)]
pub struct StealBody {
    cookie: String,
    profile_url: String,
}

async fn steal(Json(StealBody { cookie, mut profile_url }): Json<StealBody>) -> Response {
    if !profile_url.starts_with("https://steamcommunity.com/id/") {
        profile_url = format!("https://steamcommunity.com/id/{profile_url}");
    }

    let their_profile = match profile::parse_profile(&profile_url).await {
        Ok(o) => o,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    // ------WebKitFormBoundaryr5ejRMTYsE4sE75e
    let our_profile = match profile::get_self_profile(&cookie).await {
        Ok(o) => o,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };
}

// Content-Disposition: form-data; name="sessionID"
//
// 55a967cfd6f486d7ac559484
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="type"
//
// profileSave
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="weblink_1_title"
//
//
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="weblink_1_url"
//
//
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="weblink_2_title"
//
//
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="weblink_2_url"
//
//
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="weblink_3_title"
//
//
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="weblink_3_url"
//
//
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="personaName"
//
// your friendS
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="real_name"
//
//
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="customURL"
//
// coops_
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="country"
//
//
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="state"
//
//
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="city"
//
//
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="summary"
//
// No information given.
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="hide_profile_awards"
//
// 1
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="type"
//
// profileSave
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="sessionID"
//
// 55a967cfd6f486d7ac559484
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e
// Content-Disposition: form-data; name="json"
//
// 1
// ------WebKitFormBoundaryr5ejRMTYsE4sE75e--

async fn update_name<S: ToString>(cookie: S, our_profile: &Profile, target: &Profile) -> Result<()> {
    let boundary = "------WebKitFormBoundaryr5ejRMTYsE4sE75e";

    let req = Client::default()
        .post()
        .header(COOKIE, format!("steamLoginSecure={cookie}"))
        .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .header(CONTENT_TYPE, format!("multipart/form-data; boundary={boundary}"))

    // i'm sorry reqwest doesn't support custom boundaries
    let fields = [
        ("sessionID", ""), // session id,
        ("type", "profileSave"),
        ("weblink_1_title", ""),
        ("weblink_1_url", ""),
        ("weblink_2_title", ""),
        ("weblink_2_url", ""),
        ("weblink_3_title", ""),
        ("weblink_3_url", ""),
        ("personaName", &target.name),
        ("real_name", ""), // real name
        ("customURL", ""), // custom url
        ("country", ""), // country
        ("state", ""), // state
        ("city", ""), // city
        ("summary", ""), // summary
        ("hide_profile_awards", ""), // hide profile awards (1/0)
        ("type", "profileSave"), // AGAIN
        ("sessionID", ""), // session id AGAIN
        ("json", "1")
    ];

    // fields
    //     .map(|(name, value)| format!())
}

async fn update_picture<S: ToString>(cookie: S, our_profile: &Profile, target: &Profile) -> Result<()> {}