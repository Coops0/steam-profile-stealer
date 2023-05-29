use crate::{
    message::{SteamMessageIn, SteamMessageOut, WebsocketWrapper},
    profile::{get_self_profile, parse_profile},
    stealer::{headless_steam, image_to_base64},
};
use axum::extract::ws::{Message, WebSocket};
use paris::error;

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
            wrapper.error("You need to set a cookie first.").await;
            continue;
        }

        if let SteamMessageIn::Cookie { cookie } = &msg {
            wrapper.cookie = cookie.clone();
        }

        match msg {
            SteamMessageIn::Cookie { .. } | SteamMessageIn::RefreshProfile => {
                match get_self_profile(&mut wrapper).await {
                    Ok(profile) => {
                        wrapper.profile_url = profile.url.clone();

                        wrapper.log("Successfully parsed profile!").await;
                        wrapper.sm(SteamMessageOut::SelfProfile { profile }).await;
                    }
                    Err(e) => {
                        wrapper.cookie = String::new();
                        wrapper.profile_url = String::new();
                        wrapper.error(e.context("Failed to use cookie")).await;
                    }
                }
            }
            SteamMessageIn::FetchProfile { mut url } => {
                if !url.starts_with("https://steamcommunity.com/id/") {
                    url = format!("https://steamcommunity.com/id/{url}");
                }

                match parse_profile(&mut wrapper, &url).await {
                    Ok(profile) => {
                        wrapper.log("Successfully parsed profile!").await;
                        wrapper.sm(SteamMessageOut::ProfileFetch { profile }).await;
                    }
                    Err(e) => wrapper.error(e.context("Failed to steal profile!")).await,
                }
            }
            SteamMessageIn::StealProfile { image_url, name } => {
                if !image_url.starts_with("https://avatars.cloudflare.steamstatic.com/")
                    && !image_url.starts_with("https://avatars.akamai.steamstatic.com/")
                {
                    wrapper.error("Bad image url!").await;
                    continue;
                }

                if wrapper.profile_url.is_empty() {
                    wrapper
                        .error("Nno profile url has been set yet? This should be impossible.")
                        .await;
                    continue;
                }

                let base64_image = match image_to_base64(&mut wrapper, &image_url).await {
                    Ok(o) => o,
                    Err(e) => {
                        wrapper.error(e).await;
                        continue;
                    }
                };

                if let Err(e) = headless_steam(&mut wrapper, &name, Some(&base64_image)).await {
                    error!("Error running headless steam {e:?}");
                    wrapper.error(e).await;
                    continue;
                }

                wrapper
                    .sm(SteamMessageOut::PictureChange { url: image_url })
                    .await;

                wrapper.log("Successfully stole profile!").await;
            }
            SteamMessageIn::ChangeName { name } => {
                match headless_steam(&mut wrapper, &name, None).await {
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
