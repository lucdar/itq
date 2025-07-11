use crate::queue::QueueInfo;
use leptos::prelude::*;

#[component]
pub fn AddQueuePage() -> impl IntoView {
    let add_queue = ServerAction::<AddQueue>::new();
    let value = add_queue.value();
    let pending = add_queue.pending();

    view! {
        <h1>"Add a New Queue"</h1>
        <ActionForm action=add_queue>
            <label>
                "Display Name" <input type="text" name="display_name" />
            </label>
            <label>"URL Name" <input type="text" name="url_name" /></label>
            <input type="submit" value="Add Queue" />
        </ActionForm>
        <Show
            when=move || !pending.get()
            fallback=|| view! { <p>"Adding queue..."</p> }
        >
            {move || {
                value
                    .get()
                    .map(|result| {
                        match result {
                            Ok(queue_info) => {
                                view! {
                                    <p>
                                        "Queue added: "
                                        <a href=format!(
                                            "/queue/{}",
                                            queue_info.url_name,
                                        )>{queue_info.display_name}</a>
                                    </p>
                                }
                                    .into_any()
                            }
                            Err(e) => {
                                view! { <p>"Error adding queue: " {e.to_string()}</p> }
                                    .into_any()
                            }
                        }
                    })
            }}
        </Show>
    }
}

#[server]
pub async fn add_queue(
    display_name: String,
    url_name: String,
) -> Result<QueueInfo, ServerFnError> {
    use crate::db::{api::add_queue, DbPool};
    let pool = use_context::<DbPool>().expect("there to be a `pool` provided.");
    Ok(add_queue(display_name, url_name, pool).await?)
}
