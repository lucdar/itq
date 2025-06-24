use leptos::server_fn::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueueInfo {
    pub id: Uuid,
    pub url_name: String,
    pub display_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueueEntry {
    pub id: Uuid,
    pub queue_id: Uuid,
    pub order: i32,
    pub players: EntryPlayers,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
// TODO: Update this when players are added to database.
pub enum EntryPlayers {
    LeftOnly(String),
    RightOnly(String),
    Both(String, String),
}
