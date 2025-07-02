use leptos::server_fn::serde::{Deserialize, Serialize};
use std::fmt::Display;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Left => write!(f, "left"),
            Side::Right => write!(f, "right"),
        }
    }
}

#[cfg(feature = "ssr")]
use crate::db;
#[cfg(feature = "ssr")]
use thiserror::Error;

#[cfg(feature = "ssr")]
#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Empty row with id {row_id} in queue {queue_id} at order {order}")]
    EmptyRow {
        row_id: Uuid,
        queue_id: Uuid,
        order: i32,
    },
}

#[cfg(feature = "ssr")]
impl From<db::Queue> for QueueInfo {
    fn from(queue: db::Queue) -> Self {
        QueueInfo {
            id: queue.id,
            url_name: queue.url_name,
            display_name: queue.display_name,
        }
    }
}

#[cfg(feature = "ssr")]
impl TryFrom<db::QueueRow> for QueueEntry {
    type Error = ConversionError;

    fn try_from(db_row: db::QueueRow) -> Result<Self, ConversionError> {
        let player_state = match (db_row.left_player_name, db_row.right_player_name) {
            (Some(left), Some(right)) => Ok(EntryPlayers::Both(left, right)),
            (Some(left), None) => Ok(EntryPlayers::LeftOnly(left)),
            (None, Some(right)) => Ok(EntryPlayers::RightOnly(right)),
            (None, None) => Err(ConversionError::EmptyRow {
                row_id: db_row.id,
                queue_id: db_row.queue_id,
                order: db_row.queue_order,
            }),
        }?;
        Ok(QueueEntry {
            id: db_row.id,
            queue_id: db_row.queue_id,
            order: db_row.queue_order,
            players: player_state,
        })
    }
}
