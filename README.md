# 🛸 VibeCode

**The Next-Generation AI Code Editor, Built with Rust & Dioxus.**

VibeCode 是一款專為 AI 協作時代設計的跨平台程式碼編輯器。不同於傳統編輯器，它利用 Dioxus 的原生效能與 Signals 反應機制，將 AI 生成、代碼編輯與行動端預覽（Mobile View）無縫整合在一個流暢的 UI 體驗中。

A cutting-edge, AI-native code editor featuring a sleek dark-mode frameless window, a three-column layout, and a reactive Mobile View Simulator — all powered by Dioxus 0.6 Signals for zero-latency state synchronisation.

---

## ✨ 特色功能

* 🚀 極致效能 (Dioxus Powered)：超越 Tauri 的資源佔用與通訊速度。透過 Rust 原生渲染與高效的 V-DOM，實現零延遲的 AI 流式輸出。
* 🤖 AI-Native Workflow：深度整合 LLM，支持即時語義補全、自動化重構與對話式代碼修正。
* 📱 內建 Mobile View：無需額外模擬器！編輯器內建真正的行動端渲染容器，支持觸控模擬與響應式開發。
* 🦀 100% Type-Safe Rust：從後端邏輯到 UI 介面全採用 Rust 編寫，享受最強大的編譯期檢查。
* ⚡ Vibe-Driven UI：極簡、沉浸式的介面設計，專為追求「心流」的開發者打造。

## 🛠 技術棧

* Core: [Rust](https://www.rust-lang.org/)
* Frontend Framework: [Dioxus](https://dioxuslabs.com/) (Signals, Full-stack reactivity)
* Styling: Tailwind CSS
* AI Backend: LLM Connector (Supports Ollama, OpenAI, Anthropic)
* Rendering: Wry / WGPU (Native Graphics)

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

## 🚀 Getting Started

### Prerequisites

確保你已安裝 [Rust 1.75+](https://rustup.rs/) 與 dx 命令行工具：

```bash
cargo install dioxus-cli
```

**Linux** — additional system libraries required:

```bash
sudo apt-get install -y \
  pkg-config libglib2.0-dev libgtk-3-dev \
  libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev \
  libxdo-dev libappindicator3-dev librsvg2-dev
```

### Run

```bash
# Desktop (native window)
cargo run
# or
dx serve --platform desktop

# Web (requires Tailwind output — see below)
dx serve --platform web
```

### Tailwind CSS (optional)

The desktop build embeds its styles via `src/assets/head.html` (compiled into the binary with `include_str!`), so Tailwind is **not** required to run the app. To regenerate `assets/tailwind.css` for the web target:

```bash
npm install -D tailwindcss
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

---

## 🗺 Roadmap

* Phase 1: 整合高效能文字編輯元件。
* Phase 2: 實作 AI 流式對話視窗與 Context 注入機制。
* Phase 3: 完善 Mobile View 縮放與熱重載同步。
* Phase 4: 支援外掛插件系統。

- [ ] Replace `<textarea>` with a WebComponent code editor (CodeMirror / Monaco)
- [ ] Tree-sitter incremental parsing for full syntax highlighting
- [ ] Local LLM streaming via `llm` / Ollama crate into `AppState::ai_suggestion`
- [ ] File tree panel in the sidebar (via `notify` file watcher)
- [ ] Multi-tab editor support
- [ ] Dioxus web target (progressive web app)

---

## License

MIT — see [LICENSE](LICENSE).
