use crate::db;
use crate::queue::*;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use leptos::logging::error;
use leptos::prelude::*;
use std::collections::VecDeque;
use uuid::Uuid;

#[server]
pub async fn queue_from_url_name(name: String) -> Result<QueueData, ServerFnError> {
    use crate::db::Queue;
    use db::schema::queues::dsl::*;
    // Select the db::Queue with the url_name = name.
    let pool = use_context::<db::DbPool>().expect("there to be a `pool` provided.");
    let conn = &mut pool.get().await?;
    // Select the first queue with url_name matching `name`
    let dbq: Queue = queues.filter(url_name.eq(name)).first(conn).await?;
    // My cat had this to say: =----r4eghf
    Ok(QueueData {
        id: dbq.id,
        url_name: dbq.url_name,
        display_name: dbq.display_name,
        rows: get_queue_rows(dbq.id).await?,
    })
}

#[server]
pub async fn get_queue_rows(q_id: Uuid) -> Result<VecDeque<RowData>, ServerFnError> {
    // TODO: figure out how to work around this polluted namespace...
    use db::schema::queue_rows::dsl::*;
    let pool = use_context::<db::DbPool>().expect("there to be a `pool` provided.");
    let conn = &mut pool.get().await?;

    let db_rows = queue_rows
        .filter(queue_id.eq(q_id))
        .order(queue_order.asc())
        .load::<db::QueueRow>(conn)
        .await?;

    let rows: VecDeque<RowData> = db_rows
        .into_iter()
        // Throw out empty rows.
        // This shouldn't be possible anyway with the way the database is set up.
        .filter_map(to_row_data)
        .collect();
    Ok(rows)
}

fn to_row_data(r: db::QueueRow) -> Option<RowData> {
    let player_state = match (r.left_player_name, r.right_player_name) {
        (Some(left), Some(right)) => Some(RowPlayerState::Both(
            PlayerData { name: left },
            PlayerData { name: right },
        )),
        (Some(left), None) => Some(RowPlayerState::LeftOnly(PlayerData { name: left })),
        (None, Some(right)) => Some(RowPlayerState::RightOnly(PlayerData { name: right })),
        (None, None) => {
            error!(
                "Empty queue row with id {} in queue {:?} at order {}",
                r.id, r.queue_id, r.queue_order
            );
            None
        }
    }?;
    Some(RowData {
        id: r.id,
        player_state,
    })
}
