//! Blog posts: compile-time (SSG-style) list for /blog.
//! Add entries here or load from /posts directory via include_str! at build time.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogPost {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub excerpt: &'static str,
    /// Raw HTML or markdown content (for SSG, can be include_str!("posts/slug.md"))
    pub content: &'static str,
}

/// Compile-time list of blog posts (maximum performance).
pub fn blog_posts() -> &'static [BlogPost] {
    &[
        BlogPost {
            slug: "first-post",
            title: "First Post",
            date: "2026-02-23",
            excerpt: "Welcome to the blog. This is a placeholder.",
            content: "<p>Welcome to the blog. Replace this with your content or use <code>include_str!(\"posts/first-post.md\")</code>.</p>",
        },
        BlogPost {
            slug: "rust-on-the-edge",
            title: "Rust on the Edge",
            date: "2026-02-20",
            excerpt: "Running Leptos and Axum on Cloudflare Workers.",
            content: "<p>This site is built with Leptos and Axum, deployable to Cloudflare Workers for edge performance.</p>",
        },
    ]
}

pub fn get_post_by_slug(slug: &str) -> Option<&'static BlogPost> {
    blog_posts().iter().find(|p| p.slug == slug)
}
