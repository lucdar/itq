mod add_player_modal;

use crate::queue::{QueueEntry, QueueInfo, Side};
use add_player_modal::AddPlayerModal;
use leptos::server_fn::serde::{Deserialize, Serialize};
use leptos::{logging::error, prelude::*};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalQueueEntry {
    id: Uuid,
    left: RwSignal<Option<String>>,
    right: RwSignal<Option<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddModalState {
    Open {
        row_id: Option<Uuid>,
        side: Side,
        order: usize,
    },
    Closed,
}
pub type EntryStore = Vec<LocalQueueEntry>;
pub type EntryStoreResource = Resource<Result<EntryStore, ServerFnError>>;

#[component]
pub fn Rows() -> impl IntoView {
    let queue_info = use_context::<QueueInfo>()
        .expect("there to be a `queue_info` provided.");
    let (modal_state, set_modal_state) = signal(AddModalState::Closed);
    provide_context(modal_state);
    provide_context(set_modal_state);

    let entry_store_rsc: EntryStoreResource = Resource::new(
        || (),
        move |_| async move {
            get_queue_entries(queue_info.id)
                .await
                .map(|server_entries| {
                    server_entries
                        .into_iter()
                        .map(|entry| {
                            let (left, right) = entry.players.players_tuple();
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
            {move || match entry_store_rsc.get() {
                Some(Ok(entry_store)) => {
                    provide_context(entry_store.clone());
                    let max_order = entry_store.len();
                    view! {
                        <For
                            each=move || { entry_store.clone().into_iter().enumerate() }
                            key=|(_, entry)| entry.id
                            children=move |(order, entry)| view! { <Row entry order /> }
                        />
                        <EmptyRow order=max_order + 1 />
                        <AddPlayerModal
                            state=modal_state
                            set_state=set_modal_state
                        />
                    }
                        .into_any()
                }
                Some(Err(e)) => {
                    view! { <p>"Error loading rows: "{e.to_string()}</p> }
                        .into_any()
                }
                None => ().into_any(),
            }}
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
                order
            />
            <PlayerToken
                player_data=entry.right.into()
                side=Side::Right
                id=Some(entry.id)
                order
            />
        </div>
    }
}

#[component]
pub fn EmptyRow(order: usize) -> impl IntoView {
    view! {
        <div class="rowContainer">
            <div class="orderLabel">"-"</div>
            <PlayerToken
                player_data=Signal::derive(move || None)
                side=Side::Left
                id=None
                order
            />
            <PlayerToken
                player_data=Signal::derive(move || None)
                side=Side::Right
                id=None
                order
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
    order: usize,
    side: Side,
    id: Option<Uuid>,
) -> impl IntoView {
    // TODO: implement this
    let set_modal_state = expect_context::<WriteSignal<AddModalState>>();

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
                <button on:click=move |_| {
                    set_modal_state
                        .set(AddModalState::Open {
                            row_id: id,
                            side,
                            order,
                        });
                }>
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
    Ok(get_queue_entries(queue_id, pool)
        .await
        .inspect_err(|e| error!("Error getting queue entries: {}", e))?)
}
