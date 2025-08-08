<p align="center">
<img src="./src-tauri/icons/icon.png" width="200" height="200" />
</p>

# TypeCast — Keystroke visualizer for screen recording

TypeCast is a lightweight, cross‑platform desktop overlay that shows your keystrokes while you record or present. Built with Tauri (Rust + React), it stays always‑on‑top, is click‑through, and appears on all workspaces without getting in your way.

<div align="center">
  <img src="assets/typecast.png" alt="TypeCast screenshot" width="720" />
  <br/>
  <em>Minimal overlay showing recent keys with modifier caps</em>
</div>

---

## Features

- System tray controls: Start Monitoring, Stop Monitoring, Quit
- Always on top and visible on all workspaces/virtual desktops
- Transparent, frameless, full‑screen click‑through window (doesn’t block clicks)
- Displays recent key presses (with modifiers like Ctrl, Alt, Shift, ⌘)
- Clean keycaps UI; shows combos (e.g., Ctrl + C) and arrows/glyphs (↑ ↓ ← →, ⏎, ⌫, etc.)
- Auto‑hide after inactivity (default 5s) and fades in on next key press
- Privacy‑friendly: no disk logging, no network access; events are ephemeral in memory
- Cross‑platform by design (Windows/macOS/Linux)

> App icon: `src-tauri/icons/icon.png`

---

## How it works

- Global keystroke listener (Rust): `src-tauri/src/setup/keystoke.rs`
  - Uses the `rdev` crate to capture key press/release events and maintain modifier state
  - Emits events to the frontend via Tauri window events (`"key-logger"`)
- Toggle monitoring (Rust): `src-tauri/src/commands/monitoring.rs`
  - `start_monitoring` / `stop_monitoring` flip an `AtomicBool` used by the listener
- System tray (Rust): `src-tauri/src/setup/tray.rs`
  - Tray menu items emit `"start_monitoring"` / `"stop_monitoring"` to the webview
- Overlay UI (React): `src/App.tsx`
  - Listens for `"key-logger"` events and renders the last 10 key events as keycaps
  - Auto‑hides after 5s of inactivity; re‑shows on the next key press

Window behavior (from `src-tauri/tauri.conf.json`):

- `alwaysOnTop: true`, `visibleOnAllWorkspaces: true`, `transparent: true`, `decorations: false`
- `fullscreen: true`, `skipTaskbar: true`, click‑through is enabled in setup via `set_ignore_cursor_events(true)`

---

## Prerequisites

- Rust (stable) + Cargo
- Node.js 18+ and pnpm
- Tauri OS dependencies (Linux/macOS/Windows)
  - Linux (Debian/Ubuntu) commonly needs: `libwebkit2gtk-4.1-dev`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `build-essential`
  - See official docs for your distro: https://v2.tauri.app/start/prerequisites/

Note (Linux/Wayland): global key capture may require X11 (Xorg or XWayland). Pure Wayland sessions are limited.

---

## Develop

```bash
pnpm install
pnpm tauri dev
```

This runs the React dev server (Vite) and launches the Tauri shell.

---

## Build

```bash
pnpm tauri build
```

Artifacts will be produced under `src-tauri/target/release` (bundles under `bundle/`).

---

## Usage

- Launch the app; a transparent, click‑through overlay is created
- Use the tray icon menu:
  - Start Monitoring — begin emitting keystrokes to the overlay
  - Stop Monitoring — pause the listener and clear modifier state
  - Quit App — exit cleanly
- Type normally; recent keys appear at the bottom‑right and fade out after 5s

macOS: You may be prompted to grant Accessibility permissions for global key capture.
Windows: Some environments may require elevated permissions.

---

## Configuration & tweaks

- Overlay window options: `src-tauri/tauri.conf.json`
  - Always‑on‑top, transparency, visibility on all workspaces, etc.
- Click‑through behavior: enabled in `src-tauri/src/setup/mod.rs` via `set_ignore_cursor_events(true)`
- Auto‑hide timeout: `src/App.tsx` — `setTimeout(() => setVisible(false), 5000)`
- Key glyph mapping: `src-tauri/src/setup/keystoke.rs` (`get_key_string`) — adjust labels/icons
- Styles: `src/App.css`

---

## Known limitations

- Wayland capture support is limited; X11/XWayland works best today
- Some keyboards/IMEs may report keys as `Unknown` and won’t render a label
- No persistence: the tool is display‑only by design (no export history yet)

---

## Tech stack

- Tauri 2 (`@tauri-apps/cli`, `@tauri-apps/api`)
- Rust + rdev (global input capture)
- React 18, Vite 6, Tailwind CSS

---

## Project layout

- Frontend: `src/` (entry `src/main.tsx`, UI `src/App.tsx`)
- Tauri app: `src-tauri/` (entry `src-tauri/src/main.rs`, config `tauri.conf.json`)
- Tray & listener setup: `src-tauri/src/setup/`
- Commands (invokes): `src-tauri/src/commands/`
- Assets: `assets/` (screenshots, demo)

<div align="center">
  <img src="assets/typecast2.png" alt="TypeCast screenshot 2" width="720" />
</div>

---

## Credits

- Built with Tauri, React, and the rdev crate — thanks to their maintainers and communities.

If you find this useful, consider starring the repo and opening issues/PRs for ideas and improvements.
