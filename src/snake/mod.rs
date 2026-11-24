mod outside;
mod pathfinding;
mod point;
mod search;
mod solver;
mod svg;
mod tunnel;
mod types;

use chrono::NaiveDate;
use serde::Deserialize;

use self::solver::get_best_route;
use self::svg::{DrawOptions, create_snake_svg, sanitize_css_color};
use self::types::{Grid, default_snake};
use crate::DayBuckets;
use crate::palette::{PALETTES, get_palette};

const DEFAULT_ROWS: usize = 7;
const DEFAULT_COLS: usize = 53;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(default)]
pub struct SnakeParams {
    pub id: Option<String>,
    pub timezone: String,
    pub theme: String,
    pub ranges: String,
    pub cell_size: Option<u32>,
    pub dot_size: Option<u32>,
    pub speed: Option<u64>,
    pub snake_color: Option<String>,
    pub year: Option<String>,
}

impl Default for SnakeParams {
    fn default() -> Self {
        Self {
            id: None,
            timezone: "Europe/London".to_string(),
            theme: "dark".to_string(),
            ranges: "70,30,10".to_string(),
            cell_size: None,
            dot_size: None,
            speed: None,
            snake_color: None,
            year: None,
        }
    }
}

#[must_use]
pub fn calculate_level(value: u32, max: u32, ranges: &[u32]) -> u8 {
    if value < 60 || max == 0 {
        return 0;
    }

    let [r4, r3, r2, ..] = ranges else {
        return 1;
    };

    let ratio = value as f32 / max as f32;
    if ratio >= *r4 as f32 / 100.0 {
        4
    } else if ratio >= *r3 as f32 / 100.0 {
        3
    } else if ratio >= *r2 as f32 / 100.0 {
        2
    } else {
        1
    }
}

#[must_use]
pub fn generate_snake_animation_svg(
    all_dates: &[NaiveDate],
    day_buckets: &DayBuckets,
    ranges: &[u32],
    params: &SnakeParams,
) -> String {
    let max_duration = all_dates
        .iter()
        .map(|date| *day_buckets.get(date).unwrap_or(&0))
        .max()
        .unwrap_or(0);

    let mut grid = Grid::new(DEFAULT_COLS, DEFAULT_ROWS);

    for (i, date) in all_dates.iter().enumerate() {
        let col = i / DEFAULT_ROWS;
        let row = i % DEFAULT_ROWS;
        if col < DEFAULT_COLS && row < DEFAULT_ROWS {
            let seconds = *day_buckets.get(date).unwrap_or(&0);
            let level = calculate_level(seconds, max_duration, ranges);
            grid.set_color(col, row, level);
        }
    }

    let initial_snake = default_snake();
    let mut solve_grid = grid.clone();
    let chain = get_best_route(&mut solve_grid, initial_snake);

    let (palette, dark_palette) = match params.theme.as_str() {
        "auto" => (
            get_palette(PALETTES, "light"),
            Some(get_palette(PALETTES, "dark")),
        ),
        "catppuccin" => (
            get_palette(PALETTES, "catppuccin_light"),
            Some(get_palette(PALETTES, "catppuccin_dark")),
        ),
        _ => (get_palette(PALETTES, &params.theme), None),
    };

    let cell_size = (params.cell_size.unwrap_or(14) as f32).clamp(1.0, 100.0);
    let dot_size = params
        .dot_size
        .map_or(cell_size * 0.75, |d| (d as f32).clamp(0.5, cell_size * 2.0));
    let step_duration_ms = params.speed.unwrap_or(120).clamp(20, 1000);

    let color_snake = params
        .snake_color
        .as_deref()
        .map(sanitize_css_color)
        .unwrap_or("#7c3aed")
        .to_string();

    let draw_options = DrawOptions {
        size_cell: cell_size,
        size_dot: dot_size,
        size_border_radius: 2.0,
        step_duration_ms,
        color_snake,
    };

    create_snake_svg(&grid, &chain, palette, dark_palette, &draw_options)
}
