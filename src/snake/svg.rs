use std::fmt::Write;

use super::types::{Grid, SNAKE_LEN, Snake};
use crate::palette::Palette;
use crate::utils::format_color;

pub struct DrawOptions {
    pub size_cell: f32,
    pub size_dot: f32,
    pub size_border_radius: f32,
    pub step_duration_ms: u64,
    pub color_snake: String,
}

impl Default for DrawOptions {
    fn default() -> Self {
        Self {
            size_cell: 14.0,
            size_dot: 10.5,
            size_border_radius: 2.0,
            step_duration_ms: 120,
            color_snake: "#7c3aed".to_string(),
        }
    }
}

struct LivingCell {
    x: i32,
    y: i32,
    color: u8,
    t: Option<f64>,
}

// Restricts CSS color values to hex or named colors to prevent style/SVG injection.
#[must_use]
pub(crate) fn sanitize_css_color(color: &str) -> &str {
    let trimmed = color.trim();
    let valid_hex = trimmed.strip_prefix('#').is_some_and(|hex| {
        matches!(hex.len(), 3 | 4 | 6 | 8) && hex.bytes().all(|b| b.is_ascii_hexdigit())
    });
    let valid_name = !trimmed.is_empty()
        && trimmed.len() <= 32
        && trimmed.bytes().all(|b| b.is_ascii_alphabetic());

    if valid_hex || valid_name {
        trimmed
    } else {
        "#7c3aed"
    }
}

fn write_snake_part_keyframes(
    svg: &mut String,
    part_idx: usize,
    chain: &[Snake],
    total_steps: usize,
    size_cell: f32,
) {
    let _ = write!(svg, "@keyframes s{part_idx}{{");
    if let Some(first) = chain.first() {
        let p = first.cells[part_idx];
        let _ = write!(
            svg,
            "0.00%{{transform:translate({:.1}px,{:.1}px)}}",
            p.x as f32 * size_cell,
            p.y as f32 * size_cell,
        );
    }

    if chain.len() > 2 {
        for step in 1..(chain.len() - 1) {
            let prev = chain[step - 1].cells[part_idx];
            let curr = chain[step].cells[part_idx];
            let next = chain[step + 1].cells[part_idx];
            let is_colinear = prev.x + next.x == 2 * curr.x && prev.y + next.y == 2 * curr.y;
            if !is_colinear {
                let pct = (step as f64 / total_steps as f64) * 100.0;
                let _ = write!(
                    svg,
                    "{pct:.2}%{{transform:translate({:.1}px,{:.1}px)}}",
                    curr.x as f32 * size_cell,
                    curr.y as f32 * size_cell,
                );
            }
        }
    }

    if chain.len() > 1
        && let Some(last) = chain.last()
    {
        let p = last.cells[part_idx];
        let pct = ((chain.len() - 1) as f64 / total_steps as f64) * 100.0;
        let _ = write!(
            svg,
            "{pct:.2}%{{transform:translate({:.1}px,{:.1}px)}}",
            p.x as f32 * size_cell,
            p.y as f32 * size_cell,
        );
    }
    svg.push('}');
}

#[must_use]
pub fn create_snake_svg(
    grid0: &Grid,
    chain: &[Snake],
    palette: &Palette,
    dark_palette: Option<&Palette>,
    options: &DrawOptions,
) -> String {
    let size_cell = options.size_cell.max(1.0);
    let size_dot = options.size_dot.clamp(0.5, size_cell * 2.0);
    let step_duration_ms = options.step_duration_ms.max(1);
    let color_snake = sanitize_css_color(&options.color_snake);

    let mut living_cells = Vec::with_capacity(grid0.width * grid0.height);
    for x in 0..grid0.width {
        for y in 0..grid0.height {
            living_cells.push(LivingCell {
                x: x as i32,
                y: y as i32,
                color: grid0.get_color(x, y),
                t: None,
            });
        }
    }

    let total_steps = if chain.len() > 1 && chain.last() == chain.first() {
        chain.len() - 1
    } else {
        chain.len().max(1)
    };

    let mut stack_events: Vec<(u8, f64)> = Vec::with_capacity(living_cells.len());
    for (i, snake) in chain.iter().enumerate() {
        let hx = snake.head_x();
        let hy = snake.head_y();
        if grid0.is_inside(hx, hy) {
            let idx = grid0.get_index(hx as usize, hy as usize);
            let cell = &mut living_cells[idx];
            if cell.color > 0 && cell.t.is_none() {
                let t = i as f64 / total_steps as f64;
                cell.t = Some(t);
                stack_events.push((cell.color, t));
            }
        }
    }

    let duration_ms = step_duration_ms * total_steps as u64;
    let width = (grid0.width as f32 + 2.0) * size_cell;
    let height = (grid0.height as f32 + 5.0) * size_cell;

    let view_box_min_x = -size_cell;
    let view_box_min_y = -size_cell * 2.0;

    let mut svg = String::with_capacity(32 * 1024);

    let _ = write!(
        svg,
        r#"<svg viewBox="{view_box_min_x:.1} {view_box_min_y:.1} {width:.1} {height:.1}" width="{width:.1}" height="{height:.1}" xmlns="http://www.w3.org/2000/svg">"#
    );

    svg.push_str("<desc>Generated with https://github.com/ImShyMike/hackatime-heatmap</desc>");
    svg.push_str("<style>");

    let p_colors = palette.all_colors();
    let empty_color_str = format_color(p_colors[0].0, p_colors[0].1, p_colors[0].2);
    let border_color_str = "rgba(0,0,0,0.06)";

    let _ = write!(
        svg,
        ":root{{--cb:{border_color_str};--cs:{color_snake};--ce:{empty_color_str};--c1:{};--c2:{};--c3:{};--c4:{};}}",
        format_color(p_colors[1].0, p_colors[1].1, p_colors[1].2),
        format_color(p_colors[2].0, p_colors[2].1, p_colors[2].2),
        format_color(p_colors[3].0, p_colors[3].1, p_colors[3].2),
        format_color(p_colors[4].0, p_colors[4].1, p_colors[4].2),
    );

    if let Some(dp) = dark_palette {
        let dp_colors = dp.all_colors();
        let _ = write!(
            svg,
            "@media(prefers-color-scheme:dark){{:root{{--cb:rgba(255,255,255,0.06);--cs:{color_snake};--ce:{};--c1:{};--c2:{};--c3:{};--c4:{};}}}}",
            format_color(dp_colors[0].0, dp_colors[0].1, dp_colors[0].2),
            format_color(dp_colors[1].0, dp_colors[1].1, dp_colors[1].2),
            format_color(dp_colors[2].0, dp_colors[2].1, dp_colors[2].2),
            format_color(dp_colors[3].0, dp_colors[3].1, dp_colors[3].2),
            format_color(dp_colors[4].0, dp_colors[4].1, dp_colors[4].2),
        );
    }

    let _ = write!(
        svg,
        ".c{{shape-rendering:geometricPrecision;fill:var(--ce);stroke-width:1px;stroke:var(--cb);animation:none {duration_ms}ms linear infinite;width:{size_dot:.1}px;height:{size_dot:.1}px}}"
    );
    let _ = write!(
        svg,
        ".s{{shape-rendering:geometricPrecision;fill:var(--cs);animation:none linear {duration_ms}ms infinite}}"
    );
    let _ = write!(
        svg,
        ".u{{transform-origin:0 0;transform:scale(0,1);animation:none linear {duration_ms}ms infinite}}"
    );

    let mut grid_rects = String::with_capacity(16 * 1024);
    let dot_margin = (size_cell - size_dot) / 2.0;

    let mut eaten_idx = 0usize;
    for cell in &living_cells {
        let rect_x = cell.x as f32 * size_cell + dot_margin;
        let rect_y = cell.y as f32 * size_cell + dot_margin;
        if let Some(t) = cell.t {
            let t_before = ((t - 0.0001) * 100.0).clamp(0.0, 99.99);
            let t_after = ((t + 0.0001) * 100.0).clamp(t_before + 0.01, 100.0);
            let c = cell.color;

            let _ = write!(
                svg,
                "@keyframes c{eaten_idx}{{0%,{t_before:.2}%{{fill:var(--c{c})}}{t_after:.2}%,100%{{fill:var(--ce)}}}}.c.c{eaten_idx}{{fill:var(--c{c});animation-name:c{eaten_idx}}}"
            );
            let _ = write!(
                grid_rects,
                r#"<rect class="c c{eaten_idx}" x="{rect_x:.1}" y="{rect_y:.1}" rx="{:.1}" ry="{:.1}"/>"#,
                options.size_border_radius, options.size_border_radius,
            );
            eaten_idx += 1;
        } else {
            let _ = write!(
                grid_rects,
                r#"<rect class="c" x="{rect_x:.1}" y="{rect_y:.1}" rx="{:.1}" ry="{:.1}"/>"#,
                options.size_border_radius, options.size_border_radius,
            );
        }
    }

    let mut snake_rects = String::with_capacity(4096);
    let d_min = size_dot * 0.8;
    let d_max = size_cell * 0.9;
    let i_max = SNAKE_LEN as f32;

    for part_idx in 0..SNAKE_LEN {
        let interp = (1.0 - (part_idx as f32 / i_max)).powi(2);
        let seg_size = (1.0 - interp) * d_min + interp * d_max;
        let part_margin = (size_cell - seg_size) / 2.0;
        let radius = (4.0 * seg_size / size_dot).min(4.5);

        let _ = write!(
            snake_rects,
            r#"<rect class="s s{part_idx}" x="{part_margin:.1}" y="{part_margin:.1}" width="{seg_size:.1}" height="{seg_size:.1}" rx="{radius:.1}" ry="{radius:.1}"/>"#
        );

        write_snake_part_keyframes(&mut svg, part_idx, chain, total_steps, size_cell);

        if let Some(first) = chain.first() {
            let initial_p = first.cells[part_idx];
            let _ = write!(
                svg,
                ".s.s{part_idx}{{transform:translate({:.1}px,{:.1}px);animation-name:s{part_idx}}}",
                initial_p.x as f32 * size_cell,
                initial_p.y as f32 * size_cell,
            );
        }
    }

    let mut stack_rects = String::with_capacity(4096);
    if !stack_events.is_empty() {
        let stack_width = grid0.width as f32 * size_cell;
        let stack_y = (grid0.height as f32 + 2.0) * size_cell;
        let m_step = stack_width / stack_events.len() as f32;

        let mut block_runs: Vec<(u8, usize, usize)> = Vec::new();
        for (idx, &(color, _)) in stack_events.iter().enumerate() {
            if let Some(last) = block_runs.last_mut()
                && last.0 == color
            {
                last.2 += 1;
            } else {
                block_runs.push((color, idx, 1));
            }
        }

        let mut nx = 0.0f32;
        for (block_idx, (color, start, len)) in block_runs.iter().enumerate() {
            let ts = &stack_events[*start..(*start + *len)];
            let x = nx * m_step;
            let block_w = *len as f32 * m_step + 0.6;
            nx += *len as f32;

            let _ = write!(
                stack_rects,
                r#"<rect class="u u{block_idx}" height="{size_dot:.1}" width="{block_w:.1}" x="{x:.1}" y="{stack_y:.1}"/>"#
            );

            let _ = write!(svg, "@keyframes u{block_idx}{{");
            let len_f = *len as f32;
            for (i, &(_, t)) in ts.iter().enumerate() {
                let s_before = (i as f32) / len_f;
                let s_after = (i as f32 + 1.0) / len_f;
                let t_before = ((t - 0.0001) * 100.0).clamp(0.0, 99.99);
                let t_after = ((t + 0.0001) * 100.0).clamp(t_before + 0.01, 100.0);
                let _ = write!(
                    svg,
                    "{t_before:.2}%{{transform:scale({s_before:.3},1)}}{t_after:.2}%{{transform:scale({s_after:.3},1)}}"
                );
            }
            svg.push_str("100%{transform:scale(1,1)}}");

            let _ = write!(
                svg,
                ".u.u{block_idx}{{fill:var(--c{color});animation-name:u{block_idx};transform-origin:{x:.1}px 0}}"
            );
        }
    }

    svg.push_str("</style>");
    svg.push_str(&grid_rects);
    svg.push_str(&stack_rects);
    svg.push_str(&snake_rects);
    svg.push_str("</svg>");

    svg
}
