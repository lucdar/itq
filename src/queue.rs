#[cfg(feature = "ssr")]
use crate::models::{Queue, QueueRow};
use std::collections::VecDeque;
use uuid::Uuid;

#[derive(Clone, Debug /*, Serialize, Deserialize */)]
pub struct QueueData {
    pub id: Uuid,
    pub url_name: String,
    pub display_name: String,
    rows: VecDeque<RowData>,
}

#[derive(Clone, Debug)]
pub struct RowData {
    player_state: RowPlayerState,
}

#[derive(Clone, Debug)]
pub enum RowPlayerState {
    LeftOnly(PlayerData),
    RightOnly(PlayerData),
    Both(PlayerData, PlayerData),
}

#[derive(Clone, Debug)]
pub struct PlayerData {
    name: String,
}

#[cfg(feature = "ssr")]
pub async fn queue_from_name(queue_name: &str) -> Option<QueueData> {
    todo!()
}

#[cfg(feature = "ssr")]
pub async fn get_queue_rows(id: Uuid) -> Option<VecDeque<QueueData>> {
    todo!()
}
