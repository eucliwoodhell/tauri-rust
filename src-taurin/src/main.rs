// Prevents additional
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use entity::*;
use std::fs;

fn main() {
    let home_dir = match tauri::api::path::home_dir() {
        Some(path) => path,
        None => panic!("Failed to get home directory"),
    };

    let data_dir = home_dir.join(".tauri-seaorm-template").join("data");
    if let Err(_) = fs::metadata(&data_dir) {
        fs::create_dir_all(&data_dir).expect("Failed to create data directory");
    }

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get() -> Result<(), std::io::Error> {
                Ok(())
}

