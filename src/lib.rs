#![allow(non_snake_case)]
#![recursion_limit = "256"]

pub mod app;
mod components;
mod data;
mod pages;
mod server;

#[cfg(feature = "ssr")]
pub mod configuration;
#[cfg(feature = "ssr")]
pub mod email_client;
#[cfg(feature = "ssr")]
pub mod http;
#[cfg(feature = "ssr")]
pub mod rate_limit;
#[cfg(feature = "ssr")]
pub mod state;

pub use app::{shell, App};

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
