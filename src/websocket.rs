use axum::{
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};
use paris::error;
use serde::{Deserialize, Serialize};
use crate::{
    Profile,
    profile::{get_self_profile, parse_profile},
    stealer::{headless_steam, image_to_base64},
};


pub async fn websocket_handler(
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(websocket)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SteamMessageOut {
    StatusUpdate { message: String },
    SelfProfile { profile: Profile },
    ProfileFetch { profile: Profile },
    Error { message: String },

    NameChange { name: String },
    PictureChange { url: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SteamMessageIn {
    Cookie { cookie: String },
    RefreshProfile,
    StealProfile { name: String, image_url: String },
    FetchProfile { url: String },
}

pub struct WebsocketWrapper {
    // Only None for testing
    ws: Option<WebSocket>,
    pub cookie: String,
    pub profile_url: String,
}

impl WebsocketWrapper {
    pub const fn new(ws: Option<WebSocket>) -> Self {
        Self {
            ws,
            cookie: String::new(),
            profile_url: String::new(),
        }
    }

    async fn send(&mut self, text: String) {
        match &mut self.ws {
            Some(o) => { let _ = o.send(Message::Text(text)).await; }
            None => println!("{text}")
        }
    }

    pub async fn sm(&mut self, message: SteamMessageOut) {
        let string = match serde_json::to_string(&message) {
            Ok(o) => o,
            Err(e) => {
                error!("Failed to serialize message {message:?} -> {e:?}");
                return;
            }
        };

        self.send(string).await;
    }

    pub async fn log<S: ToString>(&mut self, message: S) {
        let message = message.to_string();

        self.sm(SteamMessageOut::StatusUpdate { message }).await;
    }

    pub async fn error<E: ToString>(&mut self, error: E) {
        let error = error.to_string();

        self.sm(SteamMessageOut::Error { message: error }).await;
    }
}

async fn websocket(ws: WebSocket) {
    let mut wrapper = WebsocketWrapper::new(Some(ws));

    while let Some(msg) = wrapper.ws.as_mut().unwrap().recv().await {
        let text = match msg {
            Ok(Message::Text(t)) => t,
            Ok(Message::Close(_)) => return,
            _ => continue,
        };

        let msg = match serde_json::from_str::<SteamMessageIn>(&text) {
            Ok(o) => o,
            _ => continue,
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
                    Ok(profile) => wrapper.sm(SteamMessageOut::ProfileFetch { profile }).await,
                    Err(e) => wrapper.error(e).await,
                }
            }
            SteamMessageIn::StealProfile { image_url, name } => {
                if !image_url.starts_with("https://avatars.cloudflare.steamstatic.com/") && !image_url.starts_with("https://avatars.akamai.steamstatic.com/") {
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
                    wrapper.error(e).await;
                    continue;
                }

                wrapper.sm(SteamMessageOut::PictureChange { url: image_url }).await;
                wrapper.log("Success!").await;
            }
        }
    }
}