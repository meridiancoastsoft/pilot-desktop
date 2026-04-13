/**
 * Pilot Suite Desktop — Frontend Bridge
 * 
 * This script runs on the initial loading screen and injects native
 * capabilities into the Pilot Suite web app once loaded.
 */

const { invoke } = window.__TAURI__.core;

// ─── Startup: Connect to server ─────────────────────────────────────

async function init() {
    const status = document.getElementById('status');

    try {
        // Get server URL from Rust backend
        const serverUrl = await invoke('get_server_url');
        const appVersion = await invoke('get_app_version');

        status.textContent = `Connecting to ${serverUrl}...`;

        // Check server connectivity
        const ok = await checkServer(serverUrl);
        if (ok) {
            status.textContent = 'Loading Pilot Suite...';
            // The main window navigates via Rust setup, this is just the fallback
            window.location.href = serverUrl;
        } else {
            status.textContent = 'Cannot reach server. Retrying...';
            retryConnection(serverUrl);
        }
    } catch (err) {
        status.textContent = 'Starting...';
        // Tauri API not ready yet — the Rust side handles navigation
    }
}

async function checkServer(url) {
    try {
        const resp = await fetch(url + '/health', {
            method: 'GET',
            mode: 'no-cors',
            signal: AbortSignal.timeout(5000),
        });
        return true;
    } catch {
        return false;
    }
}

async function retryConnection(url) {
    const status = document.getElementById('status');
    let attempts = 0;

    const interval = setInterval(async () => {
        attempts++;
        status.textContent = `Retrying... (attempt ${attempts})`;

        const ok = await checkServer(url);
        if (ok) {
            clearInterval(interval);
            status.textContent = 'Connected! Loading...';
            window.location.href = url;
        }

        if (attempts >= 30) {
            clearInterval(interval);
            status.textContent = 'Could not connect to server. Please check your network.';
        }
    }, 3000);
}

// ─── Native Feature Bridge ──────────────────────────────────────────
// These functions are injected into the Pilot Suite web app via
// window.__PILOT_DESKTOP__ so the web UI can call native features.

window.__PILOT_DESKTOP__ = {
    version: '3.0.0',
    isDesktopApp: true,

    /**
     * Open an RDP session in a new native window
     */
    openRdpWindow: async function (agentId, hostname, sessionUrl) {
        return invoke('open_rdp_window', {
            agentId: String(agentId),
            hostname: String(hostname),
            sessionUrl: String(sessionUrl),
        });
    },

    /**
     * Close an RDP pop-out window
     */
    closeRdpWindow: async function (agentId) {
        return invoke('close_rdp_window', { agentId: String(agentId) });
    },

    /**
     * Navigate main window to a path
     */
    navigate: async function (path) {
        return invoke('navigate', { path: String(path) });
    },

    /**
     * Get app version
     */
    getVersion: async function () {
        return invoke('get_app_version');
    },

    /**
     * Read from clipboard
     */
    readClipboard: async function () {
        const { readText } = window.__TAURI__.clipboardManager;
        return readText();
    },

    /**
     * Write to clipboard
     */
    writeClipboard: async function (text) {
        const { writeText } = window.__TAURI__.clipboardManager;
        return writeText(text);
    },
};

// Start
init();
