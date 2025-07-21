use crate::db;
use crate::db::QueueRow;
use crate::queue::*;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use leptos::logging::error;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("database connection pool error")]
    PoolError(#[from] diesel_async::pooled_connection::deadpool::PoolError),
    #[error("diesel error: {0}")]
    DieselError(#[from] diesel::result::Error),
    #[error("player slot already occupied. row: {row_id}, order: {order}, side: {side}")]
    Occupied {
        row_id: Uuid,
        order: i32,
        side: Side,
    },
    #[error("invalid order. expected: {expected}, got: {got}")]
    InvalidOrder { expected: i32, got: i32 },
}

pub async fn get_all_queues(
    pool: db::DbPool,
) -> Result<Vec<QueueInfo>, ApiError> {
    use crate::db::Queue;
    use db::schema::queues::dsl;

    let conn = &mut pool.get().await?;
    let queues: Vec<Queue> = dsl::queues
        .get_results(conn)
        .await?;

    Ok(queues
        .into_iter()
        .map(QueueInfo::from)
        .collect())
}

pub async fn get_queue_info(
    url_name: String,
    pool: db::DbPool,
) -> Result<QueueInfo, ApiError> {
    use crate::db::Queue;
    use db::schema::queues::dsl;
    let conn = &mut pool.get().await?;
    // Select the first queue with url_name matching `name`
    let dbq: Queue = dsl::queues
        .filter(dsl::url_name.eq(url_name))
        .first(conn)
        .await?;
    // My cat had this to say: =----r4eghf
    Ok(dbq.into())
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
        // This should already be enforced by the database.
        .filter_map(|r| {
            r.try_into()
                .inspect_err(|e| error!("{e}"))
                .ok()
        })
        .collect();
    Ok(rows)
}

pub async fn add_queue(
    display_name: String,
    url_name: String,
    pool: db::DbPool,
) -> Result<QueueInfo, ApiError> {
    use crate::db::{NewQueue, Queue};
    use db::schema::queues;
    let conn = &mut pool.get().await?;

    let new_queue = NewQueue {
        display_name,
        url_name,
    };
    let queue: Queue = diesel::insert_into(queues::table)
        .values(&new_queue)
        .get_result(conn)
        .await?;
    Ok(queue.into())
}

pub async fn delete_queue(
    queue_id: Uuid,
    pool: db::DbPool,
) -> Result<(), ApiError> {
    use db::schema::queues::dsl;
    let conn = &mut pool.get().await?;
    diesel::delete(dsl::queues.filter(dsl::id.eq(queue_id)))
        .execute(conn)
        .await
        .inspect_err(|e| error!("{e}"))?;
    Ok(())
}

/// Adds a player to an existing queue row.
pub async fn add_player_to_row(
    row_id: Uuid,
    player: String,
    side: Side,
    pool: db::DbPool,
) -> Result<Uuid, ApiError> {
    use db::schema::queue_rows::dsl;
    let conn = &mut pool.get().await?;

    // If a row ID is provided, query it and attempt to add the player
    let mut db_row: QueueRow = dsl::queue_rows
        .filter(dsl::id.eq(row_id))
        .first::<QueueRow>(conn)
        .await?;
    let target_slot = match side {
        Side::Left => &mut db_row.left_player_name,
        Side::Right => &mut db_row.right_player_name,
    };
    if target_slot.is_some() {
        return Err(ApiError::Occupied {
            row_id: db_row.id,
            order: db_row.queue_order,
            side,
        });
    }
    *target_slot = Some(player);
    // It might be possible to avoid this 2nd dbrt with some database
    // shenanigans, but I don't think it's worth figuring out right now.
    diesel::update(dsl::queue_rows.find(row_id))
        .set(&db_row)
        .execute(conn)
        .await?;
    return Ok(db_row.id);
}

pub async fn add_row(
    queue_id: Uuid,
    player: String,
    side: Side,
    pool: db::DbPool,
) -> Result<Uuid, ApiError> {
    use db::schema::queue_rows::dsl;
    let conn = &mut pool.get().await?;

    // Create new row
    let (left, right) = match side {
        Side::Left => (Some(player), None),
        Side::Right => (None, Some(player)),
    };
    let max_order = dsl::queue_rows
        .filter(dsl::queue_id.eq(queue_id))
        .select(dsl::queue_order)
        .order(dsl::queue_order.desc())
        .first::<i32>(conn)
        .await
        .optional()?;
    let new_row = db::NewQueueRow {
        queue_id,
        left_player_name: left,
        right_player_name: right,
        queue_order: max_order.map_or(0, |o| o + 1),
    };
    let new_row_id = diesel::insert_into(dsl::queue_rows)
        .values(&new_row)
        .returning(dsl::id)
        .get_result(conn)
        .await?;
    Ok(new_row_id)
}
