use crate::queue::QueueInfo;
use leptos::prelude::*;

#[component]
pub fn QueueHeader(queue_data: ReadSignal<QueueInfo>) -> impl IntoView {
    view! {
        <div class="queue-header">
            <h1>{ move || queue_data.get().display_name }</h1>
            <p>"id: "{ move || queue_data.get().id.to_string() }</p>
            // TODO: add # of players/rows to queue info
            // <p>"players: "{ move || queue_data }</p>
        </div>
    }
}
