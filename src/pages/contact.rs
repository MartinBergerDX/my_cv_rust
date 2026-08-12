use crate::components::ContactForm;
use leptos::prelude::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div class="max-w-2xl mx-auto px-4 py-12 md:py-16">
            <ContactForm />
        </div>
    }
}
