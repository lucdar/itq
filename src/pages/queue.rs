#[cfg(feature = "ssr")]
use crate::db::api;
use crate::queue::QueueData;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn QueuePage() -> impl IntoView {
    let params = use_params_map();
    let queue_name = move || params.read().get("url_name");
    let queue_data: Resource<Option<QueueData>> = Resource::new(queue_name, |name| async {
        match name {
            Some(n) => api::queue_from_url_name(&n).await,
            None => None,
        }
    });

    view! {
        <div class="queue-page">
            <p>"Now Viewing: "{queue_name}</p>
            <Suspense fallback=move || view! {<p>"Loading queue..."</p>}>
                {move || queue_data.get().flatten().map_or(
                        view! {<p>Error! No Queue Found</p>}.into_any(),
                        |qd| view! {
                            <h1>{qd.display_name}</h1>
                            <p>"id "{qd.id.to_string()}</p>
                        }.into_any()
                    )
                }
            </Suspense>
        </div>
    }
}
