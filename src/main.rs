use axum::{Router, http::StatusCode, http::header, routing::get};
use chrono::{Datelike, TimeZone};
use chrono_tz::Tz;
use plotters::prelude::*;
use plotters::style::RGBColor;
use std::collections::HashMap;

use axum::http::{HeaderMap, HeaderValue};
use axum::response::{IntoResponse, Response};

use axum::extract::Query;
use serde::Deserialize;

const DEFAULT_ROWS: usize = 7;
const DEFAULT_COLS: usize = 53;

const PALETTE_GITHUB_LIGHT: [RGBColor; 5] = [
    RGBColor(235, 237, 240), // level 0 (no activity)
    RGBColor(155, 233, 168), // level 1
    RGBColor(64, 196, 99),   // level 2
    RGBColor(48, 161, 78),   // level 3
    RGBColor(33, 110, 57),   // level 4 (most activity)
];
const GITHUB_PALETTE_DARK: [RGBColor; 5] = [
    RGBColor(22, 27, 34),    // level 0 (no activity)
    RGBColor(0, 92, 46),     // level 1
    RGBColor(0, 130, 60),    // level 2
    RGBColor(57, 166, 84),   // level 3
    RGBColor(112, 201, 133), // level 4 (most activity)
];

#[derive(Debug, Deserialize)]
struct SvgParams {
    id: String,
    #[serde(default = "default_timezone")]
    timezone: String,
    #[serde(default = "default_cell_size")]
    cell_size: usize,
    #[serde(default = "default_theme")]
    theme: String,
}

fn default_theme() -> String {
    "dark".to_string()
}

#[derive(Debug, Deserialize)]
struct ResponseData {
    spans: Vec<Span>,
}

#[derive(Debug, Deserialize)]
struct Span {
    start_time: f64,
    end_time: f64,
    #[allow(dead_code)]
    duration: f64,
}

fn default_timezone() -> String {
    "Europe/London".to_string()
}
fn default_cell_size() -> usize {
    50
}

fn human_time(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

async fn make_heatmap_svg(Query(params): Query<SvgParams>) -> Response {
    // Parse timezone string using chrono-tz
    let tz: Tz = match params.timezone.parse() {
        Ok(tz) => tz,
        Err(_) => {
            eprintln!("Unsupported timezone: {}", params.timezone);
            return (StatusCode::BAD_REQUEST, "Unsupported timezone".to_string()).into_response();
        }
    };
    // Get now in user's timezone
    let now_utc = chrono::Utc::now();
    let now_local = now_utc.with_timezone(&tz);
    let today = now_local.date_naive();
    let one_year_ago = (now_local - chrono::Duration::days(365)).date_naive();
    let mut day_buckets: HashMap<chrono::NaiveDate, u32> = HashMap::new();
    let mut tooltips: Vec<String> = Vec::new();
    {
        // Fetch data for the given id
        let resp = reqwest::get(&format!(
            "https://hackatime.hackclub.com/api/v1/users/{}/heartbeats/spans",
            params.id
        ))
        .await;
        if let Err(err) = resp {
            eprintln!("Error fetching data: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response();
        }

        // Parse JSON response
        let json_resp = resp.unwrap().json::<ResponseData>().await;
        if let Err(err) = json_resp {
            eprintln!("Error parsing JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response();
        }
        let spans = json_resp.unwrap().spans;
        // Calculate timestamps for one year ago and today in user's timezone
        let one_year_ago_ts = tz
            .with_ymd_and_hms(
                one_year_ago.year(),
                one_year_ago.month(),
                one_year_ago.day(),
                0,
                0,
                0,
            )
            .single()
            .unwrap()
            .timestamp() as f64;
        let today_end_ts = tz
            .with_ymd_and_hms(today.year(), today.month(), today.day(), 23, 59, 59)
            .single()
            .unwrap()
            .timestamp() as f64;
        for span in spans
            .into_iter()
            .filter(|span| span.end_time >= one_year_ago_ts && span.start_time <= today_end_ts)
        {
            // Convert span start and end to user's timezone
            let start_dt_utc =
                chrono::DateTime::<chrono::Utc>::from_timestamp(span.start_time as i64, 0).unwrap();
            let end_dt_utc =
                chrono::DateTime::<chrono::Utc>::from_timestamp(span.end_time as i64, 0).unwrap();
            let start_local = start_dt_utc.with_timezone(&tz);
            let end_local = end_dt_utc.with_timezone(&tz);
            let start_date = start_local.date_naive();
            let end_date = end_local.date_naive();
            if start_date == end_date {
                // If the span is within a single day, just add the duration (rounded to seconds)
                let entry = day_buckets.entry(start_date).or_insert(0);
                *entry += span.duration.round() as u32;
            } else {
                // If the span crosses days, split duration by actual seconds per day in user's timezone
                let mut current = start_local;
                let mut remaining = span.duration;
                while current.date_naive() < end_date {
                    let next_midnight = tz
                        .with_ymd_and_hms(
                            current.year(),
                            current.month(),
                            current.day(),
                            23,
                            59,
                            59,
                        )
                        .single()
                        .unwrap();
                    let seconds = (next_midnight.timestamp() - current.timestamp() + 1) as f64;
                    let entry = day_buckets.entry(current.date_naive()).or_insert(0);
                    let to_add = seconds.min(remaining).round() as u32;
                    *entry += to_add;
                    remaining -= seconds;
                    current = next_midnight + chrono::Duration::seconds(1);
                }
                // Add the rest to the last day
                let entry = day_buckets.entry(end_date).or_insert(0);
                *entry += remaining.round() as u32;
            }
        }
    }

    println!("Day buckets: {:?}", day_buckets);

    let mut svg_buf = String::new();
    {
        // Create in-memory SVG backend
        let width = (DEFAULT_COLS * params.cell_size) as u32;
        let height = (DEFAULT_ROWS * params.cell_size) as u32;
        let backend = SVGBackend::with_string(&mut svg_buf, (width, height));
        let root = backend.into_drawing_area();
        if let Err(err) = root.fill(&WHITE) {
            eprintln!("Error filling drawing area: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response();
        }

        // Generate all dates from one_year_ago to today
        let mut all_dates = Vec::new();
        let mut current = one_year_ago;
        while current <= today {
            all_dates.push(current);
            current = current + chrono::Duration::days(1);
        }

        // Collect all values for quantile calculation
        let mut values: Vec<u32> = all_dates
            .iter()
            .map(|date| *day_buckets.get(date).unwrap_or(&0))
            .collect();
        values.sort_unstable();
        let quantile = |v: u32| -> usize {
            if v == 0 || values.is_empty() {
                0
            } else {
                // 4 quantiles (5 levels)
                let idx = values.binary_search(&v).unwrap_or_else(|x| x);
                let q = (idx as f32 / values.len() as f32 * 4.0).floor() as usize + 1;
                q.min(4)
            }
        };

        // Select palette based on theme param (default: dark)
        let palette = match params.theme.as_str() {
            "light" => &PALETTE_GITHUB_LIGHT,
            _ => &GITHUB_PALETTE_DARK,
        };

        for (i, date) in all_dates.iter().enumerate() {
            let seconds = *day_buckets.get(date).unwrap_or(&0);
            let col = i / DEFAULT_ROWS;
            let row = i % DEFAULT_ROWS;

            let x0 = (col * params.cell_size) as i32;
            let y0 = (row * params.cell_size) as i32;
            let x1 = ((col + 1) * params.cell_size) as i32;
            let y1 = ((row + 1) * params.cell_size) as i32;

            let color = palette[quantile(seconds)];
            let color = color.to_rgba();

            // Draw rectangle
            if let Err(err) = root.draw(&Rectangle::new([(x0, y0), (x1, y1)], color.filled())) {
                eprintln!("Error drawing rectangle: {:?}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response();
            }

            // Add tooltip
            let label = human_time(seconds).to_string();
            tooltips.push(label);
        }

        if let Err(err) = root.present() {
            eprintln!("Error presenting drawing area: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response();
        }
    }

    // Add rounded corners to rectangles
    let radius = params.cell_size as f32 / 4.0;
    svg_buf = svg_buf.replace(
        "<rect ",
        &format!("<rect rx=\"{:.1}\" ry=\"{:.1}\" ", radius, radius),
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("image/svg+xml"),
    );
    (StatusCode::OK, headers, svg_buf).into_response()
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `GET /svg` goes to `make_heatmap_svg` with query params
        .route("/svg", get(make_heatmap_svg));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
