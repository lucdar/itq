mod add_player_modal;

use crate::queue::{EntryPlayers, QueueEntry, QueueInfo, Side};
use add_player_modal::AddPlayerModal;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn QueueRows() -> impl IntoView {
    let queue_info = use_context::<QueueInfo>().expect("there to be a `queue_info` provided.");
    let entries_resource = Resource::new(move || queue_info.id, get_queue_entries);

    view! {
        <Suspense fallback=move || view! {<p>"Loading rows..."</p>}>
            { move || entries_resource.get().map(|entries_result| {
                match entries_result {
                    Ok(entries) => {
                        let num_entries = entries.len();
                        view! {
                            <For
                                each={ move || { entries.clone().into_iter() } }
                                key={ |entry| entry.id }
                                children={ move |entry| view!{ <Row entry/> } }
                            />
                            <EmptyRow order={num_entries} />
                        }.into_any()
                    },
                    Err(e) => view! {
                        <p> "Error getting queue: " {e.to_string()} </p>
                    }.into_any()
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn Row(entry: QueueEntry) -> impl IntoView {
    let order: usize = entry.order.try_into().expect("order to be positive");

    let (left, right) = match entry.players {
        EntryPlayers::LeftOnly(left) => (Some(left), None),
        EntryPlayers::RightOnly(right) => (None, Some(right)),
        EntryPlayers::Both(left, right) => (Some(left), Some(right)),
    };

    view! {
        <div class="rowContainer">
            <div class="orderLabel">{order + 1}</div>
            <PlayerToken player_data=left order side=Side::Left />
            <PlayerToken player_data=right order side=Side::Right />
        </div>
    }
}

#[component]
pub fn EmptyRow(order: usize) -> impl IntoView {
    view! {
        <div class="rowContainer">
            <div class="orderLabel">"-"</div>
            <PlayerToken player_data=None order side=Side::Left />
            <PlayerToken player_data=None order side=Side::Right />
        </div>
    }
}

// TODO: When players are their own "type", we could implement IntoView on the Player struct?
// Or have different methods like token_view() or info_view() etc.
// Might not really make sense unless it's used in many places.
#[component]
pub fn PlayerToken(player_data: Option<String>, order: usize, side: Side) -> impl IntoView {
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
            <Show
                when={move || show_modal.get()}
                fallback={|| ()}
            >
                <AddPlayerModal order={order} side={side} />
            </Show>
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
