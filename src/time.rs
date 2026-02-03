use std::collections::HashMap;

use chrono::{Datelike, Duration, NaiveDate, TimeZone};
use chrono_tz::Tz;

use crate::Span;

pub fn human_time(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    if hours > 0 {
        if minutes == 0 {
            format!("{}h", hours)
        } else {
            format!("{}h {}m", hours, minutes)
        }
    } else if minutes > 0 {
        if seconds == 0 {
            format!("{}m", minutes)
        } else {
            format!("{}m {}s", minutes, seconds)
        }
    } else {
        "<1m".to_string()
    }
}

pub fn create_timezone_timestamp(
    tz: &Tz,
    date: &chrono::NaiveDate,
    hour: u32,
    minute: u32,
    second: u32,
) -> Result<f64, String> {
    tz.with_ymd_and_hms(date.year(), date.month(), date.day(), hour, minute, second)
        .single()
        .map(|dt| dt.timestamp() as f64)
        .ok_or_else(|| "Invalid date/time".to_string())
}

pub fn generate_date_range(start: NaiveDate, end: NaiveDate) -> Vec<NaiveDate> {
    let mut dates = Vec::new();
    let mut current = start;
    while current <= end {
        dates.push(current);
        current += Duration::days(1);
    }
    dates
}

pub fn process_span_into_buckets(
    span: &Span,
    tz: &Tz,
    day_buckets: &mut HashMap<chrono::NaiveDate, u32>,
) {
    let start_dt_utc =
        match chrono::DateTime::<chrono::Utc>::from_timestamp(span.start_time as i64, 0) {
            Some(dt) => dt,
            None => {
                eprintln!("Invalid start timestamp: {}", span.start_time);
                return;
            }
        };
    let end_dt_utc = match chrono::DateTime::<chrono::Utc>::from_timestamp(span.end_time as i64, 0)
    {
        Some(dt) => dt,
        None => {
            eprintln!("Invalid end timestamp: {}", span.end_time);
            return;
        }
    };

    let start_local = start_dt_utc.with_timezone(tz);
    let end_local = end_dt_utc.with_timezone(tz);
    let start_date = start_local.date_naive();
    let end_date = end_local.date_naive();

    if start_date == end_date {
        *day_buckets.entry(start_date).or_insert(0) += span.duration.round() as u32;
    } else {
        split_span_across_days(span, &start_local, &end_local, tz, day_buckets);
    }
}

fn split_span_across_days(
    span: &Span,
    start_local: &chrono::DateTime<Tz>,
    end_local: &chrono::DateTime<Tz>,
    tz: &Tz,
    day_buckets: &mut HashMap<chrono::NaiveDate, u32>,
) {
    let mut current = *start_local;
    let mut remaining = span.duration;
    let end_date = end_local.date_naive();

    while current.date_naive() < end_date {
        let next_midnight = match tz
            .with_ymd_and_hms(current.year(), current.month(), current.day(), 23, 59, 59)
            .single()
        {
            Some(dt) => dt,
            None => {
                eprintln!("Invalid next midnight date");
                break;
            }
        };

        let seconds = (next_midnight.timestamp() - current.timestamp() + 1) as f64;
        let to_add = seconds.min(remaining).round() as u32;
        *day_buckets.entry(current.date_naive()).or_insert(0) += to_add;
        remaining -= seconds;
        current = next_midnight + Duration::seconds(1);
    }

    *day_buckets.entry(end_date).or_insert(0) += remaining.round() as u32;
}
