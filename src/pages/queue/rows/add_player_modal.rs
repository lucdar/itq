use crate::queue::Side;
use leptos::prelude::*;
use uuid::Uuid;

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
