# My CV — Leptos + Axum

High-performance personal CV and portfolio site built with **Leptos 0.8** (SSR), **Axum 0.8**, and **Tailwind CSS**, deployable to **Cloudflare Workers**.

## Tech stack

- **Frontend:** Leptos 0.8 (isomorphic/SSR)
- **Server:** Axum 0.8
- **Styling:** Tailwind CSS (mobile-first)
- **Deploy:** Cloudflare Workers (wrangler, compatibility date 2026-02-23)

## Run locally

1. **Build client assets** (WASM, JS, CSS) so the app can hydrate and the contact form works:

   ```bash
   cargo leptos build
   cp target/tmp/tailwind.css target/site/pkg/my_cv_rust.css
   cp target/site/pkg/my_cv_rust.wasm target/site/pkg/my_cv_rust_bg.wasm
   ```

   cargo-leptos runs Tailwind and writes to `target/tmp/tailwind.css`; copy it to `target/site/pkg/my_cv_rust.css` so the app serves it. Copy the WASM so `/pkg/my_cv_rust_bg.wasm` is available. **Release build and run:** `./build-release.sh`.

   (Optional) To pin a Tailwind version, set `LEPTOS_TAILWIND_VERSION` before `cargo leptos build`.

2. **Run the server:**

   ```bash
   cargo run
   ```

   App is served at `http://127.0.0.1:3000`. Configuration is loaded from `configuration/base.yaml` and `configuration/local.yaml` (default). For production config:

   ```bash
   APP_ENVIRONMENT=production cargo run
   ```

   **If the “Send message” button does nothing:** ensure you ran step 1 (including the CSS and WASM copies). Missing CSS can break layout and the button.

## Project layout

- `src/lib.rs` — App, router, shell
- `src/pages/` — Home, blog list, blog post
- `src/components/` — Navbar (desktop horizontal / mobile bottom “thumb zone”), ProjectGrid, ContactForm
- `src/data/` — `ResumeData`, `project_entries()`, `blog_posts()` (SSG-style)
- `src/server/` — `submit_contact` server function (mock; ready for email API)
- `style/main.css` — Tailwind v4 entry + custom styles
- `posts/` — Future blog content (compile-time / SSG)
- `public/` — Static assets (e.g. placeholder.svg for project grid)
- `wrangler.toml` — Cloudflare Workers config (compatibility_date 2026-02-23)

## Content

- **CV data:** Edit `src/data/resume_data.rs` and `resume_data()` to match your CV (work history, skills, education).
- **Projects:** Edit `src/data/projects.rs` and `project_entries()` (image paths, titles, tech tags).
- **Blog:** Edit `src/data/blog_posts.rs` and add files under `posts/` for SSG; use `include_str!` in Rust for compile-time content.

## Deploy to Cloudflare Workers

Configure your Worker entrypoint and build (e.g. wasm target with `leptos_axum` “wasm” feature), then:

```bash
npx wrangler deploy
```

## Guidelines

- Fine-grained reactivity and `view!` macros throughout.
- Images use `loading="lazy"` and live in responsive containers.
- Idiomatic Rust with minimal clones and strong typing.
