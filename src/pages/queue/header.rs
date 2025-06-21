use crate::queue::QueueData;
use leptos::prelude::*;

#[component]
pub fn QueueHeader(queue_data: ReadSignal<QueueData>) -> impl IntoView {
    view! {
        <div class="queue-header">
            <h1>{ move || queue_data.get().display_name }</h1>
            <p>"id: "{ move || queue_data.get().id.to_string() }</p>
            <p>"players: "{ move || queue_data.get().rows.len() }</p>
        </div>
    }
}
