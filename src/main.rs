//! VibeCode — The Next-Generation AI Code Editor
//!
//! Built with Rust & Dioxus 0.6, featuring:
//! - Frameless dark-mode desktop window (Zed-like aesthetic)
//! - Three-column layout: Sidebar | Editor | Mobile Simulator
//! - Dioxus Signals API for zero-latency state synchronization
//! - Modular architecture ready for local LLM streaming integration

#![allow(non_snake_case)]

use dioxus::desktop::tao::dpi::LogicalSize;
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

mod components;

use components::{Editor, MobileSimulator, Sidebar};

// ---------------------------------------------------------------------------
// Shared application state — all fields are Dioxus Signals so any component
// that reads them will automatically re-render when they change.
// ---------------------------------------------------------------------------

/// Top-level application state shared across all components via Dioxus context.
///
/// Because `Signal<T>` is `Copy`, this struct is `Copy` as well, making it
/// trivial to pass into closures without cloning.
#[derive(Clone, Copy)]
pub struct AppState {
    /// Live source code being edited.  The Mobile Simulator reads this signal
    /// to render its preview with zero-latency synchronisation.
    pub code_content: Signal<String>,

    /// Name of the currently active file shown in the editor tab.
    pub active_file: Signal<String>,

    /// When `true` the sidebar collapses to icon-only mode.
    pub sidebar_collapsed: Signal<bool>,

    /// When `true` the Mobile View Simulator panel is visible.
    pub simulator_visible: Signal<bool>,

    /// Cursor position reported by the editor (line, column).
    pub cursor_position: Signal<(usize, usize)>,

    // -----------------------------------------------------------------------
    // Extension points for future local LLM streaming integration.
    // These signals can be set by a background Tokio task that streams tokens
    // from a locally-running model (e.g. llama.cpp / Ollama) into the same
    // shared memory space — no WebView bridge overhead.
    // -----------------------------------------------------------------------

    /// AI-generated suggestion streamed from the local LLM.
    pub ai_suggestion: Signal<String>,

    /// Whether the LLM is currently generating a response.
    pub ai_loading: Signal<bool>,
}

// ---------------------------------------------------------------------------
// Application entry-point
// ---------------------------------------------------------------------------

fn main() {
    // ------------------------------------------------------------------
    // Window configuration — frameless, sized for a modern laptop display
    // ------------------------------------------------------------------
    let window = WindowBuilder::new()
        .with_title("VibeCode")
        // Remove native title bar for a Zed-like frameless aesthetic
        .with_decorations(false)
        // Default to a comfortable widescreen size
        .with_inner_size(LogicalSize::new(1440.0_f64, 900.0_f64))
        // Prevent the window from becoming too small to use
        .with_min_inner_size(LogicalSize::new(960.0_f64, 640.0_f64));

    // ------------------------------------------------------------------
    // Embed critical styles directly in the WebView <head>.
    // For production, compile assets/tailwind.css with the Tailwind CLI:
    //   npx tailwindcss -i ./input.css -o ./assets/tailwind.css --minify
    // then replace with_custom_head with a <link> tag pointing to that file.
    // ------------------------------------------------------------------
    let head_html = include_str!("assets/head.html").to_string();

    let config = Config::default()
        .with_window(window)
        .with_custom_head(head_html);

    LaunchBuilder::desktop().with_cfg(config).launch(App);
}

// ---------------------------------------------------------------------------
// Root component
// ---------------------------------------------------------------------------

#[component]
fn App() -> Element {
    // Initialise all shared signals and register them as context so every
    // descendant component can access them with `use_context::<AppState>()`.
    use_context_provider(|| AppState {
        code_content: Signal::new(STARTER_CODE.to_string()),
        active_file: Signal::new("main.rs".to_string()),
        sidebar_collapsed: Signal::new(false),
        simulator_visible: Signal::new(true),
        cursor_position: Signal::new((1, 1)),
        ai_suggestion: Signal::new(String::new()),
        ai_loading: Signal::new(false),
    });

    rsx! {
        div {
            class: "app-shell",

            // ── Custom drag region for the frameless window ──────────────
            // Only this narrow strip can be used to drag the window; all
            // interactive children opt out via `app-region: no-drag`.
            div {
                class: "titlebar",
                // Traffic-light placeholder (macOS) — real implementation
                // would use dioxus-desktop window control APIs.
                div {
                    class: "titlebar-dots",
                    span { class: "dot dot-close" }
                    span { class: "dot dot-minimize" }
                    span { class: "dot dot-maximize" }
                }
                span { class: "titlebar-label", "VibeCode" }
            }

            // ── Three-column layout ──────────────────────────────────────
            div {
                class: "columns",
                Sidebar {}
                Editor {}
                MobileSimulator {}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Starter code shown when the editor first opens
// ---------------------------------------------------------------------------

const STARTER_CODE: &str = r#"//! VibeCode — AI-Native Rust Editor
//!
//! Start coding below.  The Mobile Preview on the right
//! mirrors your changes in real-time via Dioxus Signals.

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0_i32);

    rsx! {
        div {
            h1 { "Counter: {count}" }
            button {
                onclick: move |_| count += 1,
                "Increment"
            }
        }
    }
}
"#;
