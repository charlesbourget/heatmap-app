// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::heatmap::{Export, HeatmapDataPoint};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

mod fit;
mod fs;
mod gpx;
mod heatmap;

pub struct AppState {
    exports: Mutex<HashMap<Uuid, Export>>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            exports: Mutex::new(HashMap::new()),
        })
        .invoke_handler(tauri::generate_handler![
            load_fit_files,
            get_available_years,
            display_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn load_fit_files(path: String, app_state: tauri::State<AppState>) -> String {
    println!("load_fit_files[{}]", path);

    let uuid = match heatmap::load_data(path, app_state) {
        Some(uuid) => uuid,
        None => String::from(""),
    };

    println!("load_fit_files -> {}", uuid);

    uuid
}

#[tauri::command]
fn get_available_years(uuid: String, app_state: tauri::State<AppState>) -> Vec<i32> {
    println!("get_available_years[{}]", uuid);

    let years = match heatmap::get_available_years(uuid, app_state) {
        Ok(years) => years,
        Err(_) => Vec::new(),
    };

    println!("get_available_years -> {:?}", years);

    years
}

#[tauri::command]
fn display_data(
    uuid: String,
    year: i32,
    app_state: tauri::State<AppState>,
) -> Vec<HeatmapDataPoint> {
    println!("display_data[{}]", uuid);

    let data = match heatmap::display_data(uuid, year, app_state) {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };

    println!("display_data -> {}", data.len());

    data
}
