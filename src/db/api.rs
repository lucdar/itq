use crate::db;
use crate::queue::*;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use leptos::logging::error;
use std::collections::VecDeque;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Empty row with id {row_id} in queue {queue_id} at order {order}")]
    EmptyRow {
        row_id: Uuid,
        queue_id: Uuid,
        order: i32,
    },
    #[error("database connection pool error")]
    PoolError(#[from] diesel_async::pooled_connection::deadpool::PoolError),
    #[error("diesel error")]
    DieselError(#[from] diesel::result::Error),
}

pub async fn get_queue(url_name: String, pool: db::DbPool) -> Result<QueueData, ApiError> {
    use crate::db::Queue;
    use db::schema::queues::dsl;
    let conn = &mut pool.get().await?;
    // Select the first queue with url_name matching `name`
    let dbq: Queue = dsl::queues
        .filter(dsl::url_name.eq(url_name))
        .first(conn)
        .await?;
    // My cat had this to say: =----r4eghf
    Ok(QueueData {
        id: dbq.id,
        url_name: dbq.url_name,
        display_name: dbq.display_name,
        rows: get_queue_rows(dbq.id, pool).await?,
    })
}

async fn get_queue_rows(queue_id: Uuid, pool: db::DbPool) -> Result<VecDeque<RowData>, ApiError> {
    use db::schema::queue_rows::dsl;
    let conn = &mut pool.get().await?;

    let db_rows = dsl::queue_rows
        .filter(dsl::queue_id.eq(queue_id))
        .order(dsl::queue_order.asc())
        .load::<db::QueueRow>(conn)
        .await?;

    let rows: VecDeque<RowData> = db_rows
        .into_iter()
        // Throw out empty rows.
        // This shouldn't be possible anyway with the way the database is set up.
        .filter_map(|r| to_row_data(r).inspect_err(|e| error!("{e}")).ok())
        .collect();
    Ok(rows)
}

fn to_row_data(row: db::QueueRow) -> Result<RowData, ApiError> {
    let player_state = match (row.left_player_name, row.right_player_name) {
        (Some(left), Some(right)) => Ok(RowPlayerState::Both(
            PlayerData { name: left },
            PlayerData { name: right },
        )),
        (Some(left), None) => Ok(RowPlayerState::LeftOnly(PlayerData { name: left })),
        (None, Some(right)) => Ok(RowPlayerState::RightOnly(PlayerData { name: right })),
        (None, None) => Err(ApiError::EmptyRow {
            row_id: row.id,
            queue_id: row.queue_id,
            order: row.queue_order,
        }),
    }?;
    Ok(RowData {
        id: row.id,
        player_state,
    })
}
