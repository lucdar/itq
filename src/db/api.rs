use crate::db;
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
