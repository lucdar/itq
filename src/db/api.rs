use crate::db;
use crate::queue::*;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use leptos::logging::error;
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

pub async fn get_all_queues(pool: db::DbPool) -> Result<Vec<QueueInfo>, ApiError> {
    use crate::db::Queue;
    use db::schema::queues::dsl;

    let conn = &mut pool.get().await?;
    let queues: Vec<Queue> = dsl::queues.get_results(conn).await?;

    Ok(queues
        .into_iter()
        .map(|queue| QueueInfo {
            id: queue.id,
            url_name: queue.url_name,
            display_name: queue.display_name,
        })
        .collect())
}

pub async fn get_queue_info(url_name: String, pool: db::DbPool) -> Result<QueueInfo, ApiError> {
    use crate::db::Queue;
    use db::schema::queues::dsl;
    let conn = &mut pool.get().await?;
    // Select the first queue with url_name matching `name`
    let dbq: Queue = dsl::queues
        .filter(dsl::url_name.eq(url_name))
        .first(conn)
        .await?;
    // My cat had this to say: =----r4eghf
    Ok(QueueInfo {
        id: dbq.id,
        url_name: dbq.url_name,
        display_name: dbq.display_name,
    })
}

pub async fn get_queue_entries(
    queue_id: Uuid,
    pool: db::DbPool,
) -> Result<Vec<QueueEntry>, ApiError> {
    use db::schema::queue_rows::dsl;
    let conn = &mut pool.get().await?;

    let db_rows = dsl::queue_rows
        .filter(dsl::queue_id.eq(queue_id))
        .order(dsl::queue_order.asc())
        .load::<db::QueueRow>(conn)
        .await?;

    let rows: Vec<QueueEntry> = db_rows
        .into_iter()
        // Throw out empty rows.
        // This shouldn't be possible anyway with the way the database is set up.
        .filter_map(|r| to_row_data(r).inspect_err(|e| error!("{e}")).ok())
        .collect();
    Ok(rows)
}

fn to_row_data(entry: db::QueueRow) -> Result<QueueEntry, ApiError> {
    let player_state = match (entry.left_player_name, entry.right_player_name) {
        (Some(left), Some(right)) => Ok(EntryPlayers::Both(left, right)),
        (Some(left), None) => Ok(EntryPlayers::LeftOnly(left)),
        (None, Some(right)) => Ok(EntryPlayers::RightOnly(right)),
        (None, None) => Err(ApiError::EmptyRow {
            row_id: entry.id,
            queue_id: entry.queue_id,
            order: entry.queue_order,
        }),
    }?;
    Ok(QueueEntry {
        id: entry.id,
        queue_id: entry.queue_id,
        order: entry.queue_order,
        players: player_state,
    })
}
