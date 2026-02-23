//! Responsive project grid: image, title, tech tags. Lazy-loaded images in responsive containers.

use crate::data::ProjectEntry;
use leptos::prelude::{ElementChild, *};

#[component]
pub fn ProjectGrid(projects: &'static [ProjectEntry]) -> impl IntoView {
    view! {
        <section id="projects" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            {projects
                .iter()
                .map(|p| view! { <ProjectCard entry=p /> })
                .collect_view()}
        </section>
    }
}

#[component]
fn ProjectCard(entry: &'static ProjectEntry) -> impl IntoView {
    let content = view! {
        <article class="rounded-lg border border-gray-200 overflow-hidden bg-white shadow-sm hover:shadow-md transition-shadow">
            <div class="img-container">
                <img
                    src=entry.image_path
                    alt=entry.title
                    loading="lazy"
                    class="w-full h-full object-cover"
                />
            </div>
            <div class="p-4">
                <h3 class="font-semibold text-gray-900">{entry.title}</h3>
                {entry.description.map(|d| view! { <p class="text-sm text-gray-600 mt-1">{d}</p> })}
                <div class="flex flex-wrap gap-2 mt-2">
                    {entry
                        .tech_tags
                        .iter()
                        .map(|tag| {
                            view! {
                                <span
                                    class="text-xs px-2 py-0.5 rounded bg-gray-100 text-gray-700"
                                >
                                    {*tag}
                                </span>
                            }
                        })
                        .collect_view()}
                </div>
                {entry.link.map(|href| {
                    view! {
                        <a
                            href=href
                            target="_blank"
                            rel="noopener noreferrer"
                            class="inline-block mt-2 text-sm text-blue-600 hover:underline"
                        >
                            "View project →"
                        </a>
                    }
                })}
            </div>
        </article>
    };

    view! { {content} }
}
