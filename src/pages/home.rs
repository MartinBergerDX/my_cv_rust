use crate::data::resume_data;
use leptos::prelude::{ElementChild, *};

#[component]
pub fn HomePage() -> impl IntoView {
    let resume = resume_data();

    view! {
            <section class="py-12 md:py-16">
                <h1 class="text-3xl md:text-4xl font-bold text-gray-900">{resume.name.clone()}</h1>
                <p class="text-xl text-gray-600 mt-1">{resume.title.clone()}</p>
                <p class="text-gray-500 mt-1">{resume.tagline.clone()}</p>
                <div class="mt-4 text-sm text-gray-600">
                    <p>"Date of birth: "{resume.date_of_birth.clone()}</p>
                    <p>"Languages: "{resume.languages.iter().map(|l| format!("{} ({})", l.language, l.level)).collect::<Vec<_>>().join(", ")}</p>
                </div>
            </section>

            <section class="py-6">
                <h2 class="text-xl font-semibold text-gray-900 mb-2">"About"</h2>
                <p class="text-gray-600">{resume.summary.clone()}</p>
            </section>

            <section id="work" class="py-8">
                <h2 class="text-2xl font-bold text-gray-900 mb-4">"Work"</h2>
                <ul class="space-y-6">
                    {resume.work_history.iter().map(|w| view! {
                        <li class="border-l-2 border-gray-200 pl-4">
                            <h3 class="font-semibold text-gray-900">{w.role.clone()}</h3>
                            <p class="text-gray-600">
                                {w.company.clone()}
                                <span class="text-gray-400">
                                    {(!w.location.is_empty()).then(|| [" · ", w.location.as_str()].concat()).unwrap_or_default()}
                                </span>
                            </p>
                            <p class="text-sm text-gray-500">{w.start.clone()}" – "{w.end.clone()}</p>
                            <p class="mt-1 text-gray-600">{w.description.clone()}</p>
                            <ul class="mt-2 list-disc list-inside text-sm text-gray-600">
                                {w.highlights.iter().map(|h| view! { <li>{h.clone()}</li> }).collect_view()}
                            </ul>
                            {(!w.tech.is_empty()).then(|| view! {
                                <p class="mt-2 text-sm text-gray-500">
                                    <span class="font-medium text-gray-700">"Tech: "</span>
                                    {w.tech.clone()}
                                </p>
                            })}
                        </li>
                    }).collect_view()}
                </ul>
            </section>

            <section class="py-8">
                <h2 class="text-2xl font-bold text-gray-900 mb-4">"Skills"</h2>
                <div class="flex flex-wrap gap-2">
                    {resume.skills.technical.iter().map(|s| view! {
                        <span class="px-3 py-1 rounded-full bg-gray-100 text-gray-700 text-sm">{s.clone()}</span>
                    }).collect_view()}
                    {resume.skills.tools.iter().map(|s| view! {
                        <span class="px-3 py-1 rounded-full bg-gray-200 text-gray-700 text-sm">{s.clone()}</span>
                    }).collect_view()}
                </div>
            </section>

            <section class="py-8">
                <h2 class="text-2xl font-bold text-gray-900 mb-4">"Education"</h2>
                <ul class="space-y-3">
                    {resume.education.iter().map(|e| view! {
                        <li>
                            <span class="font-semibold text-gray-900">{e.degree.clone()}</span>
                            <span class="text-gray-600">" at "{e.institution.clone()}</span>
                            <span class="text-gray-500 text-sm">" (" {e.year.clone()} ")"</span>
                        </li>
                    }).collect_view()}
                </ul>
            </section>
    }
}
