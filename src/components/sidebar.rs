//! Sidebar — narrow navigation panel on the left.
//!
//! Displays icon-based navigation items and collapses to icon-only mode.
//! Active section is highlighted; clicking any item sets it as active.

#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::AppState;

// ---------------------------------------------------------------------------
// Navigation item definition
// ---------------------------------------------------------------------------

struct NavItem {
    id: &'static str,
    icon: &'static str,
    label: &'static str,
}

const NAV_ITEMS: &[NavItem] = &[
    NavItem { id: "explorer",    icon: "⎇",  label: "Explorer"      },
    NavItem { id: "search",      icon: "⌕",  label: "Search"        },
    NavItem { id: "git",         icon: "⑂",  label: "Source Control" },
    NavItem { id: "extensions",  icon: "⊞",  label: "Extensions"    },
    NavItem { id: "ai",          icon: "✦",  label: "AI Assistant"  },
];

const BOTTOM_ITEMS: &[NavItem] = &[
    NavItem { id: "settings", icon: "⚙", label: "Settings" },
    NavItem { id: "account",  icon: "◉", label: "Account"  },
];

// ---------------------------------------------------------------------------
// Sidebar component
// ---------------------------------------------------------------------------

/// Sidebar navigation panel.
///
/// Reads `sidebar_collapsed` from [`AppState`] and renders either icon-only
/// or full-label mode.  Clicking the collapse toggle or any navigation item
/// updates the shared signal, which causes both the sidebar and any other
/// subscriber to re-render automatically.
#[component]
pub fn Sidebar() -> Element {
    let state = use_context::<AppState>();
    let active_section = use_signal(|| "explorer");

    let collapsed = (state.sidebar_collapsed)();
    let sidebar_class = if collapsed { "sidebar collapsed" } else { "sidebar expanded" };

    rsx! {
        aside {
            class: "{sidebar_class}",

            // ── Top: collapse toggle ─────────────────────────────────────
            div { class: "sidebar-icons",
                // Collapse / expand button
                button {
                    class: "sidebar-item",
                    title: if collapsed { "Expand sidebar" } else { "Collapse sidebar" },
                    onclick: move |_| {
                        let current = (state.sidebar_collapsed)();
                        state.sidebar_collapsed.set(!current);
                    },
                    span { class: "icon", { if collapsed { "»" } else { "«" } } }
                    if !collapsed {
                        span { class: "label", "Collapse" }
                    }
                }

                div { class: "sidebar-divider" }

                // ── Primary navigation items ─────────────────────────────
                for item in NAV_ITEMS.iter() {
                    {
                        let is_active = active_section() == item.id;
                        let item_class = if is_active {
                            "sidebar-item active"
                        } else {
                            "sidebar-item"
                        };
                        let item_id = item.id;

                        rsx! {
                            button {
                                key: "{item.id}",
                                class: "{item_class}",
                                title: "{item.label}",
                                onclick: move |_| active_section.set(item_id),
                                span { class: "icon", "{item.icon}" }
                                if !collapsed {
                                    span { class: "label", "{item.label}" }
                                }
                            }
                        }
                    }
                }
            }

            // ── Bottom: settings / account ───────────────────────────────
            div { class: "sidebar-bottom",
                div { class: "sidebar-divider" }
                div { class: "sidebar-icons",
                    for item in BOTTOM_ITEMS.iter() {
                        {
                            let item_id = item.id;
                            let is_active = active_section() == item_id;
                            let item_class = if is_active {
                                "sidebar-item active"
                            } else {
                                "sidebar-item"
                            };

                            rsx! {
                                button {
                                    key: "{item.id}",
                                    class: "{item_class}",
                                    title: "{item.label}",
                                    onclick: move |_| active_section.set(item_id),
                                    span { class: "icon", "{item.icon}" }
                                    if !collapsed {
                                        span { class: "label", "{item.label}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
