//! App shell and root component (pattern from [realworld-leptos](https://github.com/Bechma/realworld-leptos)).

use leptos::config::LeptosOptions;
use leptos::hydration::HydrationScripts;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::Navbar;
use crate::pages::{BlogListPage, BlogPostPage, HomePage};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <HydrationScripts options=options.clone() />
                <link rel="shortcut icon" type="image/svg+xml" href="/favicon.svg"/>
                <MetaTags/>
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
        <Stylesheet id="leptos" href="/pkg/my_cv_rust.css"/>
        <Title text="Martin Berger"/>
        <Router>
            <div class="page">
                <Navbar/>
                <main class="pt-14 pb-24 md:pb-12 max-w-6xl mx-auto px-4">
                    <Routes fallback=|| view! { <p class="text-gray-600">"Not found."</p> }>
                        <Route path=path!("/") view=HomePage />
                        <Route path=path!("/blog") view=BlogListPage />
                        <Route path=path!("/blog/:id") view=BlogPostPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
