mod add_player_modal;

use crate::queue::{QueueEntry, QueueInfo, Side};
use add_player_modal::AddPlayerModal;
use leptos::server_fn::serde::{Deserialize, Serialize};
use leptos::{logging::error, prelude::*};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum LocalUuidState {
    /// Resolved state for existing entries
    Resolved(Uuid),
    /// Pending state with a random UUID for new entries
    Pending(Uuid),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LocalQueueEntry {
    id: RwSignal<LocalUuidState>,
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

#[component]
pub fn Rows() -> impl IntoView {
    let queue_info = use_context::<QueueInfo>()
        .expect("there to be a `queue_info` provided.");
    let (modal_state, set_modal_state) = signal(AddModalState::Closed);
    provide_context(modal_state);
    provide_context(set_modal_state);

    // Load and unpack entries from server on page load.
    let entry_store_rsc: Resource<Result<Vec<LocalQueueEntry>, ServerFnError>> =
        Resource::new(
            || (), // No dependencies, only run on page load
            move |_| async move {
                get_queue_entries(queue_info.id)
                    .await
                    .map(|server_entries| {
                        server_entries
                            .into_iter()
                            .map(|entry| {
                                let (left, right) =
                                    entry.players.players_tuple();
                                LocalQueueEntry {
                                    id: RwSignal::new(
                                        LocalUuidState::Resolved(entry.id),
                                    ),
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

    let entry_store_signal = RwSignal::new(Vec::new());
    provide_context(entry_store_signal);
    // Update entry store signal when entries load
    Effect::new(move |_| {
        if let Some(Ok(entries)) = entry_store_rsc.get() {
            entry_store_signal.set(entries);
        }
    });

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading rows..."</p> }
        }>
            {move || {
                if let Some(Err(e)) = entry_store_rsc.get() {
                    return view! {
                        <p>"Error loading rows: "{e.to_string()}</p>
                    }
                        .into_any();
                }
                view! {
                    <For
                        each=move || {
                            entry_store_signal.get().into_iter().enumerate()
                        }
                        key=|(_, entry)| entry.id.get()
                        children=move |(order, entry)| view! { <Row entry order /> }
                    />
                    <EmptyRow order=entry_store_signal.with(|es| es.len()) />
                }
                    .into_any()
            }}
            <AddPlayerModal
                modal_state
                set_modal_state
            />
        </Suspense>
    }
}

#[component]
pub fn Row(entry: LocalQueueEntry, order: usize) -> impl IntoView {
    // "Deactivate" the row if there is no UUID on the frontend
    let is_inactive =
        move || matches!(entry.id.get(), LocalUuidState::Pending(_));
    // Signal that gets the entry id and wraps in Some
    let id = Signal::derive(move || Some(entry.id.get()));

    view! {
        <div class="rowContainer" class:inactive=is_inactive>
            <div class="orderLabel">{order + 1}</div>
            <PlayerToken
                player_data=entry.left.into()
                side=Side::Left
                id=id
                order
            />
            <PlayerToken
                player_data=entry.right.into()
                side=Side::Right
                id=id
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
                id=Signal::derive(move || None)
                order
            />
            <PlayerToken
                player_data=Signal::derive(move || None)
                side=Side::Right
                id=Signal::derive(move || None)
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
    id: Signal<Option<LocalUuidState>>,
) -> impl IntoView {
    let set_modal_state = expect_context::<WriteSignal<AddModalState>>();

    view! {
        <Show
            when=move || { player_data.get().is_none() }
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
                    leptos::logging::log!("Clicked on empty player token");
                    let row_id = match id.get() {
                        None | Some(LocalUuidState::Pending(_)) => None,
                        Some(LocalUuidState::Resolved(uuid)) => Some(uuid),
                    };
                    leptos::logging::log!("Row: {:?}", row_id);
                    set_modal_state
                        .set(AddModalState::Open {
                            row_id,
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
