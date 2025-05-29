use dirs;
use lazy_static::lazy_static;
use log::error;
use rusqlite::Connection;
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::key_mapping::convert_key_name;

lazy_static! {
    static ref DB_PATH: Mutex<PathBuf> = Mutex::new(get_db_path());
}

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

fn get_db_connection() -> Result<Connection, rusqlite::Error> {
    let path = DB_PATH.lock().unwrap();
    Connection::open(&*path)
}

#[derive(Serialize, Deserialize)]
pub struct KeyStat {
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
pub fn get_key_stats(
    app_name: Option<String>,
    date: Option<String>,
) -> Result<Vec<KeyStat>, String> {
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
pub fn get_applications() -> Result<Vec<String>, String> {
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
