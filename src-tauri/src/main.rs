#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dirs;
use lazy_static::lazy_static;
use log::{error, info};
use rusqlite::{Connection, Row};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Manager, Wry};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

mod daemon;
mod key_mapping;

use key_mapping::convert_key_name;

fn get_db_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| {
        error!("Could not determine data directory, using current directory");
        std::env::current_dir().unwrap()
    });
    path.push("keycap");
    std::fs::create_dir_all(&path).unwrap();
    path.push("key_stats.db");
    path
}

lazy_static! {
    static ref DB_PATH: Mutex<PathBuf> = Mutex::new(get_db_path());
}

fn get_db_connection() -> Result<Connection, rusqlite::Error> {
    let path = DB_PATH.lock().unwrap();
    Connection::open(&*path)
}

#[derive(Serialize, Deserialize)]
struct KeyStat {
    key: String,
    count: i32,
}

fn map_row(row: &Row) -> Result<KeyStat, rusqlite::Error> {
    let db_key: String = row.get(0)?;
    let converted_key = convert_key_name(&db_key);
    Ok(KeyStat {
        key: converted_key,
        count: row.get(1)?,
    })
}

#[tauri::command]
fn get_key_stats(app_name: Option<String>, date: Option<String>) -> Result<Vec<KeyStat>, String> {
    let conn = get_db_connection().map_err(|e| e.to_string())?;

    let query = match (app_name.as_ref(), date.as_ref()) {
        (Some(_), Some(_)) => {
            "SELECT km.key_str, SUM(ks.count) as total_count
             FROM key_stats ks
             JOIN key_mapping km ON ks.key_id = km.key_id
             JOIN app_mapping am ON ks.app_id = am.app_id
             WHERE am.app_name = ?1 AND DATE(ks.date) = DATE(?2)
             GROUP BY km.key_id, km.key_str
             ORDER BY total_count DESC"
        }
        (Some(_), None) => {
            "SELECT km.key_str, SUM(ks.count) as total_count
             FROM key_stats ks
             JOIN key_mapping km ON ks.key_id = km.key_id
             JOIN app_mapping am ON ks.app_id = am.app_id
             WHERE am.app_name = ?1
             GROUP BY km.key_id, km.key_str
             ORDER BY total_count DESC"
        }
        (None, Some(_)) => {
            "SELECT km.key_str, SUM(ks.count) as total_count
             FROM key_stats ks
             JOIN key_mapping km ON ks.key_id = km.key_id
             WHERE DATE(ks.date) = DATE(?1)
             GROUP BY km.key_id, km.key_str
             ORDER BY total_count DESC"
        }
        (None, None) => {
            "SELECT km.key_str, SUM(ks.count) as total_count
             FROM key_stats ks
             JOIN key_mapping km ON ks.key_id = km.key_id
             GROUP BY km.key_id, km.key_str
             ORDER BY total_count DESC"
        }
    };

    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;

    let key_stats = match (app_name, date) {
        (Some(app), Some(date)) => stmt.query_map([app, date], map_row),
        (Some(app), None) => stmt.query_map([app], map_row),
        (None, Some(date)) => stmt.query_map([date], map_row),
        (None, None) => stmt.query_map([], map_row),
    }
    .map_err(|e| e.to_string())?;

    let result: Result<Vec<KeyStat>, rusqlite::Error> = key_stats.collect();
    result.map_err(|e| e.to_string())
}

#[tauri::command]
fn get_applications() -> Result<Vec<String>, String> {
    let conn = get_db_connection().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT DISTINCT app_name FROM app_mapping ORDER BY app_name")
        .map_err(|e| e.to_string())?;

    let apps = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    let result: Result<Vec<String>, rusqlite::Error> = apps.collect();
    result.map_err(|e| e.to_string())
}
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
            get_applications,
            start_daemon
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
