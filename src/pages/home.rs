use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    // let queue_names = OnceResource::new(load_queues());

    view! {
        <h1>"Welcome to itq!"</h1>
        // <Suspense fallback=LoadingQueues>
        //     // <For
        //     //   each=move || queue_names.get().unwrap_or(Ok(vec![]))
        //     //   key=|name| name.clone()
        //     //   let(name)
        //     // >
        //     //     <p>{name}</p>
        //     // </For>
        // </Suspense>
    }
}
