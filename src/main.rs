use axum::Router;
use leptos::config::get_configuration;
use leptos_axum::{generate_route_list, LeptosRoutes};
use my_cv_rust::{shell, Root};

#[tokio::main]
async fn main() {
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(Root);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, Root)
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", addr);
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
