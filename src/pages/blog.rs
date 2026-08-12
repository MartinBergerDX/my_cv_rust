use crate::data::{blog_posts, get_post, BlogPost};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
fn BlogPostContent(post: Option<&'static BlogPost>) -> impl IntoView {
    match post {
        Some(p) => {
            let html = p.html_content();
            view! {
                <article>
                    <h1 class="text-3xl font-bold text-gray-900 leading-tight">{p.title}</h1>
                    <time class="block text-gray-500 mt-2" datetime=p.date>{p.date}</time>
                    <div class="blog-prose mt-8" inner_html=html />
                </article>
            }
            .into_any()
        }
        None => view! {
            <article>
                <h1 class="text-3xl font-bold text-gray-900">"Post not found"</h1>
                <p class="text-gray-600 mt-4">
                    <a href="/blog" class="underline hover:text-gray-900">"← Back to blog"</a>
                </p>
            </article>
        }
        .into_any(),
    }
}

#[component]
pub fn BlogListPage() -> impl IntoView {
    let posts = blog_posts();

    view! {
        <div class="max-w-2xl mx-auto px-4 py-12 md:py-16">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">"Blog"</h1>
            <ul class="space-y-6">
                {posts
                    .iter()
                    .map(|p| {
                        let href = p.path();
                        view! {
                            <li>
                                <a
                                    href=href
                                    class="block p-4 rounded-lg border border-gray-200 hover:border-gray-300 hover:bg-gray-50 transition"
                                >
                                    <h2 class="font-semibold text-gray-900">{p.title}</h2>
                                    <time class="block text-sm text-gray-500 mt-1" datetime=p.date>{p.date}</time>
                                    <p class="text-gray-600 mt-2">{p.excerpt}</p>
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let post = move || {
        let map = params.get();
        let date = map.get("date").unwrap_or_default();
        let slug = map.get("slug").unwrap_or_default();
        get_post(&date, &slug)
    };

    view! {
        <div class="max-w-2xl mx-auto px-4 py-12 md:py-16">
            <p class="mb-8">
                <a href="/blog" class="text-sm text-gray-500 hover:text-gray-900">"← Blog"</a>
            </p>
            {move || view! { <BlogPostContent post=post() /> }}
        </div>
    }
}
