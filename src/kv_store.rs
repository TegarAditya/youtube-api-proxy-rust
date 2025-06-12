use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, params};
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
pub struct KeyValue {
    pub value: String,
    pub cached_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct KVStore {
    conn: Arc<Mutex<Connection>>,
}

impl KVStore {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let store = KVStore {
            conn: Arc::new(Mutex::new(conn)),
        };

        store.init()?;
        Ok(store)
    }

    fn init(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS kv_store (
                key       TEXT PRIMARY KEY,
                value     TEXT NOT NULL,
                cached_at DATETIME NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO kv_store (key, value, cached_at) VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, cached_at = excluded.cached_at",
            params![key, value, Utc::now()],
        )?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<KeyValue>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value, cached_at FROM kv_store WHERE key = ?1")?;

        let mut rows = stmt.query_map(params![key], |row| {
            Ok(KeyValue {
                value: row.get(0)?,
                cached_at: row.get(1)?,
            })
        })?;

        rows.next().transpose()
    }

    pub fn clear(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM kv_store", [])?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn delete(&self, key: &str) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let changes = conn.execute("DELETE FROM kv_store WHERE key = ?1", params![key])?;
        Ok(changes)
    }

    pub fn health_check(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let _: i32 = conn.query_row("SELECT 1", [], |row| row.get(0))?;
        Ok(())
    }
}
