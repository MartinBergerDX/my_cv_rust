use axum::{extract::State, http::header::{CACHE_CONTROL, CONTENT_TYPE}, response::IntoResponse, routing::get, Router};
use leptos::config::{get_configuration as get_leptos_configuration, LeptosOptions};
use leptos_axum::{generate_route_list, LeptosRoutes};
use my_cv_rust::{configuration, email_client, shell, App};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

/// Serves favicon.ico from site root if present; otherwise 204 No Content so the console stays clean.
async fn favicon_ico(
    State(options): State<LeptosOptions>,
) -> impl IntoResponse {
    let path = PathBuf::from(options.site_root.as_ref()).join("favicon.ico");
    match tokio::fs::read(&path).await {
        Ok(bytes) => (
            [
                (CONTENT_TYPE, "image/x-icon"),
                (CACHE_CONTROL, "public, max-age=86400"),
            ],
            bytes,
        )
            .into_response(),
        Err(_) => axum::http::StatusCode::NO_CONTENT.into_response(),
    }
}

/// Serves the main WASM module as `my_cv_rust_bg.wasm` when the build outputs `my_cv_rust.wasm`.
async fn serve_bg_wasm(
    State(options): State<LeptosOptions>,
) -> impl IntoResponse {
    let path = PathBuf::from(options.site_root.as_ref())
        .join(options.site_pkg_dir.as_ref())
        .join("my_cv_rust.wasm");
    match tokio::fs::read(&path).await {
        Ok(bytes) => (
            [
                (CONTENT_TYPE, "application/wasm"),
                (CACHE_CONTROL, "no-store"),
            ],
            bytes,
        )
            .into_response(),
        Err(_) => axum::http::StatusCode::NOT_FOUND.into_response(),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    tracing::info!("Starting my_cv_rust");

    let leptos_conf = get_leptos_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = leptos_conf.leptos_options;
    let addr = leptos_options.site_addr;

    let app_conf = configuration::get_configuration().expect("Failed to load application configuration.");
    let client = email_client::EmailClient::new(
        app_conf.email_client.base_url.clone(),
        app_conf.email_client.sender_email.clone(),
        app_conf.email_client.authorization_token.clone(),
        app_conf.email_client.timeout(),
    );
    email_client::init_email_client(client);

    let routes = generate_route_list(App);
    // Router must have state type LeptosOptions *before* leptos_routes_with_context so the
    // integration’s provide_context::<S>(state) provides LeptosOptions to Root.
    let leptos_options_for_shell = leptos_options.clone();
    let app = Router::new()
        .leptos_routes(
            &leptos_options,
            routes,
            move || shell(leptos_options_for_shell.clone()),
        )
        .route("/favicon.ico", get(favicon_ico))
        .route("/pkg/my_cv_rust_bg.wasm", get(serve_bg_wasm))
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let url = format!("http://{addr}");
    tracing::info!("listening on {}", url);
    axum::serve(listener, app).await.unwrap();
    tracing::info!("Server stopped");
}
