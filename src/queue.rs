use leptos::server_fn::serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueueData {
    pub id: Uuid,
    pub url_name: String,
    pub display_name: String,
    pub rows: VecDeque<RowData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RowData {
    pub id: Uuid,
    pub player_state: RowPlayerState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RowPlayerState {
    LeftOnly(PlayerData),
    RightOnly(PlayerData),
    Both(PlayerData, PlayerData),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerData {
    pub name: String,
}
