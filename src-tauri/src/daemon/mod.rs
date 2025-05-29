use active_win_pos_rs::get_active_window;
use dirs;
use log::{debug, error, info};
use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

mod key_map;
mod key_stats;
use key_stats::KeyStats;

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

pub fn run_daemon(shutdown_rx: Receiver<()>) -> Result<(), Box<dyn Error>> {
    info!("Daemon starting...");

    let db_path = get_db_path();

    let pressed_keys = Arc::new(Mutex::new(HashSet::new()));
    let key_stats = Arc::new(Mutex::new(KeyStats::new(db_path.to_str().unwrap(), None)?));

    let key_stats_clone = Arc::clone(&key_stats);

    // Create a channel for communication between threads
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread for listening
    let listen_thread = thread::spawn(move || {
        let pressed_keys_clone = Arc::clone(&pressed_keys);
        let key_stats_clone = Arc::clone(&key_stats);

        listen(move |event| {
            if rx.try_recv().is_ok() {
                // Stop signal received
                std::process::exit(0);
            }
            callback(
                event,
                Arc::clone(&pressed_keys_clone),
                Arc::clone(&key_stats_clone),
            )
        })
    });

    // Wait for shutdown signal
    shutdown_rx.recv().unwrap();
    info!("Shutdown signal received. Stopping daemon...");

    if let Ok(mut stats) = key_stats_clone.lock() {
        if let Err(e) = stats.save() {
            error!("Error saving data: {:?}", e);
        } else {
            info!("Data saved successfully.");
        }
    }
    info!("Daemon shutting down gracefully.");

    tx.send(()).unwrap();

    Ok(())
}

fn callback(event: Event, pressed_keys: Arc<Mutex<HashSet<Key>>>, key_stats: Arc<Mutex<KeyStats>>) {
    match event.event_type {
        EventType::KeyPress(key) => {
            let mut keys = pressed_keys.lock().unwrap();
            if keys.insert(key) {
                // Key wasn't in the set, so it's a new press
                let mut stats = key_stats.lock().unwrap();

                // Get the active window information
                let app_name = get_active_window()
                    .map(|window| window.app_name)
                    .unwrap_or_else(|_| "Unknown".to_string());

                if let Err(e) = stats.update(key, &app_name) {
                    error!("Error updating stats: {:?}", e);
                } else {
                    debug!("Key pressed: {:?}, Application: {}", key, app_name);
                }
            }
        }
        EventType::KeyRelease(key) => {
            let mut keys = pressed_keys.lock().unwrap();
            keys.remove(&key);
        }
        _ => {}
    }
}
