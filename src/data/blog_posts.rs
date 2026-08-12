//! Blog posts: compile-time (SSG-style) list for /blog.
//! Markdown bodies live under /posts and are included at build time.
//! Posts are ordered by date (newest first). URLs: /blog/{date}/{slug}.

use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogPost {
    pub slug: &'static str,
    pub title: &'static str,
    /// ISO date `YYYY-MM-DD` — used for sorting and URLs.
    pub date: &'static str,
    pub excerpt: &'static str,
    /// Markdown source (typically via `include_str!`).
    pub content: &'static str,
}

impl BlogPost {
    pub fn path(&self) -> String {
        format!("/blog/{}/{}", self.date, self.slug)
    }

    pub fn html_content(&self) -> String {
        markdown_to_html(self.content)
    }
}

fn markdown_to_html(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(md, options);
    let mut html_out = String::new();
    html::push_html(&mut html_out, parser);
    html_out
}

/// Compile-time list of blog posts, newest date first.
pub fn blog_posts() -> &'static [BlogPost] {
    &[BlogPost {
        slug: "the-silent-death-of-a-riverpod-provider",
        title: "The Silent Death of a Riverpod Provider: Why `ref.read` Gave Me an Object — and Then Killed Its Ref",
        date: "2026-08-12",
        excerpt: "Riverpod 3 + autoDispose taught me the hard way: `ref.read` gives you the value, but `ref.watch` keeps the provider alive. A disposed Ref story from a real upload pipeline.",
        content: include_str!(
            "../../posts/2026-08-12-the-silent-death-of-a-riverpod-provider.md"
        ),
    }]
}

pub fn get_post(date: &str, slug: &str) -> Option<&'static BlogPost> {
    blog_posts()
        .iter()
        .find(|p| p.date == date && p.slug == slug)
}
