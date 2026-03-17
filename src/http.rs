//! Axum server/router construction (SSR).

#[cfg(feature = "ssr")]
use crate::{shell, state::AppState, App};
#[cfg(feature = "ssr")]
use axum::Router;
#[cfg(feature = "ssr")]
use axum::routing::post;
#[cfg(feature = "ssr")]
use leptos::prelude::provide_context;
#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, LeptosRoutes};

/// Build the Axum router for this app.
///
/// Server functions receive `AppState` via Leptos context, while Axum router state remains
/// `LeptosOptions` to satisfy `leptos_axum`'s integration.
#[cfg(feature = "ssr")]
pub fn build_app_router(app_state: AppState) -> Router {
    let routes = generate_route_list(App);

    Router::new()
        // Leptos server functions are POSTed here by the client.
        .route("/api/{*fn_name}", post(leptos_axum::handle_server_fns))
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
        // Provide the required LeptosOptions state value but erase the router's *state type*
        // to `()`, so we can call `into_make_service_with_connect_info`.
        .with_state::<()>(app_state.leptos_options.clone())
}

