//! Application state for the Axum server + Leptos server functions (SSR).

#[cfg(feature = "ssr")]
use crate::{configuration::Settings, email_client::EmailClient, rate_limit::IpRateLimiter};
#[cfg(feature = "ssr")]
use leptos::config::LeptosOptions;
#[cfg(feature = "ssr")]
use std::sync::Arc;

/// Shared server-side application state.
#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub settings: Settings,
    pub email_client: Arc<EmailClient>,
    pub contact_rate_limiter: Arc<IpRateLimiter>,
}

