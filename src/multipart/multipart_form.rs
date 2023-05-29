use std::ops::Deref;
use anyhow::{anyhow, bail, Context, Result};
use mpart_async::client::MultipartRequest;
use mpart_async::filestream::FileStream;
use reqwest::{Body, Client, ClientBuilder, get, header, Request, StatusCode};
use reqwest::header::CONTENT_TYPE;
use scraper::{Element, Selector};
use serde::{Deserialize, Serialize};
use crate::message::WebsocketWrapper;
use crate::multipart::data_profile_edit::DataProfileEdit;
use crate::multipart::profile_details::new_name_details;

pub async fn update_name(
    wrapper: WebsocketWrapper,
    session_id: &str,
    fields: Vec<(String, String)>
) -> Result<UpdateNameResponse> {
    let mut mpart = MultipartRequest::<FileStream>::new("------WebKitFormBoundaryhSUnLTjMGhbdZ0Qg");
    for (name, value) in fields {
        mpart.add_field(name, value);
    }

    let res = Client::new()
        .post(format!("{}/edit/info", wrapper.profile.url))
        .header(header::COOKIE, format!("steamLoginSecure={};sessionid={session_id}", wrapper.cookie))
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .header(
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={}", mpart.get_boundary()),
        )
        .body(Body::wrap_stream(mpart))
        .send()
        .await?
        .json::<UpdateNameResponse>()
        .await?;

    Ok(res)
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateNameResponse {
    pub success: i64,
    #[serde(rename = "errmsg")]
    pub err_msg: String,
}

pub async fn update_image(wrapper: WebsocketWrapper, session_id: &str, image_url: &str) -> Result<UpdateImageResponse> {
    let mut mpart = MultipartRequest::new("------WebKitFormBoundaryhSUnLTjMGhbdZ0Qg");
    // stream

    let image = get(image_url).await?;

    mpart.add_stream("avatar", "joe.jpg", "image/jpeg", image.bytes_stream());


    mpart.add_field("type", "player_avatar_image");
    mpart.add_field("sessionid", &wrapper.profile.id);
    mpart.add_field("sId", session_id);
    mpart.add_field("doSub", "1");
    mpart.add_field("json", "1");

    let res = Client::new()
        .post("https://steamcommunity.com/actions/FileUploader/")
        .header(header::COOKIE, format!("steamLoginSecure={};sessionid={session_id}", wrapper.cookie))
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .header(
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={}", mpart.get_boundary()),
        )
        .body(Body::wrap_stream(mpart))
        .send()
        .await?
        .json::<UpdateImageResponse>()
        .await?;

    Ok(res)
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateImageResponse {
    pub success: bool,
    pub message: String,
}

pub async fn clear_aliases(wrapper: WebsocketWrapper, session_id: &str) -> Result<()> {
    let res = Client::new()
        .post(format!("{}/ajaxaliases/", wrapper.profile.url))
        .header(header::COOKIE, format!("steamLoginSecure={};sessionid={session_id}", wrapper.cookie))
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        bail!("Status code for clearing aliases was not 200");
    }

    Ok(())
}