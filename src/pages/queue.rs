#[cfg(feature = "ssr")]
use crate::db;
use crate::queue::{PlayerData, QueueData, RowData, RowPlayerState};
#[cfg(feature = "ssr")]
use diesel::prelude::*;
#[cfg(feature = "ssr")]
use diesel_async::RunQueryDsl;

use leptos::logging::error;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use std::collections::VecDeque;
use uuid::Uuid;

#[component]
pub fn QueuePage() -> impl IntoView {
    let params = use_params_map();
    let url_queue_name = move || params.read().get("url_name");
    let queue_data: Resource<Option<QueueData>> = Resource::new(url_queue_name, |name| async {
        match name {
            Some(name) => queue_from_url_name(name).await.ok(),
            None => None,
        }
    });

    view! {
        <div class="queue-page">
            <p>"Now Viewing: "{url_queue_name}</p>
            <Suspense fallback=move || view! {<p>"Loading queue..."</p>}>
                {move || queue_data.get().flatten().map_or(
                    view! {<p>Error! No Queue Found</p>}.into_any(),
                    move |qd| view! {
                        <h1>{qd.display_name}</h1>
                        <p>"id "{qd.id.to_string()}</p>
                        <For
                            each={move || queue_data.get().unwrap().unwrap().rows}
                            key={ |r| r.id }
                            children={ move |r| view! {
                                <p>{r.id.to_string()}</p>
                            }}
                        />
                    }.into_any()
                )}
            </Suspense>
        </div>
    }
}

#[server]
pub async fn queue_from_url_name(name: String) -> Result<QueueData, ServerFnError> {
    use db::{schema::queues::dsl::*, Queue};
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
