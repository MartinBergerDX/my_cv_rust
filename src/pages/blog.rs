use crate::data::{get_post_by_slug, blog_posts, BlogPost};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
fn BlogPostContent(post: Option<&'static BlogPost>) -> impl IntoView {
    view! {
        <article>
            {match post {
                Some(p) => view! {
                    <>
                        <h1 class="text-3xl font-bold text-gray-900">{p.title}</h1>
                        <p class="text-gray-500 mt-2">{p.date}</p>
                        <div class="prose prose-gray mt-6" inner_html=p.content />
                    </>
                }
                .into_view(),
                None => view! {
                    <>
                        <h1 class="text-3xl font-bold text-gray-900">""</h1>
                        <p class="text-gray-500 mt-2">""</p>
                        <div class="prose prose-gray mt-6" inner_html="<p class=\"text-gray-600\">Post not found.</p>" />
                    </>
                }
                .into_view(),
            }}
        </article>
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
                        view! {
                            <li>
                                <a
                                    href=format!("/blog/{}", p.slug)
                                    class="block p-4 rounded-lg border border-gray-200 hover:border-gray-300 hover:bg-gray-50 transition"
                                >
                                    <h2 class="font-semibold text-gray-900">{p.title}</h2>
                                    <p class="text-sm text-gray-500 mt-1">{p.date}</p>
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
    let slug = move || params.get().get("id").unwrap_or_default();
    let post = move || get_post_by_slug(&slug());

    view! {
        <div class="max-w-2xl mx-auto px-4 py-12 md:py-16">
            {move || view! { <BlogPostContent post=post() /> }}
        </div>
    }
}
