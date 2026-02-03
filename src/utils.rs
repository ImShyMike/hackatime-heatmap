use axum::http::{HeaderMap, HeaderValue};
use chrono::Datelike;
use reqwest::header;

use crate::time::human_time;

use crate::CACHE_HEADER;

pub fn build_headers(content_type: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(content_type).unwrap(),
    );
    headers.insert(header::CACHE_CONTROL, CACHE_HEADER);
    headers
}

fn format_day_suffix(day: u32) -> &'static str {
    match day % 10 {
        1 if day != 11 => "st",
        2 if day != 12 => "nd",
        3 if day != 13 => "rd",
        _ => "th",
    }
}

pub fn format_cell_label(date: &chrono::NaiveDate, seconds: u32) -> String {
    let date_str = date.format("%B %-d").to_string();
    let suffix = format_day_suffix(date.day());

    if seconds > 0 {
        format!("{} on {}{}", human_time(seconds), date_str, suffix)
    } else {
        format!("No activity on {}{}", date_str, suffix)
    }
}

pub fn validate_ranges(ranges_str: &str) -> Result<Vec<u32>, String> {
    let ranges = ranges_str
        .split(',')
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect::<Vec<u32>>();

    if ranges.len() != 3 {
        return Err("Invalid ranges parameter, must be three comma-separated integers".to_string());
    }
    if !(ranges[0] > ranges[1] && ranges[1] > ranges[2] && ranges[2] > 0) {
        return Err(
            "Invalid ranges parameter, must be three descending positive integers".to_string(),
        );
    }
    if ranges[0] > 100 || ranges[1] > 100 || ranges[2] > 100 {
        return Err("Invalid ranges parameter, values must be within 0 and 100".to_string());
    }

    Ok(ranges)
}
