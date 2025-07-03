use crate::queue::QueueInfo;
use leptos::prelude::*;

#[component]
pub fn QueueHeader() -> impl IntoView {
    let queue_info = use_context::<QueueInfo>().expect("there to be a `queue_info` provided.");
    view! {
        <div class="queue-header">
            <h1>{ queue_info.display_name }</h1>
            <p>"id: "{ queue_info.id.to_string() }</p>
            // TODO: add # of players/rows to queue info
            // <p>"players: "{ move || queue_data }</p>
        </div>
    }
}
