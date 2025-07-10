mod add_player_modal;

use crate::queue::{EntryPlayers, QueueEntry, QueueInfo, Side};
use leptos::server_fn::serde::{Deserialize, Serialize};
use leptos::{
    logging::{error, log},
    prelude::*,
};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalQueueEntry {
    id: Uuid,
    left: RwSignal<Option<String>>,
    right: RwSignal<Option<String>>,
}

#[component]
pub fn QueueRows() -> impl IntoView {
    let queue_info = use_context::<QueueInfo>()
        .expect("there to be a `queue_info` provided.");

    let entry_store: Resource<Result<Vec<LocalQueueEntry>, ServerFnError>> =
        Resource::new(
            || (),
            move |_| async move {
                get_queue_entries(queue_info.id)
                    .await
                    .map(|entries| {
                        entries
                            .into_iter()
                            .map(|entry| {
                                let (left, right) = match entry.players {
                                    EntryPlayers::LeftOnly(left) => {
                                        (Some(left), None)
                                    }
                                    EntryPlayers::RightOnly(right) => {
                                        (None, Some(right))
                                    }
                                    EntryPlayers::Both(left, right) => {
                                        (Some(left), Some(right))
                                    }
                                };
                                LocalQueueEntry {
                                    id: entry.id,
                                    left: RwSignal::new(left),
                                    right: RwSignal::new(right),
                                }
                            })
                            .collect()
                    })
                    .inspect_err(|e| {
                        error!("Error getting queue entries: {}", e);
                    })
            },
        );

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading rows..."</p> }
        }>
            {move || match entry_store.get() {
                Some(Ok(entries)) => {
                    view! {
                        <For
                            each=move || { entries.clone().into_iter().enumerate() }
                            key=|(_, entry)| entry.id
                            children=move |(order, entry)| view! { <Row entry order /> }
                        />
                    }
                        .into_any()
                }
                Some(Err(e)) => {
                    view! { <p>"Error loading rows: "{e.to_string()}</p> }
                        .into_any()
                }
                None => ().into_any(),
            }} <EmptyRow />
        </Suspense>
    }
}

#[component]
pub fn Row(entry: LocalQueueEntry, order: usize) -> impl IntoView {
    view! {
        <div class="rowContainer">
            <div class="orderLabel">{order + 1}</div>
            <PlayerToken
                player_data=entry.left.into()
                side=Side::Left
                id=Some(entry.id)
            />
            <PlayerToken
                player_data=entry.right.into()
                side=Side::Right
                id=Some(entry.id)
            />
        </div>
    }
}

#[component]
pub fn EmptyRow() -> impl IntoView {
    view! {
        <div class="rowContainer">
            <div class="orderLabel">"-"</div>
            <PlayerToken
                player_data=Signal::derive(move || None)
                side=Side::Left
                id=None
            />
            <PlayerToken
                player_data=Signal::derive(move || None)
                side=Side::Right
                id=None
            />
        </div>
    }
}

// TODO: When players are their own "type", we could implement IntoView on the
// Player struct? Or have different methods like token_view() or info_view()
// etc. Might not really make sense unless it's used in many places.
#[component]
pub fn PlayerToken(
    player_data: Signal<Option<String>>,
    side: Side,
    id: Option<Uuid>,
) -> impl IntoView {
    // TODO: implement this
    // let set_modal_state = expect_context::<WriteSignal<Option<(Option<Uuid>, Side)>>>();

    view! {
        <Show
            when=move || player_data.get().is_none()
            fallback=move || {
                view! {
                    <div class="player-token">
                        <p>{player_data.get().unwrap()}</p>
                    </div>
                }
            }
        >
            <div class="player-token empty">
                <button>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="w-6 h-6"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M12 4.5v15m7.5-7.5h-15"
                        />
                    </svg>
                </button>
            </div>
        </Show>
    }
}

#[server]
pub async fn get_queue_entries(
    queue_id: Uuid,
) -> Result<Vec<QueueEntry>, ServerFnError> {
    use crate::db::{api::get_queue_entries, DbPool};
    let pool = use_context::<DbPool>().expect("there to be a `pool` provided.");
    Ok(get_queue_entries(queue_id, pool).await?)
}
