//! Project grid data: image paths, titles, and tech tags.
//! Add your project screenshots and metadata here.

#[derive(Clone, Debug)]
pub struct ProjectEntry {
    /// Path to the project screenshot (under /pkg or public assets).
    pub image_path: &'static str,
    /// Project title.
    pub title: &'static str,
    /// Short description (optional).
    pub description: Option<&'static str>,
    /// Link to live demo or repo.
    pub link: Option<&'static str>,
    /// Tech tags (e.g. Rust, Leptos, Tailwind). Use vec! for easy construction.
    pub tech_tags: Vec<&'static str>,
}

use std::sync::OnceLock;

static PROJECTS: OnceLock<Vec<ProjectEntry>> = OnceLock::new();

/// Compile-time list of projects for the grid.
pub fn project_entries() -> &'static [ProjectEntry] {
    PROJECTS.get_or_init(|| {
        vec![
            ProjectEntry {
                image_path: "/placeholder.svg",
                title: "Project One",
                description: Some("Short description of the project."),
                link: Some("https://github.com/you/repo"),
                tech_tags: vec!["Rust", "Leptos", "Tailwind"],
            },
            ProjectEntry {
                image_path: "/placeholder.svg",
                title: "Project Two",
                description: Some("Another project description."),
                link: None,
                tech_tags: vec!["TypeScript", "React"],
            },
            ProjectEntry {
                image_path: "/placeholder.svg",
                title: "Project Three",
                description: None,
                link: Some("#"),
                tech_tags: vec!["Rust", "Axum"],
            },
        ]
    })
}
