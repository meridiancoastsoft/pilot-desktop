# Pilot Suite Desktop

Native desktop application for Pilot Suite MSP platform. Built with [Tauri 2](https://v2.tauri.app/).

## Prerequisites

### All Platforms
- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (latest stable)
- Tauri CLI: `cargo install tauri-cli --version "^2"`

### Windows
- [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (pre-installed on Windows 10/11)

### macOS
- Xcode Command Line Tools: `xcode-select --install`
- For universal binary: both x86_64 and aarch64 Rust targets

## Setup

```bash
# Clone
git clone https://github.com/meridiancoastsoft/pilot-desktop.git
cd pilot-desktop

# Install JS dependencies
npm install

# Install Rust dependencies (automatic on first build)
```

## Development

```bash
# Run in dev mode (points to dev.meridiancoastsoft.com)
npm run dev
```

## Build

```bash
# Build for current platform
npm run build

# Platform-specific
npm run build:windows    # Windows .exe + .msi
npm run build:mac        # macOS universal .dmg
npm run build:mac-intel  # macOS Intel only
npm run build:mac-arm    # macOS Apple Silicon only
```

Build outputs:
- Windows: `src-tauri/target/release/bundle/msi/Pilot Suite_3.0.0_x64_en-US.msi`
- macOS: `src-tauri/target/release/bundle/dmg/Pilot Suite_3.0.0_universal.dmg`

## Architecture

```
┌─────────────────────────────────────┐
│  Tauri Shell (Rust)                 │
│  - Window management (multi-window) │
│  - System tray                      │
│  - Clipboard bridge                 │
│  - Auto-updater                     │
│  - Credential store                 │
├─────────────────────────────────────┤
│  WebView                            │
│  - Pilot Suite web app (remote)     │
│  - RDP viewer (WebSocket/canvas)    │
│  - JS ↔ Rust IPC bridge            │
└─────────────────────────────────────┘
```

The app loads the hosted Pilot Suite web app in a native WebView. Rust handles native features (tray, windows, clipboard, files) and exposes them to JS via `window.__PILOT_DESKTOP__`.

## JS Bridge API

The web app can detect the desktop app and use native features:

```javascript
if (window.__PILOT_DESKTOP__) {
    // Pop out RDP to native window
    window.__PILOT_DESKTOP__.openRdpWindow(agentId, hostname, sessionUrl);

    // Clipboard
    await window.__PILOT_DESKTOP__.writeClipboard('text');
    const text = await window.__PILOT_DESKTOP__.readClipboard();

    // Navigation
    window.__PILOT_DESKTOP__.navigate('/rmm/agents');
}
```

## Roadmap

- [x] v3.0.0 — Basic shell, tray, auto-redirect to server
- [ ] v3.1.0 — Multi-window RDP pop-outs
- [ ] v3.1.5 — Vault credential integration
- [ ] v3.2.0 — Clipboard bridge (local ↔ remote)
- [ ] v3.3.0 — File drag-and-drop into RDP sessions
- [ ] v3.4.0 — Notifications, keyboard passthrough, branding
