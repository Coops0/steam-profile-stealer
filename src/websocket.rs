use anyhow::anyhow;
use crate::{message::{SteamMessageIn, SteamMessageOut, WebsocketWrapper}, profile::{get_self_profile, parse_profile}, Profile};
use axum::extract::ws::{Message, WebSocket};
use paris::error;
use crate::multipart::update::update;

pub async fn websocket(ws: WebSocket) {
    let mut wrapper = WebsocketWrapper::new(Some(ws));

    while let Some(msg) = wrapper.ws().as_mut().unwrap().recv().await {
        let text = match msg {
            Ok(Message::Text(t)) => t,
            Ok(Message::Close(_)) => return,
            _ => continue,
        };

        if &text == "ping" {
            let _ = wrapper.raw_send(Message::Text("pong".to_owned())).await;
            continue;
        }

        let msg = match serde_json::from_str::<SteamMessageIn>(&text) {
            Ok(o) => o,
            Err(e) => {
                error!("{e:?}");
                continue;
            }
        };

        if wrapper.cookie.is_empty() && !matches!(msg, SteamMessageIn::Cookie { .. }) {
            wrapper.error(anyhow!("You need to set a cookie first.")).await;
            continue;
        }

        if let SteamMessageIn::Cookie { cookie } = &msg {
            wrapper.cookie = cookie.clone();
        }

        match msg {
            SteamMessageIn::Cookie { .. } | SteamMessageIn::RefreshProfile => {
                match get_self_profile(&mut wrapper).await {
                    Ok(profile) => {
                        wrapper.profile = profile.clone();

                        wrapper.log("Successfully parsed profile!").await;
                        wrapper.sm(SteamMessageOut::SelfProfile { profile }).await;
                    }
                    Err(e) => {
                        wrapper.cookie = String::new();
                        wrapper.profile = Profile::default();
                        wrapper.error(e.context("Failed to use cookie")).await;
                    }
                }
            }
            SteamMessageIn::FetchProfile { url } => {
                let url = url_normalizer(url);

                match parse_profile(&mut wrapper, &url).await {
                    Ok(profile) => {
                        wrapper.log("Successfully parsed profile!").await;
                        wrapper.sm(SteamMessageOut::ProfileFetch { profile }).await;
                    }
                    Err(e) => wrapper.error(e.context("Failed to fetch profile!")).await,
                }
            }
            SteamMessageIn::StealProfile { image_url, name } => {
                if !image_url.starts_with("https://avatars.cloudflare.steamstatic.com/")
                    && !image_url.starts_with("https://avatars.akamai.steamstatic.com/")
                {
                    wrapper.error(anyhow!("Bad image url!")).await;
                    continue;
                }

                if wrapper.profile.url.is_empty() {
                    wrapper
                        .error(anyhow!("No profile url has been set yet? This should be impossible."))
                        .await;
                    continue;
                }


                if let Err(e) = update(&mut wrapper, &name, Some(&image_url)).await {
                    error!("Error updating something {e:?}");
                    wrapper.error(e).await;
                    continue;
                }

                wrapper.log("Successfully stole profile!").await;
            }
            SteamMessageIn::ChangeName { name } => {
                match update(&mut wrapper, &name, None).await {
                    Ok(_) => {
                        wrapper.log("Successfully changed name!").await;
                    }
                    Err(e) => {
                        wrapper.error(e.context("Failed to change name!")).await;
                        continue;
                    }
                }
            }
        }
    }
}

fn url_normalizer(url: String) -> String {
    if url.starts_with("https://steamcommunity.com/profiles/") || url.starts_with("https://steamcommunity.com/id/") {
        return url;
    }

    if url.len() == 17 {
        return format!("https://steamcommunity.com/profiles/{url}");
    }


    format!("https://steamcommunity.com/id/{url}")
}