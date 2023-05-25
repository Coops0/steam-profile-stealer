use axum::extract::ws::{Message, WebSocket};
use paris::error;
use serde::{Deserialize, Serialize};
use crate::Profile;

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

    pub fn ws(&mut self) -> &mut Option<WebSocket> {
        &mut self.ws
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
