//! Responsive navbar: horizontal on desktop, bottom-docked "Thumb-Zone" on mobile.

use leptos::prelude::{ElementChild, *};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <>
            /* Desktop: horizontal nav at top */
            <nav class="hidden md:flex fixed top-0 left-0 right-0 z-50 bg-white/90 backdrop-blur border-b border-gray-200">
                <div class="max-w-6xl mx-auto w-full px-4 h-14 flex items-center justify-between">
                    <a href="/" class="font-semibold text-gray-900 hover:text-gray-600">"Home"</a>
                    <ul class="flex gap-6">
                        <li><a href="/#work" class="text-gray-600 hover:text-gray-900">"Work"</a></li>
                        <li><a href="/#projects" class="text-gray-600 hover:text-gray-900">"Projects"</a></li>
                        // <li><a href="/blog" class="text-gray-600 hover:text-gray-900">"Blog"</a></li>
                        <li><a href="/#contact" class="text-gray-600 hover:text-gray-900">"Contact"</a></li>
                    </ul>
                </div>
            </nav>

            /* Mobile: bottom-docked Thumb-Zone menu */
            <nav
                class="md:hidden fixed bottom-0 left-0 right-0 z-50 bg-white/95 backdrop-blur border-t border-gray-200 pb-[env(safe-area-inset-bottom)]"
                aria-label="Main navigation"
            >
                <ul class="flex justify-around items-center h-16 px-2">
                    <li>
                        <a href="/" class="flex flex-col items-center gap-0.5 text-gray-600 hover:text-gray-900 aria-[current=page]:text-gray-900">
                            <span class="text-lg">"🏠"</span>
                            <span class="text-xs">"Home"</span>
                        </a>
                    </li>
                    <li>
                        <a href="/#work" class="flex flex-col items-center gap-0.5 text-gray-600 hover:text-gray-900">
                            <span class="text-lg">"💼"</span>
                            <span class="text-xs">"Work"</span>
                        </a>
                    </li>
                    <li>
                        <a href="/#projects" class="flex flex-col items-center gap-0.5 text-gray-600 hover:text-gray-900">
                            <span class="text-lg">"📁"</span>
                            <span class="text-xs">"Projects"</span>
                        </a>
                    </li>
                    /* <li>
                        <a href="/blog" class="flex flex-col items-center gap-0.5 text-gray-600 hover:text-gray-900">
                            <span class="text-lg">"📝"</span>
                            <span class="text-xs">"Blog"</span>
                        </a>
                    </li> */
                    <li>
                        <a href="/#contact" class="flex flex-col items-center gap-0.5 text-gray-600 hover:text-gray-900">
                            <span class="text-lg">"✉️"</span>
                            <span class="text-xs">"Contact"</span>
                        </a>
                    </li>
                </ul>
            </nav>
        </>
    }
}
