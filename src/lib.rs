#![allow(non_snake_case)]

mod components;
mod data;
mod pages;
mod server;

use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use pages::{BlogListPage, BlogPostPage, HomePage};

/// Document shell for SSR: HTML structure. Used by Axum fallback handler.
/// Stylesheet is in the shell so we don't need leptos_meta injection (avoids </head> panic).
pub fn shell(_options: LeptosOptions) -> impl IntoView {
    view! {
        <html>
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" href="/pkg/my_cv_rust.css"/>
            </head>
            <body></body>
        </html>
    }
}

/// Full document root used by leptos_routes so every SSR response includes head + stylesheet.
#[component]
pub fn Root() -> impl IntoView {
    view! {
        <html>
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" href="/pkg/my_cv_rust.css"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Router>
            <Routes fallback=|| view! { <p class="text-gray-600">"Not found."</p> }>
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/blog") view=BlogListPage />
                <Route path=path!("/blog/:id") view=BlogPostPage />
            </Routes>
        </Router>
    }
}
