use crate::Profile;
use axum::extract::ws::{Message, WebSocket};
use axum::Error;
use paris::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "tag", content = "fields")]
#[serde(rename_all = "snake_case")]
pub enum SteamMessageOut {
    StatusUpdate { message: String },
    SelfProfile { profile: Profile },
    ProfileFetch { profile: Profile },
    Error { message: String },

    NameChange { name: String },
    PictureChange { url: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "tag", content = "fields")]
#[serde(rename_all = "snake_case")]
pub enum SteamMessageIn {
    Cookie { cookie: String },
    RefreshProfile,
    StealProfile { name: String, image_url: String },
    FetchProfile { url: String },
    ChangeName { name: String },
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

    pub async fn raw_send(&mut self, msg: Message) -> Result<(), Error> {
        self.ws.as_mut().unwrap().send(msg).await
    }

    async fn send(&mut self, text: String) {
        match &mut self.ws {
            Some(o) => {
                let _ = o.send(Message::Text(text)).await;
            }
            None => println!("{text}"),
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

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::message::{SteamMessageIn, SteamMessageOut};
    use crate::Profile;

    #[tokio::test]
    async fn test_message_out() -> Result<()> {
        let d = "dummy".to_string();

        let message = SteamMessageOut::SelfProfile {
            profile: Profile {
                url: d.clone(),
                name: d.clone(),
                image_url: d.clone(),
            },
        };

        let str = serde_json::to_string(&message)?;
        assert_eq!(
            str,
            r#"{"tag":"self_profile","fields":{"profile":{"name":"dummy","image_url":"dummy","url":"dummy"}}}"#
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_message_in() -> Result<()> {
        let d = "dummy".to_string();

        let str = r#"{"tag": "steal_profile", "fields":{"name": "dummy", "image_url": "dummy"}}"#;

        let serialized: SteamMessageIn = serde_json::from_str(str)?;
        assert_eq!(
            serialized,
            SteamMessageIn::StealProfile {
                name: d.clone(),
                image_url: d.clone(),
            }
        );

        Ok(())
    }
}
