use crate::heatmap::HeatmapDataPoint;
use chrono::DateTime;
use gpx::{Gpx, Waypoint};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn parse_gpx_file(file_path: &Path) -> Option<Vec<HeatmapDataPoint>> {
    let file = File::open(file_path).ok()?;
    let buf_reader = BufReader::new(file);
    let gpx: Gpx = gpx::read(buf_reader).ok()?;

    if gpx.tracks.is_empty() {
        return None;
    }

    let data = gpx.tracks[0].segments[0]
        .points
        .iter()
        .flat_map(extract_position_from_waypoint)
        .collect();

    Some(data)
}

fn extract_position_from_waypoint(waypoint: &Waypoint) -> Option<HeatmapDataPoint> {
    let lat = waypoint.point().y();
    let lng = waypoint.point().x();
    let timestamp_s = DateTime::parse_from_rfc3339(&waypoint.time?.format().ok()?)
        .ok()?
        .timestamp();

    Some(HeatmapDataPoint::new(lat, lng, timestamp_s))
}
