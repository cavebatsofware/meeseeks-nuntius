use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomData {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub member_count: Option<u32>,
}
