#[cfg(feature = "ssr")]
use crate::db;
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

// Maybe all this should go into db?

#[cfg(feature = "ssr")]
pub async fn queue_from_url_name(url_name: &str) -> Option<QueueData> {
    // Diesel stuff goes here?
    // Select the db::Queue with the url_name = name.
    // My cat had this to say: =----r4eghf

    todo!()
}

#[cfg(feature = "ssr")]
pub async fn get_queue_rows(id: Uuid) -> Option<VecDeque<RowData>> {
    // Here is where we're gonna get the QueueData
    // I imagine there's going to be some trasnformation from
    // Iterable<db::QueueRow> to VecDeque<RowData>?

    todo!()
}
