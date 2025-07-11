use crate::pages::queue::rows::{AddModalState, EntryStore};
use crate::queue::{QueueInfo, Side};
use leptos::logging::{error, log};
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn AddPlayerModal(
    state: ReadSignal<AddModalState>,
    set_state: WriteSignal<AddModalState>,
) -> impl IntoView {
    let queue_info = use_context::<QueueInfo>()
        .expect("there to be a `queue_info` provided.");
    let entry_store = use_context::<EntryStore>()
        .expect("there to be a `entry_store` provided.");

    let is_visible = move || state.get() != AddModalState::Closed;

    let add_player = ServerAction::<AddPlayer>::new();
    // Could use these for UI stuff
    let _pending = add_player.pending();
    let _value = add_player.value();

    view! {
        <div class="modal-container" class:is_visible=is_visible>
            <div class="modal-content">
                <h1>"Add Player"</h1>
                {move || {
                    match state.get() {
                        AddModalState::Open { row_id, side, order } => {
                            let row_id = row_id.map(|id| id.to_string());
                            view! {
                                <ActionForm action=add_player>
                                    <input
                                        type="hidden"
                                        name="queue_id"
                                        value=queue_info.id.to_string()
                                    />
                                    <input type="hidden" name="row_id" value=row_id />
                                    <input type="hidden" name="side" value=side.to_string() />
                                    <label>
                                        "Player Name" <input type="text" name="player" />
                                    </label>
                                    <input
                                        type="submit"
                                        value="Add Player"
                                        on:submit=move |e| { log!(
                                            // TODO: Update the UI here...
                                            // or somewhere else because this doesn't work...
                                            // aubwuh
                                            "event: {}", e.to_string())
                                        }
                                    />
                                </ActionForm>
                            }
                                .into_any()
                        }
                        AddModalState::Closed => ().into_any(),
                    }
                }}
            </div>
        </div>
    }
}

#[server]
pub async fn add_player(
    queue_id: Uuid,
    row_id: Option<Uuid>,
    side: Side,
    player: String,
) -> Result<(), ServerFnError> {
    use crate::db::{api::add_player, DbPool};
    let pool = use_context::<DbPool>().expect("there to be a `pool` provided.");
    Ok(add_player(queue_id, row_id, player, side, pool)
        .await
        .inspect_err(|e| error!("Error adding player: {}", e))?)
}
