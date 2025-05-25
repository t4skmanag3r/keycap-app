#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

mod key_mapping;

use key_mapping::convert_key_name;

#[derive(Serialize, Deserialize)]
struct KeyStat {
    key: String,
    count: i32,
}
#[tauri::command]
fn get_key_stats() -> Result<Vec<KeyStat>, String> {
    let conn = Connection::open("key_stats.db").map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT km.key_str, SUM(ks.count) as total_count
             FROM key_stats ks
             JOIN key_mapping km ON ks.key_id = km.key_id
             GROUP BY km.key_id, km.key_str
             ORDER BY total_count DESC",
        )
        .map_err(|e| e.to_string())?;

    let key_stats = stmt
        .query_map([], |row| {
            let db_key: String = row.get(0)?;
            let converted_key = convert_key_name(&db_key);
            Ok(KeyStat {
                key: converted_key,
                count: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let result: Result<Vec<KeyStat>, rusqlite::Error> = key_stats.collect();
    result.map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_key_stats])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
