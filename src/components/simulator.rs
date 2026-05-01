//! Mobile View Simulator — reactive right-hand panel.
//!
//! Displays the editor's content inside a fixed-aspect-ratio (9 : 19.5)
//! container styled to look like a modern smartphone.  Because it reads
//! `AppState::code_content` — a Dioxus [`Signal`] — every change the user
//! makes in the editor is reflected here with zero additional latency: Dioxus
//! simply marks the component as dirty and re-renders only the changed subtree.

#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::AppState;

// ---------------------------------------------------------------------------
// Mobile Simulator component
// ---------------------------------------------------------------------------

/// Reactive Mobile View Simulator panel.
///
/// The phone frame maintains a strict **9 : 19.5** aspect ratio via CSS custom
/// properties (`--phone-w` / `--phone-h`).  The inner "screen" shows a
/// miniaturised, read-only rendering of `AppState::code_content`.
///
/// # Zero-latency sync
/// No message-passing or bridge is involved.  Both this component and
/// [`Editor`] hold a `Signal<String>` from the same [`AppState`] context.
/// When the signal changes, Dioxus schedules a targeted re-render of only the
/// components that subscribed to that signal — giving the same performance
/// characteristics you would get from hand-written pointer sharing in C++.
///
/// # Future LLM integration
/// The panel includes a placeholder section for AI suggestions
/// (`AppState::ai_suggestion`) that a background Tokio task can stream tokens
/// into.  Because everything lives in the same process / memory space, there
/// is no serialisation overhead at all.
#[component]
pub fn MobileSimulator() -> Element {
    let state = use_context::<AppState>();

    let visible = (state.simulator_visible)();
    let panel_class = if visible {
        "simulator-panel"
    } else {
        "simulator-panel hidden"
    };

    // Count how many characters are in the preview (for the badge)
    let char_count = (state.code_content)().len();
    let line_count = (state.code_content)().lines().count();

    rsx! {
        aside { class: "{panel_class}",

            // ── Panel header ─────────────────────────────────────────────
            div { class: "simulator-header",
                span { class: "simulator-header-title", "Mobile Preview" }
                div { class: "simulator-header-actions",
                    // Refresh / sync indicator button
                    button {
                        class: "icon-btn",
                        title: "Sync is live — powered by Dioxus Signals",
                        "⟳"
                    }
                    // Toggle simulator visibility
                    button {
                        class: "icon-btn",
                        title: if visible { "Hide simulator" } else { "Show simulator" },
                        onclick: move |_| state.simulator_visible.set(!visible),
                        { if visible { "⊟" } else { "⊞" } }
                    }
                }
            }

            // ── Phone viewport ───────────────────────────────────────────
            div { class: "simulator-viewport",

                // Phone frame (9 : 19.5 ratio enforced by CSS custom props)
                div { class: "phone-frame",

                    // Dynamic Island / notch
                    div { class: "phone-notch" }

                    // Screen content — mirrors the editor signal
                    div { class: "phone-screen",
                        div { class: "phone-screen-content",
                            // Read the signal — Dioxus tracks this access and
                            // will re-render this component whenever the signal
                            // changes, providing the live preview behaviour.
                            pre {
                                class: "phone-code-preview",
                                dangerous_inner_html: "{render_preview(&(state.code_content)())}",
                            }
                        }
                    }

                    // iOS-style home indicator
                    div { class: "phone-home-bar" }
                }
            }

            // ── Live sync badge + stats ───────────────────────────────────
            div { class: "simulator-info",
                div { class: "simulator-badge",
                    span { class: "badge-dot" }
                    "Live"
                }
                div { class: "simulator-badge",
                    "{line_count} lines"
                }
                div { class: "simulator-badge",
                    "{char_count} chars"
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Preview renderer — lightweight syntax-colour pass
// ---------------------------------------------------------------------------

/// Converts raw source code into minimal HTML with syntax colour hints.
///
/// This is intentionally simple — a production implementation would use
/// `tree-sitter` for full incremental parsing.  Running inside the same
/// process means you can call `tree-sitter` synchronously on every keystroke
/// without any async overhead.
fn render_preview(source: &str) -> String {
    let mut out = String::with_capacity(source.len() * 2);

    for line in source.lines() {
        let escaped = html_escape(line);

        // Simple pattern-based colouring (no regex dependency)
        let coloured = if escaped.trim_start().starts_with("//") {
            // Comment
            format!("<span style='color:#6e7681;font-style:italic'>{escaped}</span>")
        } else if starts_with_keyword(&escaped) {
            // Keyword
            let (kw, rest) = split_first_word(&escaped);
            format!("<span style='color:#ff7b72'>{kw}</span>{rest}")
        } else if escaped.trim_start().starts_with('"') {
            // String literal
            format!("<span style='color:#a5d6ff'>{escaped}</span>")
        } else {
            escaped
        };

        out.push_str(&coloured);
        out.push('\n');
    }

    out
}

/// Escapes `<`, `>`, `&` for safe HTML embedding.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Returns `true` if the (already HTML-escaped) line starts with a Rust keyword.
///
/// Uses word-boundary checking (the character after the keyword must be
/// non-alphanumeric / non-underscore) so that e.g. `"elseif"` is not coloured
/// as the `else` keyword.
fn starts_with_keyword(s: &str) -> bool {
    const KEYWORDS: &[&str] = &[
        "fn", "let", "mut", "pub", "use", "mod", "struct", "enum",
        "impl", "trait", "type", "const", "static", "async", "await",
        "return", "if", "else", "match", "for", "while", "loop",
        "break", "continue", "where", "self", "Self", "super",
    ];
    let trimmed = s.trim_start();
    KEYWORDS.iter().any(|kw| {
        trimmed.starts_with(kw)
            && trimmed[kw.len()..]
                .chars()
                .next()
                .map(|c| !c.is_alphanumeric() && c != '_')
                .unwrap_or(true)
    })
}

/// Splits `"  keyword rest…"` into `("  keyword", " rest…")`.
///
/// Leading whitespace is kept in the first slice so that indented lines
/// still receive correct colouring without losing their indentation.
///
/// The split point is the end of the first identifier token, not the first
/// space, so constructs like `fn(`, `if(`, and `match(` only colour the
/// keyword itself.
fn split_first_word(s: &str) -> (&str, &str) {
    let trimmed = s.trim_start();
    let leading = s.len() - trimmed.len();
    let token_len = trimmed
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .unwrap_or(trimmed.len());

    // Include any leading indent inside the keyword span.
    (&s[..leading + token_len], &s[leading + token_len..])
}
