#[cfg(feature = "ssr")]
use crate::db::{api, DbPool};
use crate::queue::QueueInfo;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use uuid::Uuid;

#[component]
pub fn DeleteButton() -> impl IntoView {
    let queue_info = use_context::<QueueInfo>().expect("there to be a `queue_info` provided.");
    let delete_queue = ServerAction::<DeleteQueue>::new();
    let pending = delete_queue.pending();
    let value = delete_queue.value();
    let navigate = use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(())) = value.get() {
            navigate("/", Default::default());
        }
    });

    view! {
        <ActionForm action=delete_queue>
            <input type="hidden" name="id" value={move || queue_info.id.to_string()}/>
            <button type="submit">"Delete Queue"</button>
        </ActionForm>
        <Show
            when=move || !pending.get()
            fallback=|| view! { <p>"Deleting queue..."</p> }
        >
            { match value.get() {
                Some(Ok(())) => view! { <p>"Queue deleted"</p> }.into_any(),
                Some(Err(e)) => view! { <p>"Error deleting queue: " {e.to_string()}</p> }.into_any(),
                None => ().into_any()
            }}
        </Show>
    }
}

#[server]
pub async fn delete_queue(id: Uuid) -> Result<(), ServerFnError> {
    let pool = use_context::<DbPool>().expect("there to be a `pool` provided.");
    Ok(api::delete_queue(id, pool).await?)
}
