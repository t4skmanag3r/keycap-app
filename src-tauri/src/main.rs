#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::{error, info};

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Manager, Wry};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

mod commands;
mod daemon;
mod key_mapping;

use commands::{get_app_counts, get_daily_click_counts, get_key_stats};

#[tauri::command]
fn start_daemon(app_handle: tauri::AppHandle) -> Result<(), String> {
    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    // Store the sender in the app state
    app_handle.manage(shutdown_tx);

    // Run the daemon in a separate thread
    let daemon_thread = thread::spawn(move || {
        if let Err(e) = daemon::run_daemon(shutdown_rx) {
            error!("Daemon error: {:?}", e);
        }
    });

    // Store the daemon thread handle in the app state, wrapped in an Arc and Mutex
    app_handle.manage(Arc::new(Mutex::new(Some(daemon_thread))));

    Ok(())
}

fn shutdown_app(app_handle: &tauri::AppHandle) {
    info!("Shutting down daemon...");
    if let Some(shutdown_tx) = app_handle.try_state::<mpsc::Sender<()>>() {
        let _ = shutdown_tx.send(());
    }
    if let Some(daemon_thread) =
        app_handle.try_state::<Arc<Mutex<Option<thread::JoinHandle<()>>>>>()
    {
        info!("Waiting for daemon to finish...");
        if let Some(handle) = daemon_thread.lock().unwrap().take() {
            if let Err(e) = handle.join() {
                error!("Error joining daemon thread: {:?}", e);
            }
        }
        info!("Daemon finished. Closing application.");
    }
}

pub fn create_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

pub fn on_system_tray_event(app: &AppHandle<Wry>, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            window.unminimize().unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                app.get_window("main").unwrap().hide().unwrap();

                shutdown_app(app);
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}

fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();
    start_daemon(app_handle.clone())?;

    let window = app.get_window("main").unwrap();

    // Handle window events
    window.clone().on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            window.hide().unwrap();
            api.prevent_close();
        }
    });

    Ok(())
}

fn main() {
    // Initialize the logger
    use env_logger::{Builder, Env};

    Builder::from_env(Env::default().default_filter_or("debug")).init();

    tauri::Builder::default()
        .system_tray(create_system_tray())
        .on_system_tray_event(on_system_tray_event)
        .setup(setup_app)
        .invoke_handler(tauri::generate_handler![
            get_key_stats,
            get_app_counts,
            get_daily_click_counts,
            start_daemon
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
