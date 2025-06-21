use crate::queue::{PlayerData, QueueData, RowData, RowPlayerState};
use leptos::prelude::*;

#[component]
pub fn QueueRows(queue_data: ReadSignal<QueueData>) -> impl IntoView {
    view! {
        <For
            each={ move || { queue_data.get().rows.into_iter().enumerate() } }
            key={ |(_, row)| row.id }
            children={ move |(idx, row_data)| view!{ <Row row_data idx/> } }
        />
    }
}

#[component]
pub fn Row(row_data: RowData, idx: usize) -> impl IntoView {
    let (left, right) = match row_data.player_state {
        RowPlayerState::LeftOnly(left) => (Some(left), None),
        RowPlayerState::RightOnly(right) => (None, Some(right)),
        RowPlayerState::Both(left, right) => (Some(left), Some(right)),
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

// TODO: Maybe when this becomes more complicated we could implement IntoView on the Player struct?
#[component]
pub fn PlayerToken(player_data: ReadSignal<Option<PlayerData>>) -> impl IntoView {
    match player_data.get() {
        None => view! {
            <div class="player-token empty">
                <p>"Empty"</p>
            </div>
        }
        .into_any(),
        Some(pd) => view! {
            <div class="player-token">
                <p>{pd.name}</p>
            </div>
        }
        .into_any(),
    }
}
