use axum::{
    Router,
    extract::{OriginalUri, State},
    http::StatusCode,
    http::header,
    routing::get,
};
use chrono::{Datelike, TimeZone};
use chrono_tz::Tz;
use std::time::{Duration, Instant};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tower_http::catch_panic::CatchPanicLayer;

use axum::http::{HeaderMap, HeaderValue};
use axum::response::{IntoResponse, Response};

use axum::extract::Query;
use serde::Deserialize;
use svg::Document;
use svg::node::element::{Rectangle, Title};

const DEFAULT_ROWS: usize = 7;
const DEFAULT_COLS: usize = 53;
const RESPONSE_CACHE_DURATION_SECONDS: u64 = 60 * 5; // (5 minutes)
const REQUEST_CACHE_DURATION_SECONDS: u64 = 60 * 5; // (5 minutes)
const CACHE_HEADER: HeaderValue = HeaderValue::from_static("public, max-age=300"); // 5 minutes
const MAX_RESPONSE_CACHE_ENTRIES: usize = 200;
const MAX_REQUEST_CACHE_ENTRIES: usize = 25;

const PALETTE_GITHUB_LIGHT: [(u8, u8, u8); 5] = [
    (235, 237, 240), // level 0 (no activity)
    (155, 233, 168), // level 1
    (64, 196, 99),   // level 2
    (48, 161, 78),   // level 3
    (33, 110, 57),   // level 4 (most activity)
];
const GITHUB_PALETTE_DARK: [(u8, u8, u8); 5] = [
    (22, 27, 34),    // level 0 (no activity)
    (0, 92, 46),     // level 1
    (0, 130, 60),    // level 2
    (57, 166, 84),   // level 3
    (112, 201, 133), // level 4 (most activity)
];

const PALLETE_CATPUCCIN_FRAPPE: [(u8, u8, u8); 5] = [
    (204, 208, 218), // level 0 (no activity)
    (64, 160, 43),   // level 1
    (223, 142, 29),  // level 2
    (254, 100, 11),  // level 3
    (210, 15, 57),   // level 4 (most activity)
];

const PALLETE_CATPUCCIN_MOCHA: [(u8, u8, u8); 5] = [
    (49, 50, 68),    // level 0 (no activity)
    (166, 227, 161), // level 1
    (249, 226, 175), // level 2
    (250, 179, 135), // level 3
    (243, 139, 168), // level 4 (most activity)
];

#[derive(Clone)]
struct AppState {
    response_cache: Arc<Mutex<HashMap<SvgParams, (String, Instant)>>>,
    request_cache: Arc<Mutex<HashMap<String, (RequestData, Instant)>>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
struct SvgParams {
    id: Option<String>,
    #[serde(default = "default_timezone")]
    timezone: String,
    #[serde(default = "default_cell_size")]
    cell_size: usize,
    #[serde(default = "default_padding")]
    padding: usize,
    #[serde(default = "default_rounding")]
    rounding: u8,
    #[serde(default = "default_theme")]
    theme: String,
    #[serde(default = "default_ranges")]
    ranges: String,
    #[serde(default = "default_standalone")]
    standalone: bool,
}

fn default_timezone() -> String {
    "Europe/London".to_string()
}
fn default_cell_size() -> usize {
    15
}
fn default_padding() -> usize {
    2
}
fn default_rounding() -> u8 {
    50
}
fn default_theme() -> String {
    "dark".to_string()
}
fn default_ranges() -> String {
    "70,30,10".to_string()
}
fn default_standalone() -> bool {
    false
}

fn enforce_cache_limit<K, V>(cache: &mut HashMap<K, (V, Instant)>, max_entries: usize)
where
    K: Eq + std::hash::Hash + Clone,
{
    if cache.len() >= max_entries {
        // Collect entries with their timestamps
        let mut entries: Vec<_> = cache
            .iter()
            .map(|(k, (_, timestamp))| (k.clone(), *timestamp))
            .collect();
        // Sort by timestamp (oldest first)
        entries.sort_by_key(|(_, timestamp)| *timestamp);
        // Remove oldest entries until we're under the limit
        let to_remove = cache.len() - max_entries + 1;
        for (key, _) in entries.into_iter().take(to_remove) {
            cache.remove(&key);
        }
    }
}
fn build_headers(content_type: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(content_type).unwrap(),
    );
    headers.insert(header::CACHE_CONTROL, CACHE_HEADER);
    headers
}

#[derive(Debug, Deserialize, Clone)]
struct RequestData {
    spans: Vec<Span>,
}

#[derive(Debug, Deserialize, Clone)]
struct Span {
    start_time: f64,
    end_time: f64,
    duration: f64,
}

fn human_time(seconds: u32) -> String {
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

fn embed_page(svg: &str, standalone: bool) -> String {
    if !standalone {
        return svg.to_string();
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Hackatime Heatmap</title>
    <style>
        :root {{
            --bg-color: #ffffff;
            --text-color: #24292f;
        }}
        @media (prefers-color-scheme: dark) {{
            :root {{
                --bg-color: #0d1117;
                --text-color: #c9d1d9;
            }}
        }}
        body {{
            margin: 0;
            padding: 0;
            background-color: var(--bg-color);
            color: var(--text-color);
            min-height: 100vh;
            display: flex;
            align-items: flex-start;
            justify-content: center;
            overflow: hidden;
        }}
        .container {{
            text-align: center;
            padding: 20px;
        }}
        .title {{
            font-size: 2rem;
            font-weight: 600;
            margin-bottom: 24px;
            color: var(--text-color);
        }}
        .heatmap-container {{
            display: inline-block;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1 class="title">Hackatime Activity Heatmap</h1>
        <p>Hover over each cell to see detailed data for that day!</p>
        <div class="heatmap-container">
            {}
        </div>
    </div>
    <script>
        const params = new URLSearchParams(window.location.search);
        const currentTheme = params.get('theme') || 'auto';
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        
        let preferredTheme;
        if (currentTheme === 'auto') {{
            preferredTheme = prefersDark ? 'dark' : 'light';
        }} else if (currentTheme === 'catppuccin') {{
            preferredTheme = prefersDark ? 'catppuccin_dark' : 'catppuccin_light';
        }} else {{
            preferredTheme = currentTheme;
        }}
        
        if (currentTheme !== preferredTheme) {{
            params.set('theme', preferredTheme);
            window.location.search = params.toString();
        }}
    </script>
</body>
</html>"#,
        svg
    )
}

async fn make_heatmap_svg(
    State(state): State<AppState>,
    Query(params): Query<SvgParams>,
    OriginalUri(uri): OriginalUri,
) -> Response {
    let current_time = chrono::Utc::now().timestamp();
    println!("{} - {}", current_time, uri);

    let now = Instant::now();
    let cached_response = {
        let cache = match state.response_cache.lock() {
            Ok(cache) => cache,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response(),
        };
        cache.get(&params).and_then(|(response, timestamp)| {
            if now.duration_since(*timestamp) < Duration::from_secs(RESPONSE_CACHE_DURATION_SECONDS)
            {
                Some(response.clone())
            } else {
                None
            }
        })
    };

    let id = match &params.id {
        Some(id) => id.clone(),
        None => return (StatusCode::BAD_REQUEST, "Missing required parameter: id").into_response(),
    };

    let ranges = params
        .ranges
        .split(',')
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect::<Vec<u32>>();
    if ranges.len() != 3 {
        return (
            StatusCode::BAD_REQUEST,
            "Invalid ranges parameter, must be three comma-separated integers".to_string(),
        )
            .into_response();
    }
    if !(ranges[0] > ranges[1] && ranges[1] > ranges[2] && ranges[2] > 0) {
        return (
            StatusCode::BAD_REQUEST,
            "Invalid ranges parameter, must be three descending positive integers".to_string(),
        )
            .into_response();
    }
    if ranges[0] > 100 || ranges[1] > 100 || ranges[2] > 100 {
        return (
            StatusCode::BAD_REQUEST,
            "Invalid ranges parameter, values must be within 0 and 100".to_string(),
        )
            .into_response();
    }

    let content_type = if params.standalone {
        "text/html"
    } else {
        "image/svg+xml"
    };

    if let Some(response) = cached_response {
        return (StatusCode::OK, build_headers(content_type), response).into_response();
    }

    // Parse timezone string
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
        let cached_request = {
            let cache = match state.request_cache.lock() {
                Ok(cache) => cache,
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response();
                }
            };
            cache.get(&id).and_then(|(request, timestamp)| {
                if now.duration_since(*timestamp)
                    < Duration::from_secs(REQUEST_CACHE_DURATION_SECONDS)
                {
                    Some(request.clone())
                } else {
                    None
                }
            })
        };

        let spans: Vec<Span>;
        if let Some(request) = cached_request {
            spans = request.spans.clone();
        } else {
            // Fetch data for the given id
            let resp = reqwest::get(&format!(
                "https://hackatime.hackclub.com/api/v1/users/{}/heartbeats/spans",
                id
            ))
            .await;
            let resp = match resp {
                Ok(resp) => resp,
                Err(err) => {
                    eprintln!("Error fetching data: {:?}", err);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response();
                }
            };

            // Parse JSON response
            let json_resp = resp.json::<RequestData>().await;
            let json_resp = match json_resp {
                Ok(json_resp) => json_resp,
                Err(err) => {
                    eprintln!("Error parsing JSON: {:?}", err);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response();
                }
            };

            spans = json_resp.spans;

            {
                let mut request_cache = match state.request_cache.lock() {
                    Ok(cache) => cache,
                    Err(_) => {
                        return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response();
                    }
                };
                enforce_cache_limit(&mut request_cache, MAX_REQUEST_CACHE_ENTRIES);
                request_cache.insert(
                    id,
                    (
                        RequestData {
                            spans: spans.clone(),
                        },
                        now,
                    ),
                );
            }
        }

        // Calculate timestamps for one year ago and today in user's timezone
        let one_year_ago_ts = match tz
            .with_ymd_and_hms(
                one_year_ago.year(),
                one_year_ago.month(),
                one_year_ago.day(),
                0,
                0,
                0,
            )
            .single()
        {
            Some(dt) => dt.timestamp() as f64,
            None => {
                eprintln!("Invalid date for one year ago");
                return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response();
            }
        };
        let today_end_ts = match tz
            .with_ymd_and_hms(today.year(), today.month(), today.day(), 23, 59, 59)
            .single()
        {
            Some(dt) => dt.timestamp() as f64,
            None => {
                eprintln!("Invalid date for today");
                return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response();
            }
        };
        for span in spans
            .into_iter()
            .filter(|span| span.end_time >= one_year_ago_ts && span.start_time <= today_end_ts)
        {
            // Convert span start and end to user's timezone
            let start_dt_utc =
                match chrono::DateTime::<chrono::Utc>::from_timestamp(span.start_time as i64, 0) {
                    Some(dt) => dt,
                    None => {
                        eprintln!("Invalid start timestamp: {}", span.start_time);
                        continue; // Skip invalid spans
                    }
                };
            let end_dt_utc =
                match chrono::DateTime::<chrono::Utc>::from_timestamp(span.end_time as i64, 0) {
                    Some(dt) => dt,
                    None => {
                        eprintln!("Invalid end timestamp: {}", span.end_time);
                        continue; // Skip invalid spans
                    }
                };
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
                    let next_midnight = match tz
                        .with_ymd_and_hms(
                            current.year(),
                            current.month(),
                            current.day(),
                            23,
                            59,
                            59,
                        )
                        .single()
                    {
                        Some(dt) => dt,
                        None => {
                            eprintln!("Invalid next midnight date");
                            break; // Exit the loop on error
                        }
                    };
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

    let svg_buf: String;
    {
        // Create SVG document
        let width = (DEFAULT_COLS * params.cell_size + params.padding * (DEFAULT_COLS + 1)) as u32;
        let height = (DEFAULT_ROWS * params.cell_size + params.padding * (DEFAULT_ROWS + 1)) as u32;
        let radius = (params.rounding.min(100) as f32 / 200.0) * params.cell_size as f32;
        let mut document = Document::new()
            .set("width", width)
            .set("height", height)
            .set("viewBox", format!("0 0 {} {}", width, height));

        // Generate all dates from one_year_ago to today
        let mut all_dates = Vec::new();
        let mut current = one_year_ago;
        while current <= today {
            all_dates.push(current);
            current += chrono::Duration::days(1);
        }

        // Collect all values and make a color scale function
        let mut values: Vec<u32> = all_dates
            .iter()
            .map(|date| *day_buckets.get(date).unwrap_or(&0))
            .collect();
        values.sort_unstable();
        let max_duration = *values.last().unwrap_or(&0);
        let get_color = |v: u32| -> usize {
            if v < 60 {
                0
            } else {
                let ratio = v as f32 / max_duration as f32;
                if ratio >= ranges[0] as f32 / 100.0 {
                    4
                } else if ratio >= ranges[1] as f32 / 100.0 {
                    3
                } else if ratio >= ranges[2] as f32 / 100.0 {
                    2
                } else {
                    1
                }
            }
        };

        // Select palette based on theme param (default: dark)
        let palette = match params.theme.as_str() {
            "light" => &PALETTE_GITHUB_LIGHT,
            "catppuccin_light" => &PALLETE_CATPUCCIN_FRAPPE,
            "catppuccin_dark" => &PALLETE_CATPUCCIN_MOCHA,
            _ => &GITHUB_PALETTE_DARK,
        };

        for (i, date) in all_dates.iter().enumerate() {
            let seconds = *day_buckets.get(date).unwrap_or(&0);
            let col = i / DEFAULT_ROWS;
            let row = i % DEFAULT_ROWS;
            let x = (col * params.cell_size + params.padding * (col + 1)) as i32;
            let y = (row * params.cell_size + params.padding * (row + 1)) as i32;
            let w = params.cell_size as i32;
            let h = params.cell_size as i32;
            let color = palette[get_color(seconds)];
            let color_str = format!("#{:02x}{:02x}{:02x}", color.0, color.1, color.2);

            // Add tooltip
            let date_str = date.format("%B %-d").to_string();
            let day = date.day();
            let suffix = match day % 10 {
                1 if day != 11 => "st",
                2 if day != 12 => "nd",
                3 if day != 13 => "rd",
                _ => "th",
            };
            let label = if seconds > 0 {
                format!("{} on {}{}", human_time(seconds), date_str, suffix)
            } else {
                format!("No activity on {}{}", date_str, suffix)
            };
            tooltips.push(label.clone());

            let rect = Rectangle::new()
                .set("title", label.clone())
                .set("x", x)
                .set("y", y)
                .set("width", w)
                .set("height", h)
                .set("fill", color_str)
                .set("rx", radius)
                .set("ry", radius);

            let title = Title::new(&label);
            let rect_with_title = rect.add(title);
            document = document.add(rect_with_title);
        }

        svg_buf = embed_page(&document.to_string(), params.standalone);
    }

    {
        let mut response_cache = match state.response_cache.lock() {
            Ok(cache) => cache,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response(),
        };
        enforce_cache_limit(&mut response_cache, MAX_RESPONSE_CACHE_ENTRIES);
        response_cache.insert(params, (svg_buf.clone(), now));
    }

    (StatusCode::OK, build_headers(content_type), svg_buf).into_response()
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let state = AppState {
        response_cache: Arc::new(Mutex::new(HashMap::new())),
        request_cache: Arc::new(Mutex::new(HashMap::new())),
    };

    // Build application with a route
    let app = Router::new()
        // `GET /` goes to `make_heatmap_svg` with query params
        .route("/", get(make_heatmap_svg))
        .layer(CatchPanicLayer::new())
        .with_state(state);

    // Run app with hyper, listening globally on port 8282
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:8282").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to port 8282: {}", e);
            return;
        }
    };
    println!("Listening on http://localhost:8282");
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", e);
    }
}
