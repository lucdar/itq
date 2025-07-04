#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use dotenvy;
    use itq::app::*;
    use itq::db;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    // Load config and environment variables
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    dotenvy::dotenv().expect(".env should be loadable");

    // Create database pool
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&database_url).await;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let pool = pool.clone();
                move || provide_context(pool.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
