mod add_player_modal;

use crate::queue::{EntryPlayers, QueueEntry, QueueInfo, Side};
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
                            children={ move |(idx, entry)| view!{ <Row entry idx/> } }
                        />
                    }.into_any(),
                    Err(e) => view! {
                        <p> "Error getting queue: " {e.to_string()} </p>
                    }.into_any()
                }
            })}
            <EmptyRow />
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

    view! {
        <div class="rowContainer">
            <div class="orderLabel">{idx}</div>
            <PlayerToken player_data=left />
            <PlayerToken player_data=right />
        </div>
    }
}

#[component]
pub fn EmptyRow() -> impl IntoView {
    view! {
        <div class="rowContainer">
            <div class="orderLabel">"-"</div>
            <PlayerToken player_data=None />
            <PlayerToken player_data=None />
        </div>
    }
}

// TODO: When players are their own "type", we could implement IntoView on the Player struct?
// Or have different methods like token_view() or info_view() etc.
// Might not really make sense unless it's used in many places.
#[component]
pub fn PlayerToken(player_data: Option<String>) -> impl IntoView {
    let (show_modal, write_show_modal) = signal(false);

    match player_data {
        None => view! {
            <div class="player-token empty">
                <button on:click={move |_| write_show_modal.set(true)}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                    </svg>
                </button>
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

#[server]
pub async fn add_player(
    queue_id: Uuid,
    player: String,
    order: usize,
    side: Side,
) -> Result<(), ServerFnError> {
    use crate::db::{api::add_player, DbPool};
    let pool = use_context::<DbPool>().expect("there to be a `pool` provided.");
    Ok(add_player(queue_id, player, order as i32, side, pool).await?)
}
