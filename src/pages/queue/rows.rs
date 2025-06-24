use crate::queue::{EntryPlayers, QueueEntry, QueueInfo};
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn QueueRows(queue_data: ReadSignal<QueueInfo>) -> impl IntoView {
    let entries = Resource::new(move || queue_data.get().id, get_queue_entries);

    view! {
        <Suspense fallback=move || view! {<p>"Loading rows..."</p>}>
            { move || entries.get().map(|v| {
                match v {
                    Ok(v) => view! {
                        <For
                            each={ move || { v.clone().into_iter().enumerate() } }
                            key={ |(_, row)| row.id }
                            children={ move |(idx, entry)| view!{ <Row entry=entry idx/> } }
                        />
                    }.into_any(),
                    Err(e) => view! {
                        <p> "Error getting queue: " {e.to_string()} </p>
                    }.into_any()
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn Row(entry: QueueEntry, idx: usize) -> impl IntoView {
    let (left, right) = match entry.players {
        EntryPlayers::LeftOnly(left) => (Some(left), None),
        EntryPlayers::RightOnly(right) => (None, Some(right)),
        EntryPlayers::Both(left, right) => (Some(left), Some(right)),
    };

    // Idk if this even really makes sense?
    let (left, _) = signal(left);
    let (right, _) = signal(right);

    view! {
        <div class="rowContainer">
            <div class="orderLabel">{idx}</div>
            <PlayerToken player_data=left />
            <PlayerToken player_data=right />
        </div>
    }
}

// TODO: When players are their own "type", we could implement IntoView on the Player struct?
// Or have different methods like token_view() or info_view() etc.
// Might not really make sense unless it's used in many places.
#[component]
pub fn PlayerToken(player_data: ReadSignal<Option<String>>) -> impl IntoView {
    match player_data.get() {
        None => view! {
            <div class="player-token empty">
                <p>"Empty"</p>
            </div>
        }
        .into_any(),
        Some(name) => view! {
            <div class="player-token">
                <p>{name}</p>
            </div>
        }
        .into_any(),
    }
}

#[server]
pub async fn get_queue_entries(queue_id: Uuid) -> Result<Vec<QueueEntry>, ServerFnError> {
    use crate::db::{api::get_queue_entries, DbPool};
    let pool = use_context::<DbPool>().expect("there to be a `pool` provided.");
    Ok(get_queue_entries(queue_id, pool).await?)
}
