//! Editor — central high-performance code editing pane.
//!
//! Uses a `<textarea>` for editing and a line-number gutter computed from the
//! current content.  All state is stored in [`AppState`] signals so the
//! Mobile Simulator automatically reflects every keystroke with zero latency.

#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::AppState;

// ---------------------------------------------------------------------------
// Editor component
// ---------------------------------------------------------------------------

/// Central code editor pane.
///
/// Writes changes to `AppState::code_content` which is a shared [`Signal`].
/// Any other component that reads the same signal (e.g. [`MobileSimulator`])
/// will re-render automatically — no message passing, no WebView bridge.
///
/// Future integrations:
/// - Swap the `<textarea>` for a CodeMirror / Monaco WebComponent.
/// - Spawn a Tokio task that feeds `AppState::ai_suggestion` from a local LLM
///   (e.g. llama.cpp via `llm` crate) without leaving Rust's memory space.
#[component]
pub fn Editor() -> Element {
    let mut state = use_context::<AppState>();

    // Derive line count from the signal — Dioxus will re-compute only when
    // `code_content` changes, keeping rendering cost minimal.
    let line_count = (state.code_content)().lines().count().max(1);
    let (cursor_line, cursor_col) = (state.cursor_position)();

    rsx! {
        section { class: "editor-pane",

            // ── Tab bar ──────────────────────────────────────────────────
            div { class: "editor-tabs",
                div { class: "editor-tab active",
                    span { "󰌠 " }
                    span { "{(state.active_file)()}" }
                    span {
                        class: "tab-close",
                        title: "Close tab",
                        "×"
                    }
                }
            }

            // ── Editor body: gutter + textarea ───────────────────────────
            div { class: "editor-body",

                // Line-number gutter (derived from signal, zero extra state)
                div { class: "editor-gutter",
                    for n in 1..=line_count {
                        span { key: "{n}", "{n}" }
                    }
                }

                // Code textarea — writes directly to the shared signal.
                // The Mobile Simulator reads the same signal, so its preview
                // updates on every `oninput` event without any extra plumbing.
                textarea {
                    class: "editor-textarea",
                    spellcheck: "false",
                    autocomplete: "off",
                    autocapitalize: "off",
                    wrap: "off",
                    placeholder: "// Start coding…",
                    value: "{(state.code_content)()}",

                    // Update shared signal on every keystroke
                    oninput: move |e| {
                        state.code_content.set(e.value());
                    },

                    // Track cursor position for the status bar
                    onkeyup: move |_e| {
                        // In a full implementation, use JS interop to read
                        // selectionStart from the textarea for exact position.
                        // For now we keep the last known value.
                    },
                }
            }

            // ── Status bar ───────────────────────────────────────────────
            div { class: "editor-statusbar",
                // Left segment
                span { class: "statusbar-item", "⎇  main" }
                span { class: "statusbar-item", "Rust" }

                // Right segment
                div { class: "statusbar-right",
                    span { class: "statusbar-item", "Ln {cursor_line}, Col {cursor_col}" }
                    span { class: "statusbar-item", "UTF-8" }
                    span { class: "statusbar-item", "LF" }
                    span { class: "statusbar-item", "Spaces: 4" }
                }
            }
        }
    }
}
