use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomInfo {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub member_ids: Vec<i32>,
}
