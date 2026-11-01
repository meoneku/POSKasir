pub mod schema;

use parking_lot::Mutex;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    pub conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn init(db_path: Option<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = match db_path {
            Some(p) => {
                if let Some(parent) = p.parent() {
                    fs::create_dir_all(parent)?;
                }
                p
            }
            None => {
                let local_path = PathBuf::from("pos_kasir.db");
                local_path
            }
        };

        let conn = Connection::open(&path)?;

        // Aktifkan konfigurasi SQLite WAL Mode untuk performa maksimal & tanpa lock
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA busy_timeout = 5000;",
        )?;

        // Eksekusi pembuatan skema tabel & seed data
        conn.execute_batch(schema::CREATE_TABLES_SQL)?;
        conn.execute_batch(schema::SEED_DATA_SQL)?;

        // Bersihkan nama default lama jika masih ada 'Budi' di basis data lokal yang tersimpan
        let _ = conn.execute(
            "UPDATE users SET full_name = 'Kasir 1' WHERE full_name LIKE '%Budi%'",
            [],
        );

        println!("Database SQLite (WAL Mode) berhasil diinisialisasi di: {:?}", path);

        Ok(Database {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}
