//! Contact form with server-side submission. Ready for email API integration.

use crate::server::{submit_contact, ContactFormInput, ContactFormResult};
use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn ContactForm() -> impl IntoView {
    let submit = Action::new(|input: &ContactFormInput| submit_contact(input.clone()));
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (subject, set_subject) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let pending = submit.pending();
    let result = submit.value();

    let on_submit = move |_| {
        submit.dispatch(ContactFormInput {
            name: name.get(),
            email: email.get(),
            subject: Some(subject.get()),
            message: message.get(),
        });
    };

    view! {
        <section id="contact" class="max-w-xl mx-auto">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">"Get in touch"</h2>
            <form on:submit=move |ev: ev::SubmitEvent| { ev.prevent_default(); on_submit(()); } class="space-y-4">
                <div>
                    <label for="name" class="block text-sm font-medium text-gray-700">"Name"</label>
                    <input
                        type="text"
                        id="name"
                        name="name"
                        required
                        prop:value=name
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                </div>
                <div>
                    <label for="email" class="block text-sm font-medium text-gray-700">"Email"</label>
                    <input
                        type="email"
                        id="email"
                        name="email"
                        required
                        prop:value=email
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                </div>
                <div>
                    <label for="subject" class="block text-sm font-medium text-gray-700">"Subject"</label>
                    <input
                        type="text"
                        id="subject"
                        name="subject"
                        prop:value=subject
                        on:input=move |ev| set_subject.set(event_target_value(&ev))
                        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                </div>
                <div>
                    <label for="message" class="block text-sm font-medium text-gray-700">"Message"</label>
                    <textarea
                        id="message"
                        name="message"
                        rows=4
                        required
                        prop:value=message
                        on:input=move |ev| set_message.set(event_target_value(&ev))
                        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    ></textarea>
                </div>
                <button
                    type="submit"
                    disabled=move || pending.get()
                    class="w-full rounded-md bg-gray-900 px-4 py-2 text-white font-medium hover:bg-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    {move || if pending.get() { "Sending…" } else { "Send message" }}
                </button>
            </form>
            <div class="mt-4">
                {move || result.get().map(|r| match r {
                    Ok(ContactFormResult { success, message }) => view! {
                        <p class=if success { "text-green-600" } else { "text-red-600" }>
                            {message}
                        </p>
                    }.into_view(),
                    Err(e) => view! { <p class="text-red-600">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </div>
        </section>
    }
}
