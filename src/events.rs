use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageEventType {
    Read,
    Send,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEvent {
    pub id: i32,
    #[serde(rename = "eventType")]
    pub event_type: MessageEventType,
    #[serde(rename = "userId")]
    pub user_id: i32,
}
