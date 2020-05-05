use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomInfo {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub member_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Queue {
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "queueName")]
    pub queue_name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Message {
    pub id: i32,
    #[serde(rename = "roomId")]
    pub room_id: i32,
    #[serde(rename = "senderId")]
    pub sender_id: i32,
    pub content: String,
}
