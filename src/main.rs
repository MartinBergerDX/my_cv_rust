use leptos::config::get_configuration as get_leptos_configuration;
use my_cv_rust::{configuration, email_client, http, rate_limit, state::AppState};
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
    let app = http::build_app_router(app_state);

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
