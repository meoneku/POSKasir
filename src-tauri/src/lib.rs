pub mod commands;
pub mod db;
pub mod models;

use db::Database;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Ambil direktori app data lokal untuk penyimpanan database yang aman
            let app_data_dir = app.path().app_data_dir().ok();
            let db_path = app_data_dir.map(|mut p| {
                p.push("pos_kasir.db");
                p
            });

            // Inisialisasi Database SQLite WAL Mode
            let db = Database::init(db_path).expect("Gagal menginisialisasi SQLite database");
            app.manage(db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_store_settings,
            commands::update_store_setting,
            commands::get_categories,
            commands::get_products,
            commands::create_product,
            commands::get_taxes,
            commands::get_payment_methods,
            commands::get_current_shift,
            commands::start_shift,
            commands::close_shift,
            commands::add_cash_movement,
            commands::create_transaction,
            commands::get_recent_transactions,
            commands::get_customers,
            commands::get_suppliers,
            commands::get_dashboard_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
