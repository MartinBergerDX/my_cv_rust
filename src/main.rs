use axum::Router;
use leptos::config::get_configuration as get_leptos_configuration;
use leptos_axum::{generate_route_list, LeptosRoutes};
use my_cv_rust::{configuration, email_client, shell, Root};
use tracing_subscriber::EnvFilter;

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

    let routes = generate_route_list(Root);
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, Root)
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let url = format!("http://{addr}");
    tracing::info!("listening on {}", url);
    axum::serve(listener, app.into_make_service()).await.unwrap();
    tracing::info!("Server stopped");
}
