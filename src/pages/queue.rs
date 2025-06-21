use crate::queue::QueueData;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn QueuePage() -> impl IntoView {
    let params = use_params_map();
    let url_queue_name = move || params.read().get("url_name");
    let queue_data: Resource<Option<QueueData>> = Resource::new(url_queue_name, |name| async {
        match name {
            Some(name) => get_queue(name).await.ok(),
            None => None,
        }
    });

    view! {
        <div class="queue-page">
            <p>"Now Viewing: "{url_queue_name}</p>
            <Suspense fallback=move || view! {<p>"Loading queue..."</p>}>
                {move || queue_data.get().flatten().map_or(
                    view! {<p>Error! No Queue Found</p>}.into_any(),
                    move |qd| view! {
                        <h1>{qd.display_name}</h1>
                        <p>"id "{qd.id.to_string()}</p>
                        <For
                            each={move || queue_data.get().unwrap().unwrap().rows}
                            key={ |r| r.id }
                            children={ move |r| view! {
                                <p>{r.id.to_string()}</p>
                            }}
                        />
                    }.into_any()
                )}
            </Suspense>
        </div>
    }
}

#[server]
/// Gets a queue from the database from the queue's unique url_name
pub async fn get_queue(url_name: String) -> Result<QueueData, ServerFnError> {
    // TODO: As it stands right now, I think this function makes two round-trips
    // to the database (see [`crate::db::api::get_queue`]). It could be made
    // more efficient by keeping track of the queue ID and getting rows as its
    // own server function.
    use crate::db::{api::get_queue, DbPool};
    let pool = use_context::<DbPool>().expect("there to be a `pool` provided.");
    Ok(get_queue(url_name, pool).await?)
}
