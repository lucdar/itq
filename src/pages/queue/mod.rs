mod delete_button;
mod header;
mod rows;

use crate::queue::QueueInfo;
use delete_button::DeleteButton;
use header::QueueHeader;
use leptos::context::provide_context;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use rows::Rows;

#[component]
pub fn QueuePage() -> impl IntoView {
    let params = use_params_map();
    let url_queue_name = move || {
        params
            .read()
            .get("url_name")
            .expect("there to be a `url_name` guaranteed by the router")
    };
    let queue_info = Resource::new(url_queue_name, |name| async {
        get_queue(name).await.ok()
    });

    view! {
        <div class="queue-page">
            <p>"Now Viewing: "{url_queue_name}</p>
            <Suspense fallback=move || {
                view! { <p>"Loading queue..."</p> }
            }>
                {move || {
                    queue_info
                        .get()
                        .flatten()
                        .map_or(
                            view! { <h1>"Error: No Queue Found"</h1> }.into_any(),
                            move |queue_info| {
                                provide_context(queue_info);
                                // Provide context for deeply nested components
                                view! {
                                    <QueueHeader />
                                    <Rows />
                                    <DeleteButton />
                                }
                                    .into_any()
                            },
                        )
                }}
            </Suspense>
        </div>
    }
}

#[server]
/// Gets a queue from the database from the queue's unique url_name
async fn get_queue(url_name: String) -> Result<QueueInfo, ServerFnError> {
    use crate::db::{api::get_queue_info, DbPool};
    let pool = use_context::<DbPool>().expect("there to be a `pool` provided.");
    Ok(get_queue_info(url_name, pool).await?)
}
