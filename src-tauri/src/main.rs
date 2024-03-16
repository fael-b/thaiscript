// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use entity::letter_variant::Model as LetterVariant;
use sea_orm::DatabaseConnection;
use std::fs;
use tauri::Manager;
use thaiscript_db_service::{DbService, Query};

struct AppState {
    db: DatabaseConnection,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_letter_variants(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<LetterVariant>, String> {
    let db = state.db.clone();
    Query::find_all_letter_variants(&db)
        .await
        .map_err(|e| e.to_string())
}

#[tokio::main]
async fn main() {
    // Get the database directory
    let data_dir = get_data_dir();
    println!("Data directory: {}", data_dir);

    // Initialize the DB service
    let db_service = DbService::build(data_dir);

    // Check if the DB exists, if not, initialize it
    if db_service.does_db_exist() {
        println!("DB exists, skipping initialization");
    } else {
        println!("DB does not exist, initializing...");
        db_service.init_db().await.expect("Could not initialize DB");
    }

    let db_connection = db_service
        .get_db_connection()
        .await
        .expect("Could not connect to DB");

    tauri::Builder::default()
        .manage(AppState { db: db_connection })
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                // Wait for 2 seconds for aesthetic purposes
                // WARNING: Do not remove this line, some OSes will close the app immediately if the splashscreen is closed too quickly
                std::thread::sleep(std::time::Duration::from_secs(2));

                // After it's done, close the splashscreen and display the main window
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_letter_variants])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_data_dir() -> String {
    let home_dir = match tauri::api::path::home_dir() {
        Some(val) => val,
        None => panic!("Could not get home directory"),
    };
    let data_dir = home_dir.join(".thaiscript/data");
    if let Err(_) = fs::metadata(&data_dir) {
        fs::create_dir_all(&data_dir).expect("Could not create data directory");
    }
    data_dir.to_str().unwrap().to_owned()
}
