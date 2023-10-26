use crate::fit::parse_fit_file;
use crate::fs::{path_extension_contains, path_extension_contains_any};
use crate::gpx::parse_gpx_file;
use crate::AppState;
use chrono::{Datelike, NaiveDateTime};
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::{DirEntry, File};
use std::io::{Error, Read, Write};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Export {
    data: HashMap<i32, Vec<Activity>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activity {
    date_timestamp_s: i64,
    heatmap_data_points: Vec<HeatmapDataPoint>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeatmapDataPoint {
    lat: f64,
    lng: f64,
    timestamp_s: i64,
    count: u8,
}

impl Export {
    pub fn new(data: HashMap<i32, Vec<Activity>>) -> Self {
        Self { data }
    }
}

impl Activity {
    pub fn new(date_timestamp_s: i64, heatmap_data_points: Vec<HeatmapDataPoint>) -> Self {
        Self {
            date_timestamp_s,
            heatmap_data_points,
        }
    }
}

impl HeatmapDataPoint {
    pub fn new(lat: f64, lng: f64, timestamp_s: i64) -> Self {
        Self {
            lat,
            lng,
            timestamp_s,
            count: 1,
        }
    }
}

pub fn load_data(path: String, app_state: tauri::State<AppState>) -> Option<String> {
    let now = Instant::now();
    let paths = fs::read_dir(path).ok()?;

    let data: Vec<Activity> = paths
        .into_iter()
        .par_bridge()
        .flat_map(parse_dir_entry)
        .collect();

    let uuid = Uuid::new_v4();
    let export = create_export(data);
    let mut exports = app_state.exports.lock().unwrap();
    exports.insert(uuid, export);

    println!("⚡️ Elapsed: {:.2?}", now.elapsed());

    Some(uuid.to_string())
}

pub fn load_json_export(path: String, app_state: tauri::State<AppState>) -> Option<String> {
    let now = Instant::now();
    let mut file = File::open(path).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let uuid = Uuid::new_v4();
    let export: Export = serde_json::from_str(&buff).unwrap();
    let mut exports = app_state.exports.lock().unwrap();
    exports.insert(uuid, export);

    println!("⚡️ Elapsed: {:.2?}", now.elapsed());

    Some(uuid.to_string())
}

pub fn get_available_years(
    uuid: String,
    app_state: tauri::State<AppState>,
) -> Result<Vec<i32>, ()> {
    let exports = app_state.exports.lock().unwrap();
    let export = exports.get(&Uuid::from_str(&uuid).unwrap()).ok_or(())?;
    let mut dates: Vec<i32> = export.data.keys().copied().collect();
    dates.sort();

    Ok(dates)
}

pub fn display_data(
    uuid: String,
    year: i32,
    app_state: tauri::State<AppState>,
) -> Result<Vec<HeatmapDataPoint>, ()> {
    let exports = app_state.exports.lock().unwrap();
    let export = exports.get(&Uuid::from_str(&uuid).unwrap()).ok_or(())?;
    let data_points = export
        .data
        .get(&year)
        .unwrap()
        .iter()
        .flat_map(|activity| activity.heatmap_data_points.clone())
        .collect();

    Ok(data_points)
}

pub fn display_all_data(
    uuid: String,
    app_state: tauri::State<AppState>,
) -> Result<Vec<HeatmapDataPoint>, ()> {
    let exports = app_state.exports.lock().unwrap();
    let export = exports.get(&Uuid::from_str(&uuid).unwrap()).ok_or(())?;
    let data_points = export
        .data
        .values()
        .flat_map(|activities| {
            activities
                .iter()
                .flat_map(|activity| activity.heatmap_data_points.clone())
        })
        .collect();

    Ok(data_points)
}

pub fn create_json_export(
    path: String,
    uuid: String,
    app_state: tauri::State<AppState>,
) -> Result<(), ()> {
    let exports = app_state.exports.lock().unwrap();
    let export = exports.get(&Uuid::from_str(&uuid).unwrap()).ok_or(())?;
    let export_string = match serde_json::to_string(export) {
        Ok(result) => result,
        Err(_) => return Err(()),
    };

    let file_path = Path::new(&path).join(format!("{}.json", uuid));
    let mut file = File::create(file_path).map_err(|_| ())?;
    write!(file, "{}", export_string).map_err(|_| ())?;

    Ok(())
}

fn parse_dir_entry(dir_entry: Result<DirEntry, Error>) -> Option<Activity> {
    let path = dir_entry.ok()?.path();
    if !path.is_file() {
        return None;
    }

    let positions: Vec<HeatmapDataPoint>;
    if path_extension_contains_any(&path, &["fit.gz", ".fit"]) {
        positions = parse_fit_file(&path)?;
    } else if path_extension_contains(&path, "gpx") {
        positions = parse_gpx_file(&path)?;
    } else {
        return None;
    }

    if positions.is_empty() {
        return None;
    }

    Some(Activity::new(positions.get(0)?.timestamp_s, positions))
}

fn create_export(data: Vec<Activity>) -> Export {
    let map: HashMap<i32, Vec<Activity>> =
        data.iter()
            .fold(HashMap::new(), |mut acc: HashMap<i32, Vec<Activity>>, x| {
                let date = NaiveDateTime::from_timestamp_opt(x.date_timestamp_s, 0)
                    .expect("Unable to find activity's date");
                let year = date.year();

                if let std::collections::hash_map::Entry::Vacant(e) = acc.entry(year) {
                    e.insert(vec![x.clone()]);
                } else {
                    let activities = acc.get_mut(&year).unwrap();
                    activities.push(x.clone());
                }

                acc
            });

    Export::new(map)
}
