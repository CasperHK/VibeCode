/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
  ],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        // GitHub-inspired dark palette (Zed-like aesthetic)
        canvas: {
          DEFAULT: "#0d1117",
          subtle:  "#161b22",
          inset:   "#010409",
        },
        border: {
          DEFAULT: "#30363d",
          muted:   "#21262d",
        },
        fg: {
          DEFAULT: "#e6edf3",
          muted:   "#8b949e",
          subtle:  "#6e7681",
        },
        accent: {
          blue:    "#388bfd",
          green:   "#3fb950",
          purple:  "#bc8cff",
          orange:  "#e3b341",
          red:     "#f85149",
        },
      },
      fontFamily: {
        mono: [
          "JetBrains Mono",
          "Fira Code",
          "Cascadia Code",
          "ui-monospace",
          "SFMono-Regular",
          "monospace",
        ],
      },
      boxShadow: {
        "phone": "0 0 0 1px #2c2c2e, 0 24px 80px rgba(0,0,0,0.85), inset 0 1px 0 rgba(255,255,255,0.05)",
        "glow":  "0 0 20px rgba(56,139,253,0.15)",
      },
    },
  },
  plugins: [],
};
