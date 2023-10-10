use crate::fs::path_extension_contains;
use crate::heatmap::HeatmapDataPoint;
use fitparser::profile::MesgNum;
use fitparser::{FitDataField, FitDataRecord};
use flate2::bufread::GzDecoder;
use std::fs::{read, File};
use std::io::{BufReader, Read};
use std::path::Path;

pub fn parse_fit_file(path: &Path) -> Option<Vec<HeatmapDataPoint>> {
    let bytes: Vec<u8>;
    if path_extension_contains(path, ".fit.gz") {
        bytes = extract_gunzip_file(path).ok()?;
    } else if path_extension_contains(path, ".fit") {
        bytes = read(path).ok()?;
    } else {
        return None;
    }

    let data = fitparser::from_bytes(bytes.as_slice())
        .ok()?
        .iter()
        .filter(|record| record.kind() == MesgNum::Record)
        .flat_map(extract_position_from_record)
        .collect();

    Some(data)
}

fn extract_gunzip_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    let mut bytes = Vec::new();
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let mut gz_file = GzDecoder::new(buf_reader);
    gz_file.read_to_end(&mut bytes)?;

    Ok(bytes)
}

fn extract_position_from_record(record: &FitDataRecord) -> Option<HeatmapDataPoint> {
    let lat = convert_semicircles_degrees(find_value(record.fields(), "position_lat"))?;
    let lng = convert_semicircles_degrees(find_value(record.fields(), "position_long"))?;
    let timestamp_s = find_value(record.fields(), "timestamp").unwrap_or(0);

    Some(HeatmapDataPoint::new(lat, lng, timestamp_s))
}

fn find_value(fields: &[FitDataField], field_name: &str) -> Option<i64> {
    fields
        .iter()
        .find(|field| field.name() == field_name)
        .and_then(|field| field.value().try_into().ok())
}

fn convert_semicircles_degrees(semicircles: Option<i64>) -> Option<f64> {
    semicircles.map(|value| (value as f64) * (180f64 / 2f64.powf(31f64)))
}
