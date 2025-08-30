use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ChatMessage {
    #[serde(rename = "chat")]
    Chat { text: String },
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "chat")]
    Chat { username: String, text: String },
    #[serde(rename = "system")]
    System {message: String},
}