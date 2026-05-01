# 🛸 VibeCode
The Next-Generation AI Code Editor, Built with Rust & Dioxus.
VibeCode 是一款專為 AI 協作時代設計的跨平台程式碼編輯器。不同於傳統編輯器，它利用 Dioxus 的原生效能與 Signals 反應機制，將 AI 生成、代碼編輯與行動端預覽（Mobile View）無縫整合在一個流暢的 UI 體驗中。


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

## 🚀 快速上手

### 前置要求
確保你已安裝 [Rust 1.75+](https://rustup.rs/) 與 dx 命令行工具：
```bash
cargo install dioxus-cli
```

### 安裝與運行

   1. 克隆專案：
   ```
   git clone https://github.com
   cd vibecode
   ```
   
   2. 啟動桌面版開發模式：
   ```bash   
   dx serve --platform desktop
   ```
   
## 📂 專案架構
```text
src/
├── ai/             # AI 邏輯與 LLM 串接
├── components/     # Dioxus UI 組件 (Editor, Sidebar, etc.)
├── mobile_view/    # Mobile 模擬容器與渲染邏輯
├── state/          # 全域狀態管理 (Signals)
└── main.rs         # 應用程式入口
```

## 🗺 路線圖 (Roadmap)

* Phase 1: 整合高效能文字編輯元件。
* Phase 2: 實作 AI 流式對話視窗與 Context 注入機制。
* Phase 3: 完善 Mobile View 縮放與熱重載同步。
* Phase 4: 支援外掛插件系統。
