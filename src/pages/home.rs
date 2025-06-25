use leptos::prelude::*;

use crate::queue::QueueInfo;

#[component]
pub fn HomePage() -> impl IntoView {
    let queue_infos = OnceResource::new(load_queues());

    view! {
        <h1>Welcome to itq!</h1>
        <Suspense fallback= move || view!{<p>Loading queues...</p>}>
            { move || queue_infos.get().map(|info| {
                match info {
                    Ok(q) => view! {
                        <For
                          each={move || q.clone().into_iter()}
                          key={|q| q.id}
                          children={move |q| view!{<QueueDisplay queue_info=q />}}
                        />
                    }.into_any(),
                    Err(e) => view! {
                        <p>"Error Loading Queues: "{e.to_string()}</p>
                    }.into_any()
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn QueueDisplay(queue_info: QueueInfo) -> impl IntoView {
    let href = format!("./queue/{}", queue_info.url_name);

    view! {
        <div class="homepage-info">
            <a href={href}>{queue_info.display_name}</a>
        </div>
    }
}

#[server]
pub async fn load_queues() -> Result<Vec<QueueInfo>, ServerFnError> {
    use crate::db::{api::get_all_queues, DbPool};
    let pool = use_context::<DbPool>().expect("there to be a `pool` provided.");
    Ok(get_all_queues(pool).await?)
}
