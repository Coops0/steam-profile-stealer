use axum::extract::ws::{Message, WebSocket};
use paris::error;
use crate::{
    message::{SteamMessageIn, SteamMessageOut, WebsocketWrapper},
    profile::{get_self_profile, parse_profile},
    stealer::{headless_steam, image_to_base64},
};

pub async fn websocket(ws: WebSocket) {
    let mut wrapper = WebsocketWrapper::new(Some(ws));

    while let Some(msg) = wrapper.ws().as_mut().unwrap().recv().await {
        let text = match msg {
            Ok(Message::Text(t)) => t,
            Ok(Message::Close(_)) => return,
            _ => continue,
        };

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
                        wrapper.error(e).await;
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
                    Err(e) => wrapper.error(e).await,
                }
            }
            SteamMessageIn::StealProfile { image_url, name } => {
                if !image_url.starts_with("https://avatars.cloudflare.steamstatic.com/")
                    && !image_url.starts_with("https://avatars.akamai.steamstatic.com/")
                {
                    wrapper.error("bad image url").await;
                    continue;
                }

                if wrapper.profile_url.is_empty() {
                    wrapper.error("no profile url set yet").await;
                    continue;
                }

                let base64_image = match image_to_base64(&mut wrapper, &image_url).await {
                    Ok(o) => o,
                    Err(e) => {
                        wrapper.error(e).await;
                        continue;
                    }
                };

                if let Err(e) = headless_steam(&mut wrapper, &name, &base64_image).await {
                    error!("Error running headless steam {e:?}");
                    wrapper.error(e).await;
                    continue;
                }

                wrapper
                    .sm(SteamMessageOut::PictureChange { url: image_url })
                    .await;
                wrapper.log("Successfully stole profile!").await;
            }
        }
    }
}
