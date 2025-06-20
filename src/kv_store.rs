use chrono::{DateTime, Utc};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Result, params};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct KeyValue {
    pub value: String,
    pub cached_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct KVStore {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl KVStore {
    pub fn new(db_path: &str) -> Result<Self> {
        let manager = SqliteConnectionManager::file(db_path).with_init(|c| {
            c.execute_batch(
                "PRAGMA journal_mode = WAL;
                 PRAGMA synchronous = NORMAL;
                 PRAGMA temp_store = MEMORY;
                 PRAGMA busy_timeout = 3000;",
            )
        });

        let pool = Pool::builder()
            .max_size(8)
            .build(manager)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        let store = KVStore {
            pool: Arc::new(pool),
        };

        store.init()?;
        Ok(store)
    }

    fn conn(&self) -> Result<PooledConnection<SqliteConnectionManager>> {
        self.pool
            .get()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))
    }

    fn init(&self) -> Result<()> {
        let conn = self.conn()?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS kv_store (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                cached_at DATETIME NOT NULL
            );
        "#,
        )?;
        Ok(())
    }

    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO kv_store (key, value, cached_at)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, cached_at = excluded.cached_at",
            params![key, value, Utc::now()],
        )?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<KeyValue>> {
        let conn = self.conn()?;
        let mut stmt =
            conn.prepare_cached("SELECT value, cached_at FROM kv_store WHERE key = ?1")?;

        let mut rows = stmt.query_map(params![key], |row| {
            Ok(KeyValue {
                value: row.get(0)?,
                cached_at: row.get(1)?,
            })
        })?;

        rows.next().transpose()
    }

    pub fn clear(&self) -> Result<()> {
        let conn = self.conn()?;
        conn.execute("DELETE FROM kv_store", [])?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn delete(&self, key: &str) -> Result<usize> {
        let conn = self.conn()?;
        let changes = conn.execute("DELETE FROM kv_store WHERE key = ?1", params![key])?;
        Ok(changes)
    }

    pub fn health_check(&self) -> Result<()> {
        let conn = self.conn()?;
        let _: i32 = conn.query_row("SELECT 1", [], |row| row.get(0))?;
        Ok(())
    }
}
