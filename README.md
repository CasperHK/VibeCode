# VibeCode

**The Next-Generation AI Code Editor, Built with Rust & Dioxus.**

A cutting-edge, AI-native code editor featuring a sleek dark-mode frameless window, a three-column layout, and a reactive Mobile View Simulator — all powered by Dioxus 0.6 Signals for zero-latency state synchronisation.

---

## Architecture

```
vibecode/
├── src/
│   ├── main.rs                  # App shell, AppState, window config
│   ├── assets/
│   │   └── head.html            # Embedded CSS (compiled at build time)
│   └── components/
│       ├── mod.rs               # Re-exports
│       ├── sidebar.rs           # Collapsible navigation sidebar
│       ├── editor.rs            # Code editing pane with line numbers
│       └── simulator.rs         # 9:19.5 Mobile View Simulator
├── assets/
│   └── tailwind.css             # Tailwind CLI output (web target)
├── Cargo.toml
├── Dioxus.toml
├── tailwind.config.js
└── input.css                    # Tailwind CSS source
```

### Three-Column Layout

| Column | Component | Description |
|--------|-----------|-------------|
| Left   | `Sidebar` | Icon-based navigation; collapses to 48 px |
| Centre | `Editor`  | `<textarea>` + line-number gutter; writes to shared signal |
| Right  | `MobileSimulator` | 9:19.5 phone frame; reads the same signal for live preview |

### State Management (Dioxus Signals)

All shared state lives in `AppState`, a `Copy` struct of `Signal<T>` fields provided via `use_context_provider` in the root component:

```rust
#[derive(Clone, Copy)]
pub struct AppState {
    pub code_content:      Signal<String>,   // editor ↔ simulator
    pub active_file:       Signal<String>,
    pub sidebar_collapsed: Signal<bool>,
    pub simulator_visible: Signal<bool>,
    pub cursor_position:   Signal<(usize, usize)>,
    // LLM streaming hooks
    pub ai_suggestion:     Signal<String>,
    pub ai_loading:        Signal<bool>,
}
```

Child components call `use_context::<AppState>()` to read and write signals — no message passing, no WebView bridge, no serialisation overhead.

---

## Getting Started

### Prerequisites

- **Rust** 1.70+ — <https://rustup.rs>
- **GTK 3 / WebKit2GTK** (Linux) — see below
- **Node.js + npm** (optional, for Tailwind CLI)

#### Linux system libraries

```bash
sudo apt-get install -y \
  pkg-config libglib2.0-dev libgtk-3-dev \
  libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev \
  libxdo-dev libappindicator3-dev librsvg2-dev
```

### Run (desktop)

```bash
cargo run
```

### Tailwind CSS (optional)

The desktop build embeds its styles via `src/assets/head.html` (compiled into the binary with `include_str!`), so Tailwind is **not** required to run the app. To regenerate `assets/tailwind.css` for the web target:

```bash
npm install -D tailwindcss
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

---

## Roadmap

- [ ] Replace `<textarea>` with a WebComponent code editor (CodeMirror / Monaco)
- [ ] Tree-sitter incremental parsing for full syntax highlighting
- [ ] Local LLM streaming via `llm` / Ollama crate into `AppState::ai_suggestion`
- [ ] File tree panel in the sidebar (via `notify` file watcher)
- [ ] Multi-tab editor support
- [ ] Dioxus web target (progressive web app)

---

## License

MIT — see [LICENSE](LICENSE).
