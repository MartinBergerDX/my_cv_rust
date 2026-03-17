use axum::Router;
use leptos::config::get_configuration as get_leptos_configuration;
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos::prelude::provide_context;
use my_cv_rust::{configuration, email_client, rate_limit, shell, state::AppState, App};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Load .env so APP_EMAIL_CLIENT__AUTHORIZATION_TOKEN (and other APP_*) are set before config.
    if let Err(e) = dotenvy::dotenv() {
        if e.not_found() {
            tracing::debug!("No .env file found; using environment and config files only");
        } else {
            tracing::warn!("Failed to load .env: {}", e);
        }
    }

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
    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        settings: app_conf,
        email_client: Arc::new(client),
        contact_rate_limiter: Arc::new(rate_limit::default_contact_rate_limiter()),
    };

    let routes = generate_route_list(App);
    let app = Router::new()
        .leptos_routes_with_context(
            &app_state.leptos_options,
            routes,
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            {
                let leptos_options_for_shell = app_state.leptos_options.clone();
                move || shell(leptos_options_for_shell.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        // Provide LeptosOptions as Axum router state value.
        // (Leptos server functions get AppState via context, not Axum State.)
        .with_state(app_state.leptos_options.clone());

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let url = format!("http://{addr}");
    tracing::info!("listening on {}", url);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
    tracing::info!("Server stopped");
}
