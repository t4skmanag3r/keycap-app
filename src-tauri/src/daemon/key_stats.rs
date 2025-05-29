use chrono::{Local, NaiveDate};
use log::info;
use rdev::Key;
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::error::Error;

use super::key_map::{ID_TO_KEY, KEY_TO_ID};

pub struct KeyStats {
    conn: Connection,
    buffer: HashMap<(Key, i32, NaiveDate), usize>,
    app_cache: HashMap<String, i32>,
    buffer_count: usize,
    buffer_limit: usize,
}

const DEFAULT_BUFFER_LIMIT: usize = 100;

impl KeyStats {
    pub fn new(db_path: &str, buffer_limit: Option<usize>) -> Result<Self, Box<dyn Error>> {
        let mut conn = Connection::open(db_path)?;

        // Create the key_stats table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS key_stats (
            key_id INTEGER NOT NULL,
            app_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            count INTEGER NOT NULL,
            PRIMARY KEY (key_id, app_id, date)
        )",
            [],
        )?;

        // Create the key_mapping table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS key_mapping (
            key_id INTEGER PRIMARY KEY,
            key_str TEXT NOT NULL UNIQUE
        )",
            [],
        )?;

        // Create the app_mapping table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS app_mapping (
            app_id INTEGER PRIMARY KEY AUTOINCREMENT,
            app_name TEXT NOT NULL UNIQUE
        )",
            [],
        )?;

        // Populate the key_mapping table
        let transaction = conn.transaction()?;
        {
            let mut stmt = transaction
                .prepare("INSERT OR IGNORE INTO key_mapping (key_id, key_str) VALUES (?, ?)")?;

            for (key, &id) in KEY_TO_ID.iter() {
                stmt.execute(params![id, format!("{:?}", key)])?;
            }
        }
        transaction.commit()?;

        // Set buffer limit
        let new_buffer_limit: usize;
        match buffer_limit {
            Some(limit) => {
                new_buffer_limit = limit;
            }
            None => {
                new_buffer_limit = DEFAULT_BUFFER_LIMIT;
            }
        }

        Ok(Self {
            conn,
            buffer: HashMap::new(),
            app_cache: HashMap::new(),
            buffer_count: 0,
            buffer_limit: new_buffer_limit,
        })
    }

    pub fn get_app_id(&mut self, app_name: &str) -> Result<i32, Box<dyn Error>> {
        if let Some(&id) = self.app_cache.get(app_name) {
            return Ok(id);
        }

        let mut stmt = self
            .conn
            .prepare("INSERT OR IGNORE INTO app_mapping (app_name) VALUES (?)")?;
        stmt.execute(params![app_name])?;

        let app_id: i32 = self.conn.query_row(
            "SELECT app_id FROM app_mapping WHERE app_name = ?",
            params![app_name],
            |row| row.get(0),
        )?;

        self.app_cache.insert(app_name.to_string(), app_id);
        Ok(app_id)
    }

    pub fn key_to_id(&self, key: &Key) -> i32 {
        *KEY_TO_ID.get(key).unwrap_or(&0)
    }

    pub fn id_to_key(&self, id: i32) -> Option<Key> {
        ID_TO_KEY.get(&id).cloned()
    }

    pub fn update(&mut self, key: Key, app_name: &str) -> Result<(), Box<dyn Error>> {
        let today = Local::now().date_naive();
        let app_id = self.get_app_id(app_name)?;
        let entry = self.buffer.entry((key, app_id, today)).or_insert(0);
        *entry += 1;
        self.buffer_count += 1;

        if self.buffer_count >= self.buffer_limit {
            self.flush_buffer()?;
        }
        Ok(())
    }

    pub fn flush_buffer(&mut self) -> Result<(), Box<dyn Error>> {
        info!(
            "Buffer size reached {}. Writing to database...",
            self.buffer_limit
        );
        let transaction = self.conn.transaction()?;
        {
            let mut stmt = transaction.prepare(
                "INSERT INTO key_stats (key_id, app_id, date, count)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(key_id, app_id, date) DO UPDATE SET
                 count = count + ?4",
            )?;

            let key_to_id = &*KEY_TO_ID;

            for ((key, app_id, date), count) in self.buffer.drain() {
                let key_id = *key_to_id.get(&key).unwrap_or(&0);
                stmt.execute(params![key_id, app_id, date.to_string(), count])?;
            }
        }
        self.buffer_count = 0;
        transaction.commit()?;
        info!("Database write complete. Buffer cleared.");
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        self.flush_buffer()
    }
}
