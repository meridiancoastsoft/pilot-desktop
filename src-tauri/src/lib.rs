use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WebviewUrl, WebviewWindowBuilder,
};

// ─── Server Configuration ───────────────────────────────────────────
const PROD_URL: &str = "https://app.meridiancoastsoft.com";
const DEV_URL: &str = "https://dev.meridiancoastsoft.com";

/// Get the server URL based on build mode
fn server_url() -> &'static str {
    if cfg!(debug_assertions) {
        DEV_URL
    } else {
        PROD_URL
    }
}

// ─── IPC Commands ───────────────────────────────────────────────────

/// Open a new native window for an RDP session (pop-out)
#[tauri::command]
async fn open_rdp_window(
    app: tauri::AppHandle,
    agent_id: String,
    hostname: String,
    session_url: String,
) -> Result<(), String> {
    let label = format!("rdp-{}", agent_id);
    let title = format!("RDP — {}", hostname);

    // Check if window already exists
    if let Some(window) = app.get_webview_window(&label) {
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    let url = if session_url.starts_with("http") {
        session_url.clone()
    } else {
        format!("{}{}", server_url(), session_url)
    };

    WebviewWindowBuilder::new(&app, &label, WebviewUrl::External(url.parse().unwrap()))
        .title(&title)
        .inner_size(1280.0, 900.0)
        .min_inner_size(800.0, 600.0)
        .resizable(true)
        .center()
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Close an RDP pop-out window
#[tauri::command]
async fn close_rdp_window(app: tauri::AppHandle, agent_id: String) -> Result<(), String> {
    let label = format!("rdp-{}", agent_id);
    if let Some(window) = app.get_webview_window(&label) {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Navigate the main window to a specific page
#[tauri::command]
async fn navigate(app: tauri::AppHandle, path: String) -> Result<(), String> {
    let url = format!("{}{}", server_url(), path);
    if let Some(window) = app.get_webview_window("main") {
        window
            .navigate(url.parse().unwrap())
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Get the server URL (so JS can determine the environment)
#[tauri::command]
fn get_server_url() -> String {
    server_url().to_string()
}

/// Get app version
#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ─── App Setup ──────────────────────────────────────────────────────

pub fn run() {
    tauri::Builder::default()
        // Plugins
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::default().build())
        // IPC command handlers
        .invoke_handler(tauri::generate_handler![
            open_rdp_window,
            close_rdp_window,
            navigate,
            get_server_url,
            get_app_version,
        ])
        // App setup
        .setup(|app| {
            // ── System Tray ──
            let open = MenuItem::with_id(app, "open", "Open Pilot Suite", true, None::<&str>)?;
            let dashboard = MenuItem::with_id(app, "dashboard", "Dashboard", true, None::<&str>)?;
            let agents = MenuItem::with_id(app, "agents", "RMM Agents", true, None::<&str>)?;
            let tickets = MenuItem::with_id(app, "tickets", "Service Desk", true, None::<&str>)?;
            let separator = MenuItem::with_id(app, "sep", "─────────", false, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[&open, &separator, &dashboard, &agents, &tickets, &separator, &quit],
            )?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("Pilot Suite")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "dashboard" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let url = format!("{}/", server_url());
                            let _ = window.navigate(url.parse().unwrap());
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "agents" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let url = format!("{}/rmm/agents", server_url());
                            let _ = window.navigate(url.parse().unwrap());
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "tickets" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let url = format!("{}/servicedesk/tickets", server_url());
                            let _ = window.navigate(url.parse().unwrap());
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // ── Main Window: Load Pilot Suite ──
            if let Some(window) = app.get_webview_window("main") {
                let url = format!("{}/", server_url());
                let _ = window.navigate(url.parse().unwrap());

                // Minimize to tray on close instead of quitting
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = window_clone.hide();
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Pilot Suite Desktop");
}
