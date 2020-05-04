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
    #[serde(rename = "roomId")]
    pub room_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientEventQueueNameWrapper {
    #[serde(rename = "queueName")]
    pub queue_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomInfo {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub member_ids: Vec<i32>,
}
