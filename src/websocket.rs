use axum::async_trait;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::Response;
use paris::error;
use serde::{Deserialize, Serialize};
use crate::Profile;
use crate::profile::{get_self_profile, parse_profile};
use crate::stealer::{headless_steam, image_to_base64};


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
    StealProfile { name: String, our_url: String, image_url: String },
    FetchProfile { url: String },
}

pub struct WebsocketWrapper {
    ws: WebSocket,
    pub cookie: String,
}

impl Messager for WebsocketWrapper {
    async fn sm(&mut self, message: SteamMessageOut) {
        let string = match serde_json::to_string(&message) {
            Ok(o) => o,
            Err(e) => {
                error!("Failed to serialize message {message:?} -> {e:?}");
                return;
            }
        };

        let _ = self.ws.send(Message::Text(string)).await;
    }
    async fn log<S: ToString>(&mut self, message: S) {
        let message = message.to_string();

        self.sm(SteamMessageOut::StatusUpdate { message }).await;
    }
    async fn error<E: ToString>(&mut self, error: E) {
        let error = error.to_string();

        self.sm(SteamMessageOut::Error { message: error }).await;
    }
}

pub trait Messager {
    fn cookie() -> String;

    async fn sm(&mut self, message: SteamMessageOut);
    async fn log<S: ToString>(&mut self, message: S);
    async fn error<E: ToString>(&mut self, error: E);
}


async fn websocket(ws: WebSocket) {
    let mut wrapper = WebsocketWrapper {
        ws,
        cookie: String::new(),
    };

    while let Some(msg) = wrapper.ws.recv().await {
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
            wrapper.cookie = cookie.to_owned();
        }

        match msg {
            SteamMessageIn::Cookie { .. } | SteamMessageIn::RefreshProfile => {
                match get_self_profile(&mut wrapper).await {
                    Ok(profile) => wrapper.sm(SteamMessageOut::SelfProfile { profile }).await,
                    Err(e) => {
                        wrapper.cookie = String::new();
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
            SteamMessageIn::StealProfile { image_url, name, our_url } => {
                if !image_url.starts_with("https://avatars.cloudflare.steamstatic.com/") {
                    wrapper.error("bad image url").await;
                    continue;
                }


                let base64_image = match image_to_base64(&mut wrapper, &image_url).await {
                    Ok(o) => o,
                    Err(e) => {
                        wrapper.error(e).await;
                        continue;
                    }
                };

                if let Err(e) = headless_steam(&mut wrapper, &our_url, &name, &base64_image).await {
                    wrapper.error(e).await;
                    continue;
                }

                wrapper.sm(SteamMessageOut::PictureChange { url: image_url }).await;
            }
        }
    }
}