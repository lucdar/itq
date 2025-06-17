use leptos::prelude::*;
use leptos::Params;
use leptos_router::hooks::use_params_map;
use leptos_router::params::Params;

#[derive(Params, PartialEq)]
struct QueueParams {
    url_name: Option<String>,
}

#[component]
pub fn QueuePage() -> impl IntoView {
    let params = use_params_map();
    let queue_data = Resource::new(
        move || {
            let url_name = params.read().get("url_name");
        }
    )

    view! {
        <div class="queue-page">
            <p>"now viewing: "{queue_name}</p>
            <Suspense fallback=move || view! {<p>"Loading queue..."</p>}>
            {move || {

            }}
        </div>
    }
}
