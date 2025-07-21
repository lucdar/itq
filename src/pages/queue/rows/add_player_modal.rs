use crate::pages::queue::rows::{
    AddModalState, EntryStore, LocalQueueEntry, LocalUuidState,
};
use crate::queue::{QueueInfo, Side};
use leptos::logging::{error, log};
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn AddPlayerModal(
    modal_state: ReadSignal<AddModalState>,
    set_modal_state: WriteSignal<AddModalState>,
) -> impl IntoView {
    let queue_info = use_context::<QueueInfo>()
        .expect("there to be a `queue_info` provided.");
    let entry_store_signal = use_context::<RwSignal<EntryStore>>()
        .expect("there to be a `entry_store` provided.");

    let is_visible = move || modal_state.get() != AddModalState::Closed;

    let add_player = ServerAction::<AddPlayer>::new();
    let add_player_value = add_player.value();

    Effect::new(move |_| {
        let server_fn_result = add_player_value.get();
        log!(
            "EFFECT: AddPlayerModal value updated: {:?}",
            server_fn_result
        );
        // If a new row was created, update the optimistically rendered row.
        if let Some(Ok(Some((old_id, new_id)))) = server_fn_result {
            log!("EFFECT: Updating new row id.\nOld: {old_id}\nNew: {new_id}");
            entry_store_signal
                .read_untracked()
                .iter()
                .find(|e| {
                    e.id.read_untracked() == LocalUuidState::Pending(old_id)
                })
                .map(|e| {
                    e.id.set(LocalUuidState::Resolved(new_id));
                });
        }
    });

    view! {
        <div class="modal-container" class:is_visible=is_visible>
            <div class="modal-content">
                <h1>"Add Player"</h1>
                {move || {
                    if let AddModalState::Open { row_id, side, order } = modal_state
                        .get()
                    {
                        let (resolved_id, pending_id) = match row_id {
                            Some(id) => (Some(id.to_string()), None),
                            None => (None, Some(uuid::Uuid::new_v4().to_string())),
                        };
                        log!("MODAL_CREATE: resolved_id: {:?}", resolved_id);
                        log!("MODAL_CREATE: pending_id: {:?}", pending_id);

                        view! {
                            <ActionForm
                                action=add_player
                                on:submit=move |ev| {
                                    set_modal_state.set(AddModalState::Closed);
                                    let input = AddPlayer::from_event(&ev)
                                        .expect("submission to be well-formed");
                                    let row_id = local_uuid_helper(
                                            input.resolved_id,
                                            input.pending_id,
                                        )
                                        .expect("row id hack to be well-formed");
                                    match row_id {
                                        LocalUuidState::Resolved(_) => {
                                            let entry = entry_store_signal
                                                .read()
                                                .get(order)
                                                .expect("entry to exist")
                                                .to_owned();
                                            let slot = match side {
                                                Side::Left => entry.left,
                                                Side::Right => entry.right,
                                            };
                                            if slot.get().is_none() {
                                                slot.set(Some(input.player))
                                            } else {
                                                leptos::logging::error!(
                                                    "ON_SUBMIT: Attempting to add player to occupied slot!"
                                                )
                                            }
                                        }
                                        LocalUuidState::Pending(_) => {
                                            let (left, right) = match side {
                                                Side::Left => {
                                                    (RwSignal::new(Some(input.player)), RwSignal::new(None))
                                                }
                                                Side::Right => {
                                                    (RwSignal::new(None), RwSignal::new(Some(input.player)))
                                                }
                                            };
                                            let new_entry = LocalQueueEntry {
                                                id: RwSignal::new(row_id),
                                                left,
                                                right,
                                            };
                                            entry_store_signal
                                                .update(|es| {
                                                    es.push(new_entry);
                                                    leptos::logging::log!(
                                                        "ON_SUBMIT: Added player to new row with pending ID"
                                                    );
                                                });
                                        }
                                    }
                                }
                            >
                                <input
                                    type="hidden"
                                    name="queue_id"
                                    value=queue_info.id.to_string()
                                />
                                <input type="hidden" name="side" value=side.to_string() />
                                <input type="hidden" name="resolved_id" value=resolved_id />
                                <input type="hidden" name="pending_id" value=pending_id />
                                <label>
                                    "Player Name" <input type="text" name="player" />
                                </label>
                                <input type="submit" value="Add Player" />
                            </ActionForm>
                        }
                            .into_any()
                    } else {
                        ().into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[server]
pub async fn add_player(
    queue_id: Uuid,
    resolved_id: Option<Uuid>,
    pending_id: Option<Uuid>,
    side: Side,
    player: String,
) -> Result<Option<(Uuid, Uuid)>, ServerFnError> {
    use crate::db::{api::add_player_to_row, api::add_row, DbPool};
    let pool = use_context::<DbPool>().expect("there to be a pool provided.");

    // TODO: get rid of this
    let row_id = local_uuid_helper(resolved_id, pending_id)?;

    match row_id {
        // If we add a new player to an existing row, return none
        LocalUuidState::Resolved(row_id) => {
            log!("SERVER_FN: adding player to row {}", row_id);
            add_player_to_row(row_id, player, side, pool)
                .await
                .inspect_err(|e| error!("Error adding player: {}", e))?;
            Ok(None)
        }
        // If we add a player to a new row, return the old and new IDs for the UI update.
        LocalUuidState::Pending(temp_id) => {
            log!("SERVER_FN: adding player to new row");
            let new_id = add_row(queue_id, player, side, pool)
                .await
                .inspect_err(|e| error!("Error adding player: {}", e))?;
            log!("SERVER_FN: Returning new row ID {}", new_id);
            Ok(Some((temp_id, new_id)))
        }
    }
}

fn local_uuid_helper(
    resolved_id: Option<Uuid>,
    pending_id: Option<Uuid>,
) -> Result<LocalUuidState, ServerFnError> {
    // I was having a really hard time getting a LocalUuidState to pass through
    // the ActionForm input, so this is my compromise (as well as the match in
    // the ActionForm's inputs)
    match (resolved_id, pending_id) {
        (Some(resolved), None) => Ok(LocalUuidState::Resolved(resolved)),
        (None, Some(pending)) => Ok(LocalUuidState::Pending(pending)),
        _ => Err(ServerFnError::ServerError(format!(
            "Invalid UUID arguments. resolved: {:?}, pending: {:?}",
            resolved_id, pending_id
        ))),
    }
}
