use reqwest::{Client, ClientBuilder};
use axum::http::header;
use scraper::Selector;
use anyhow::Context;
use std::ops::Deref;
use crate::multipart::data_profile_edit::DataProfileEdit;
use anyhow::Result;
use crate::message::WebsocketWrapper;

async fn get_profile_details(wrapper: &mut WebsocketWrapper) -> Result<(String, DataProfileEdit)> {
    let res = Client::new()
        .get(format!("{}/edit/info", wrapper.profile.url))
        .header(header::COOKIE, format!("steamLoginSecure={}", wrapper.cookie))
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .send()
        .await?;

    let session_id = res
        .cookies()
        .find(|c| c.name() == "sessionid")
        .context("no session id cookie")?
        .value()
        .to_owned();

    let text = res.text().await?;
    let data_profile_edit = {
        let document = scraper::Html::parse_document(&text);
        // #profile_edit_config[data-profile-edit=

        document
            .select(&Selector::parse("#profile_edit_config").unwrap())
            .filter_map(|e| e.value().attr("data-profile-edit"))
            .collect::<Vec<&str>>()
            .first()
            .context("Couldn't parse data profile edit from profile edit config element")?
            .deref()
            .to_owned()
    };

    Ok((session_id, serde_json::from_str::<DataProfileEdit>(&data_profile_edit)?))
}

pub async fn new_name_details(wrapper: &mut WebsocketWrapper, new_name: &str) -> Result<(String, [(String, String); 19])> {
    let (session_id, dpf) = get_profile_details(wrapper).await?;

    let fields = [
        s("sessionID", &session_id), // session id,
        s("type", "profileSave"),
        s("weblink_1_title", ""),
        s("weblink_1_url", ""),
        s("weblink_2_title", ""),
        s("weblink_2_url", ""),
        s("weblink_3_title", ""),
        s("weblink_3_url", ""),
        s("personaName", new_name),
        s("real_name", &dpf.str_real_name), // real name
        s("customURL", &dpf.str_custom_url), // custom url
        s("country", &dpf.location_data.loc_country_code), // country
        s("state", &dpf.location_data.loc_state_code), // state
        s("city", format!("{}", &dpf.location_data.loc_city_code)), // city
        s("summary", &dpf.str_summary), // summary
        s("hide_profile_awards", format!("{}", &dpf.profile_preferences.hide_profile_awards)), // hide profile awards (1/0)
        s("type", "profileSave"), // AGAIN
        s("sessionID", &session_id), // session id AGAIN
        s("json", "1")
    ];

    Ok((session_id, fields))
}

fn s<S: ToString>(name: &str, value: S) -> (String, String) {
    (name.to_owned(), value.to_string())
}
