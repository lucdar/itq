use crate::queue::QueueData;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn QueuePage() -> impl IntoView {
    let params = use_params_map();
    let queue_name = move || params.read().get("url_name");

    let queue_data: Resource<Option<QueueData>> = Resource::new(queue_name, fetch_queue_data);

    view! {
        <div class="queue-page">
            <p>"Now Viewing: "{queue_name}</p>
            <Suspense fallback=move || view! {<p>"Loading queue..."</p>}>
                {move || match queue_data.get().flatten() {
                    None => view! {<p>Error! No Queue Found</p>},
                    Some(qd) => view! {
                        <h1>{qd.display_name}</h1>
                        <p>"id "{qd.id}</p>
                    }
                }}
            </Suspense>
        </div>
    }
}
