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
    #[error("diesel error")]
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

pub async fn get_all_queues(pool: db::DbPool) -> Result<Vec<QueueInfo>, ApiError> {
    use crate::db::Queue;
    use db::schema::queues::dsl;

    let conn = &mut pool.get().await?;
    let queues: Vec<Queue> = dsl::queues.get_results(conn).await?;

    Ok(queues.into_iter().map(QueueInfo::from).collect())
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
        .filter_map(|r| r.try_into().inspect_err(|e| error!("{e}")).ok())
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

pub async fn delete_queue(queue_id: Uuid, pool: db::DbPool) -> Result<(), ApiError> {
    use db::schema::queues::dsl;
    let conn = &mut pool.get().await?;
    diesel::delete(dsl::queues.filter(dsl::id.eq(queue_id)))
        .execute(conn)
        .await
        .inspect_err(|e| error!("{e}"))?;
    Ok(())
}

pub async fn add_player(
    queue_id: Uuid,
    player: String,
    order: i32,
    side: Side,
    pool: db::DbPool,
) -> Result<(), ApiError> {
    use db::schema::queue_rows::dsl;
    let conn = &mut pool.get().await?;

    // Query database for row that matches provided order
    let db_row: Option<QueueRow> = dsl::queue_rows
        .filter(dsl::queue_id.eq(queue_id))
        .filter(dsl::queue_order.eq(order))
        .first::<QueueRow>(conn)
        .await
        .optional()?;

    if let Some(mut row) = db_row {
        // A row with the given order already exists, so we're adding a player to it.
        let player_slot = match side {
            Side::Left => &mut row.left_player_name,
            Side::Right => &mut row.right_player_name,
        };
        if player_slot.is_some() {
            return Err(ApiError::Occupied {
                row_id: row.id,
                order: row.queue_order,
                side,
            });
        }
        *player_slot = Some(player);
        diesel::update(dsl::queue_rows.find(row.id))
            .set(&row)
            .execute(conn)
            .await?;
    } else {
        // The row does not exist, so we are creating a new entry at the end of the queue.
        use diesel::dsl::max;

        // First, verify that the new row is sequential
        let max_order: Option<i32> = dsl::queue_rows
            .filter(dsl::queue_id.eq(queue_id))
            .select(max(dsl::queue_order))
            .first(conn)
            .await?;

        // If no rows exist, the first order should be 0.
        let expected_order = max_order.map_or(0, |val| val + 1);
        if order != expected_order {
            return Err(ApiError::InvalidOrder {
                expected: expected_order,
                got: order,
            });
        }

        let (left_player_name, right_player_name) = match side {
            Side::Left => (Some(player), None),
            Side::Right => (None, Some(player)),
        };
        let new_row = db::NewQueueRow {
            queue_id,
            left_player_name,
            right_player_name,
            queue_order: order,
        };
        diesel::insert_into(dsl::queue_rows)
            .values(&new_row)
            .execute(conn)
            .await?;
    }

    Ok(())
}
