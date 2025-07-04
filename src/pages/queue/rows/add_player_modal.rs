use crate::queue::{QueueInfo, Side};
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn AddPlayerModal(order: usize, side: Side) -> impl IntoView {
    let queue_info = use_context::<QueueInfo>()
        .expect("there to be a `queue_info` provided.");
    let add_player = ServerAction::<AddPlayer>::new();
    let pending = add_player.pending();
    let value = add_player.value();

    view! {
        <div class="modal-container">
            <div class="modal-content">
                <h1>"Add Player"</h1>
                <ActionForm action=add_player>
                    <input
                        type="hidden"
                        name="queue_id"
                        value=queue_info.id.to_string()
                    />
                    <input type="hidden" name="order" value=order />
                    <input type="hidden" name="side" value=side.to_string() />
                    <label>
                        "Player Name" <input type="text" name="player" />
                    </label>
                    <input type="submit" value="Add Player" />
                </ActionForm>
            </div>
        </div>
    }
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
