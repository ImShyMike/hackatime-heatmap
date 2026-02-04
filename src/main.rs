mod palette;
mod time;
mod utils;

use axum::Router;
use axum::extract::{OriginalUri, Query, State};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;

use chrono::Datelike;
use chrono_tz::Tz;

use tower_http::compression::CompressionLayer;
use tower_http::decompression::DecompressionLayer;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::timeout::TimeoutLayer;

use std::collections::HashMap;
use std::time::{Duration, Instant};

use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::CorsLayer;

use metrics::{counter, histogram};

use serde::Deserialize;

use svg::Document;
use svg::node::element::{Group, Rectangle, Text, Title};

use moka::sync::Cache;

use crate::palette::{PALETTES, get_palette};
use crate::time::{create_timezone_timestamp, generate_date_range, process_span_into_buckets};
use crate::utils::{build_headers, format_cell_label, validate_ranges};

const DEFAULT_ROWS: usize = 7;
const DEFAULT_COLS: usize = 53;
const MONTH_LABEL_HEIGHT: usize = 15;
const WEEKDAY_LABEL_WIDTH: usize = 28;
const LEGEND_HEIGHT: usize = 20;

const MONTH_LABELS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
const WEEKDAY_LABELS: [(usize, &str); 3] = [(1, "Mon"), (3, "Wed"), (5, "Fri")];
const RESPONSE_CACHE_DURATION_SECONDS: u64 = 60 * 15; // (15 minutes)
const MAX_RESPONSE_CACHE_ENTRIES: u64 = 200;
const REQUEST_CACHE_DURATION_SECONDS: u64 = 60 * 15; // (15 minutes)
const MAX_REQUEST_CACHE_ENTRIES: u64 = 25;
const CACHE_HEADER: HeaderValue = HeaderValue::from_static("public, max-age=900"); // 15 minutes
const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

const TEMPLATE: &str = include_str!("template.html");

#[derive(Clone)]
struct AppState {
    response_cache: Cache<SvgParams, String>,
    request_cache: Cache<String, RequestData>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(default)]
struct SvgParams {
    id: Option<String>,
    timezone: String,
    cell_size: usize,
    padding: usize,
    rounding: u8,
    theme: String,
    ranges: String,
    standalone: bool,
    labels: bool,
    year: Option<String>,
}

impl Default for SvgParams {
    fn default() -> Self {
        Self {
            id: None,
            timezone: "Europe/London".to_string(),
            cell_size: 10,
            padding: 3,
            rounding: 20,
            theme: "dark".to_string(),
            ranges: "70,30,10".to_string(),
            standalone: false,
            labels: false,
            year: None,
        }
    }
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

fn embed_page(svg: &str, standalone: bool) -> String {
    if !standalone {
        return svg.to_string();
    }

    TEMPLATE.replace("{{SVG_CONTENT}}", svg)
}

async fn fetch_user_spans(
    id: &str,
    cache: &Cache<String, RequestData>,
) -> Result<Vec<Span>, String> {
    if let Some(cached) = cache.get(id) {
        counter!("heatmap_cache_hits_total", "cache" => "request").increment(1);
        return Ok(cached.spans.clone());
    }
    counter!("heatmap_cache_misses_total", "cache" => "request").increment(1);

    let fetch_start = Instant::now();
    let url = format!(
        "https://hackatime.hackclub.com/api/v1/users/{}/heartbeats/spans",
        id
    );
    let resp = reqwest::get(&url).await.map_err(|err| {
        tracing::error!("Error fetching data: {:?}", err);
        counter!("heatmap_upstream_errors_total", "type" => "fetch").increment(1);
        "Failed to fetch data".to_string()
    })?;

    let json_resp = resp.json::<RequestData>().await.map_err(|err| {
        tracing::error!("Error parsing JSON: {:?}", err);
        counter!("heatmap_upstream_errors_total", "type" => "parse").increment(1);
        "Failed to parse response".to_string()
    })?;

    histogram!("heatmap_upstream_request_duration_seconds")
        .record(fetch_start.elapsed().as_secs_f64());
    cache.insert(id.to_string(), json_resp.clone());
    Ok(json_resp.spans)
}

fn create_svg_document(
    all_dates: &[chrono::NaiveDate],
    day_buckets: &HashMap<chrono::NaiveDate, u32>,
    ranges: &[u32],
    params: &SvgParams,
) -> String {
    let cell_size = params.cell_size;
    let padding = params.padding;
    let show_labels = params.labels;

    let weekday_width = if show_labels { WEEKDAY_LABEL_WIDTH } else { 0 };
    let month_height = if show_labels { MONTH_LABEL_HEIGHT } else { 0 };
    let legend_height = if show_labels { LEGEND_HEIGHT } else { 0 };

    let grid_width = DEFAULT_COLS * (cell_size + padding);
    let grid_height = DEFAULT_ROWS * (cell_size + padding);
    let total_width = weekday_width + grid_width + if show_labels { 3 } else { 0 };
    let total_height = month_height + grid_height + legend_height;

    let mut document = Document::new()
        .set("width", total_width)
        .set("height", total_height)
        .set("viewBox", format!("0 0 {} {}", total_width, total_height));

    let mut values: Vec<u32> = all_dates
        .iter()
        .map(|date| *day_buckets.get(date).unwrap_or(&0))
        .collect();
    values.sort_unstable();
    let max_duration = *values.last().unwrap_or(&0);

    let selected_palette = get_palette(PALETTES, &params.theme);
    let text_color = selected_palette.text_color();
    let text_color_str = format!(
        "#{:02x}{:02x}{:02x}",
        text_color.0, text_color.1, text_color.2
    );

    if show_labels {
        let month_group = create_month_labels(
            all_dates,
            &text_color_str,
            cell_size,
            padding,
            weekday_width,
        );
        document = document.add(month_group);

        let weekday_group =
            create_weekday_labels(&text_color_str, cell_size, padding, month_height);
        document = document.add(weekday_group);
    }

    for (i, date) in all_dates.iter().enumerate() {
        let rect = create_cell_rectangle(
            i,
            date,
            day_buckets,
            max_duration,
            ranges,
            selected_palette,
            params,
            weekday_width,
            month_height,
        );
        document = document.add(rect);
    }

    if show_labels {
        let legend_group = create_legend(
            selected_palette,
            &text_color_str,
            cell_size,
            padding,
            weekday_width,
            month_height,
        );
        document = document.add(legend_group);
    }

    document.to_string()
}

fn create_month_labels(
    all_dates: &[chrono::NaiveDate],
    text_color: &str,
    cell_size: usize,
    padding: usize,
    weekday_width: usize,
) -> Group {
    use chrono::Datelike;

    let mut group = Group::new();
    let mut last_month: Option<u32> = None;

    for (i, date) in all_dates.iter().enumerate() {
        let col = i / DEFAULT_ROWS;
        let row = i % DEFAULT_ROWS;

        if row == 0 {
            let month = date.month();
            if last_month != Some(month) {
                last_month = Some(month);
                let x = weekday_width + col * (cell_size + padding);
                let label = MONTH_LABELS[(month - 1) as usize];
                let text = Text::new(label)
                    .set("x", x)
                    .set("y", 10)
                    .set("fill", text_color)
                    .set("font-size", "10px")
                    .set("font-family", "-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', Helvetica, Arial, sans-serif");
                group = group.add(text);
            }
        }
    }
    group
}

fn create_weekday_labels(
    text_color: &str,
    cell_size: usize,
    padding: usize,
    month_height: usize,
) -> Group {
    let mut group = Group::new();

    for (row, label) in WEEKDAY_LABELS {
        let y = month_height + row * (cell_size + padding) + cell_size;
        let text = Text::new(label)
            .set("x", 0)
            .set("y", y)
            .set("fill", text_color)
            .set("font-size", "10px")
            .set("font-family", "-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', Helvetica, Arial, sans-serif");
        group = group.add(text);
    }
    group
}

fn create_legend(
    palette: &palette::Palette,
    text_color: &str,
    cell_size: usize,
    padding: usize,
    weekday_width: usize,
    month_height: usize,
) -> Group {
    let mut group = Group::new();
    let legend_y = month_height + DEFAULT_ROWS * (cell_size + padding) + 8;
    let legend_start_x = weekday_width + DEFAULT_COLS * (cell_size + padding) - 120;

    let less_text = Text::new("Less")
        .set("x", legend_start_x)
        .set("y", legend_y + 9)
        .set("fill", text_color)
        .set("font-size", "10px")
        .set("font-family", "-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', Helvetica, Arial, sans-serif");
    group = group.add(less_text);

    let colors = palette.all_colors();
    let box_start_x = legend_start_x + 28;
    for (i, color) in colors.iter().enumerate() {
        let color_str = format!("#{:02x}{:02x}{:02x}", color.0, color.1, color.2);
        let rect = Rectangle::new()
            .set("x", box_start_x + i * (cell_size + 2))
            .set("y", legend_y)
            .set("width", cell_size)
            .set("height", cell_size)
            .set("fill", color_str)
            .set("rx", 2)
            .set("ry", 2);
        group = group.add(rect);
    }

    let more_text = Text::new("More")
        .set("x", box_start_x + 5 * (cell_size + 2) + 2)
        .set("y", legend_y + 9)
        .set("fill", text_color)
        .set("font-size", "10px")
        .set("font-family", "-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', Helvetica, Arial, sans-serif");
    group = group.add(more_text);

    group
}

#[allow(clippy::too_many_arguments)]
fn create_cell_rectangle(
    index: usize,
    date: &chrono::NaiveDate,
    day_buckets: &HashMap<chrono::NaiveDate, u32>,
    max_duration: u32,
    ranges: &[u32],
    palette: &palette::Palette,
    params: &SvgParams,
    weekday_width: usize,
    month_height: usize,
) -> Rectangle {
    let cell_size = params.cell_size;
    let padding = params.padding;
    let radius = (params.rounding.min(100) as f32 / 200.0) * cell_size as f32;

    let seconds = *day_buckets.get(date).unwrap_or(&0);
    let col = index / DEFAULT_ROWS;
    let row = index % DEFAULT_ROWS;
    let x = weekday_width + col * (cell_size + padding);
    let y = month_height + row * (cell_size + padding);

    let color = palette.calculate_color(seconds, max_duration, ranges);
    let color_str = format!("#{:02x}{:02x}{:02x}", color.0, color.1, color.2);

    let label = format_cell_label(date, seconds);

    let rect = Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", cell_size)
        .set("height", cell_size)
        .set("fill", color_str)
        .set("rx", radius)
        .set("ry", radius);

    let title = Title::new(&label);
    rect.add(title)
}

async fn make_heatmap_svg(
    State(state): State<AppState>,
    Query(params): Query<SvgParams>,
    OriginalUri(uri): OriginalUri,
) -> Response {
    let request_start = Instant::now();
    counter!("heatmap_http_requests_total").increment(1);

    tracing::info!("Request: {}", uri);

    let id = match &params.id {
        Some(id) => id.clone(),
        None => {
            counter!("heatmap_http_requests_errors_total", "error" => "missing_id").increment(1);
            histogram!("heatmap_http_request_duration_seconds", "status" => "400")
                .record(request_start.elapsed().as_secs_f64());
            return (StatusCode::BAD_REQUEST, "Missing required parameter: id").into_response();
        }
    };

    counter!("heatmap_user_requests_total", "user_id" => id.clone()).increment(1);

    if let Some(response) = state.response_cache.get(&params) {
        counter!("heatmap_cache_hits_total", "cache" => "response").increment(1);
        let content_type = if params.standalone {
            "text/html"
        } else {
            "image/svg+xml"
        };
        histogram!("heatmap_http_request_duration_seconds", "status" => "200")
            .record(request_start.elapsed().as_secs_f64());
        return (StatusCode::OK, build_headers(content_type), response).into_response();
    }

    counter!("heatmap_cache_misses_total", "cache" => "response").increment(1);

    let ranges = match validate_ranges(&params.ranges) {
        Ok(r) => r,
        Err(err) => {
            counter!("heatmap_http_requests_errors_total", "error" => "invalid_ranges")
                .increment(1);
            histogram!("heatmap_http_request_duration_seconds", "status" => "400")
                .record(request_start.elapsed().as_secs_f64());
            return (StatusCode::BAD_REQUEST, err).into_response();
        }
    };

    let content_type = if params.standalone {
        "text/html"
    } else {
        "image/svg+xml"
    };

    let tz: Tz = match params.timezone.parse() {
        Ok(tz) => tz,
        Err(_) => {
            tracing::warn!("Unsupported timezone: {}", params.timezone);
            counter!("heatmap_http_requests_errors_total", "error" => "invalid_timezone")
                .increment(1);
            histogram!("heatmap_http_request_duration_seconds", "status" => "400")
                .record(request_start.elapsed().as_secs_f64());
            return (StatusCode::BAD_REQUEST, "Unsupported timezone".to_string()).into_response();
        }
    };

    let now_utc = chrono::Utc::now();
    let now_local = now_utc.with_timezone(&tz);
    let today = now_local.date_naive();
    let current_year = today.year();

    let (start_date, end_date) = match &params.year {
        Some(year_str) => {
            let year = if year_str.eq_ignore_ascii_case("current") {
                current_year
            } else {
                match year_str.parse::<i32>() {
                    Ok(y) => y,
                    Err(_) => {
                        counter!("heatmap_http_requests_errors_total", "error" => "invalid_year")
                            .increment(1);
                        histogram!("heatmap_http_request_duration_seconds", "status" => "400")
                            .record(request_start.elapsed().as_secs_f64());
                        return (StatusCode::BAD_REQUEST, "Invalid year parameter").into_response();
                    }
                }
            };
            let jan_1 = chrono::NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
            let dec_31 = chrono::NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
            (jan_1, dec_31)
        }
        None => {
            let one_year_ago = (now_local - chrono::Duration::days(365)).date_naive();
            (one_year_ago, today)
        }
    };

    let spans = match fetch_user_spans(&id, &state.request_cache).await {
        Ok(s) => s,
        Err(err) => {
            counter!("heatmap_http_requests_errors_total", "error" => "upstream_failure")
                .increment(1);
            histogram!("heatmap_http_request_duration_seconds", "status" => "500")
                .record(request_start.elapsed().as_secs_f64());
            return (StatusCode::INTERNAL_SERVER_ERROR, err).into_response();
        }
    };

    let start_ts = match create_timezone_timestamp(&tz, &start_date, 0, 0, 0) {
        Ok(ts) => ts,
        Err(err) => {
            tracing::error!("Invalid start date: {}", err);
            counter!("heatmap_http_requests_errors_total", "error" => "invalid_start_date")
                .increment(1);
            histogram!("heatmap_http_request_duration_seconds", "status" => "500")
                .record(request_start.elapsed().as_secs_f64());
            return (StatusCode::INTERNAL_SERVER_ERROR, err).into_response();
        }
    };
    let end_ts = match create_timezone_timestamp(&tz, &end_date, 23, 59, 59) {
        Ok(ts) => ts,
        Err(err) => {
            tracing::error!("Invalid end date: {}", err);
            counter!("heatmap_http_requests_errors_total", "error" => "invalid_end_date")
                .increment(1);
            histogram!("heatmap_http_request_duration_seconds", "status" => "500")
                .record(request_start.elapsed().as_secs_f64());
            return (StatusCode::INTERNAL_SERVER_ERROR, err).into_response();
        }
    };

    let mut day_buckets: HashMap<chrono::NaiveDate, u32> = HashMap::new();
    for span in spans
        .iter()
        .filter(|span| span.end_time >= start_ts && span.start_time <= end_ts)
    {
        process_span_into_buckets(span, &tz, &mut day_buckets);
    }

    let all_dates = generate_date_range(start_date, end_date);
    let svg_content = create_svg_document(&all_dates, &day_buckets, &ranges, &params);
    let svg_buf = embed_page(&svg_content, params.standalone);

    state.response_cache.insert(params, svg_buf.clone());

    histogram!("heatmap_http_request_duration_seconds", "status" => "200")
        .record(request_start.elapsed().as_secs_f64());
    (StatusCode::OK, build_headers(content_type), svg_buf).into_response()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let metrics_enabled = std::env::var("METRICS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if metrics_enabled {
        let prometheus_builder = metrics_exporter_prometheus::PrometheusBuilder::new();
        prometheus_builder
            .with_http_listener(([0, 0, 0, 0], 9292))
            .install()
            .expect("Failed to install Prometheus exporter");

        tracing::info!("Prometheus metrics available at http://localhost:9292/metrics");
    }

    let state = AppState {
        response_cache: Cache::builder()
            .max_capacity(MAX_RESPONSE_CACHE_ENTRIES)
            .time_to_live(Duration::from_secs(RESPONSE_CACHE_DURATION_SECONDS))
            .build(),
        request_cache: Cache::builder()
            .max_capacity(MAX_REQUEST_CACHE_ENTRIES)
            .time_to_live(Duration::from_secs(REQUEST_CACHE_DURATION_SECONDS))
            .build(),
    };

    let app = Router::new()
        .route("/", get(make_heatmap_svg))
        .route("/health", get(|| async { "OK" }))
        .layer(CompressionLayer::new().gzip(true))
        .layer(DecompressionLayer::new().gzip(true))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            DEFAULT_REQUEST_TIMEOUT,
        ))
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(CatchPanicLayer::new())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:8282").await {
        Ok(listener) => listener,
        Err(e) => {
            tracing::error!("Failed to bind to port 8282: {}", e);
            return;
        }
    };
    tracing::info!("Listening on http://localhost:8282");
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {}", e);
    }
}
