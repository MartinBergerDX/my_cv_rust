#![allow(non_snake_case)]
#![recursion_limit = "256"]

pub mod app;
mod components;
mod data;
mod pages;
mod server;

#[cfg(not(target_arch = "wasm32"))]
pub mod configuration;
#[cfg(not(target_arch = "wasm32"))]
pub mod email_client;

pub use app::{shell, App};

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
