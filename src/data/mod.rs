mod blog_posts;
mod projects;
mod resume_data;

pub use blog_posts::{blog_posts, get_post_by_slug, BlogPost};
pub use projects::{project_entries, ProjectEntry};
pub use resume_data::resume_data;
