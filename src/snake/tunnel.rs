use std::cmp::Ordering;
use std::collections::BinaryHeap;

use super::outside::Outside;
use super::point::{AROUND4, Point};
use super::search::SearchScratch;
use super::types::{Grid, SNAKE_LEN, Snake};

const TARGET_COLOR_COST: i32 = 1000;

#[derive(Clone, Copy)]
struct EscapeNode {
    snake: Snake,
    parent: Option<usize>,
}

#[derive(Eq, PartialEq)]
struct HeapEntry {
    w: i32,
    index: usize,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.w.cmp(&self.w)
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TunnelPriority {
    pub less: u32,
    pub color: u32,
}

impl Ord for TunnelPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.color == 0, other.color == 0) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => {
                (u64::from(self.less) * u64::from(other.color))
                    .cmp(&(u64::from(other.less) * u64::from(self.color)))
            }
        }
    }
}

impl PartialOrd for TunnelPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_snake_escape_path(
    grid: &Grid,
    outside: &Outside,
    snake0: Snake,
    color: u8,
    scratch: &mut SearchScratch,
) -> Option<Vec<Point>> {
    scratch.begin_search();
    scratch.try_visit(snake0, 0);

    let mut nodes: Vec<EscapeNode> = Vec::with_capacity(128);
    let mut open_set = BinaryHeap::with_capacity(128);

    nodes.push(EscapeNode {
        snake: snake0,
        parent: None,
    });
    open_set.push(HeapEntry { w: 0, index: 0 });

    while let Some(HeapEntry { w, index }) = open_set.pop() {
        let current_snake = nodes[index].snake;
        // Skip stale entries superseded by a cheaper path.
        if scratch.best_cost_of(current_snake) != Some(w) {
            continue;
        }

        let head = current_snake.head();
        if outside.is_outside(head.x, head.y) {
            let mut path = Vec::new();
            let mut curr = Some(index);
            while let Some(idx) = curr {
                path.push(nodes[idx].snake.head());
                curr = nodes[idx].parent;
            }
            path.reverse();
            return Some(path);
        }

        for a in AROUND4 {
            let nx = head.x + a.x;
            let ny = head.y + a.y;
            let c = grid.get_color_safe(nx, ny);

            if c <= color && !current_snake.will_self_collide(a.x, a.y) {
                let nsnake = current_snake.next(a.x, a.y);
                let next_w = w + 1 + if c == color { TARGET_COLOR_COST } else { 0 };
                if scratch.try_visit(nsnake, next_w) {
                    let next_idx = nodes.len();
                    nodes.push(EscapeNode {
                        snake: nsnake,
                        parent: Some(index),
                    });
                    open_set.push(HeapEntry {
                        w: next_w,
                        index: next_idx,
                    });
                }
            }
        }
    }

    None
}

pub(crate) fn get_best_tunnel_with_scratch(
    grid: &Grid,
    outside: &Outside,
    x: i32,
    y: i32,
    color: u8,
    scratch: &mut SearchScratch,
) -> Option<Vec<Point>> {
    let target = Point::new(x, y);
    let snake0 = Snake::from_cells([target; SNAKE_LEN]);

    let mut one = get_snake_escape_path(grid, outside, snake0, color, scratch)?;
    let last = *one.last()?;
    let snake_i = Snake::from_cells(std::array::from_fn(|i| one.get(i).copied().unwrap_or(last)));

    let mut grid_i = scratch.temp_grid.clone();
    grid_i.data.copy_from_slice(&grid.data);
    for p in &one {
        grid_i.set_empty_safe(p.x, p.y);
    }

    let two = get_snake_escape_path(&grid_i, outside, snake_i, color, scratch)?;

    if !one.is_empty() {
        one.remove(0);
    }
    one.reverse();
    one.extend(two);

    trim_tunnel_start(grid, &mut one);
    trim_tunnel_end(grid, &mut one);

    if one.is_empty() { None } else { Some(one) }
}

#[must_use]
pub fn get_tunnel_path(snake0: Snake, tunnel: &[Point]) -> Vec<Snake> {
    let mut chain = Vec::with_capacity(tunnel.len().saturating_sub(1));
    let mut snake = snake0;

    for point in tunnel.iter().skip(1) {
        let dx = point.x - snake.head_x();
        let dy = point.y - snake.head_y();
        snake = snake.next(dx, dy);
        chain.push(snake);
    }

    chain
}

pub fn trim_tunnel_start(grid: &Grid, tunnel: &mut Vec<Point>) {
    let first_non_empty = tunnel
        .iter()
        .position(|p| !grid.is_empty_safe(p.x, p.y))
        .unwrap_or(tunnel.len());
    if first_non_empty > 0 {
        tunnel.drain(0..first_non_empty);
    }
}

pub fn trim_tunnel_end(grid: &Grid, tunnel: &mut Vec<Point>) {
    while let Some(&p_val) = tunnel.last() {
        let last_idx = tunnel.len() - 1;
        let is_empty = grid.is_empty_safe(p_val.x, p_val.y);
        let is_duplicate = tunnel[..last_idx].contains(&p_val);

        if is_empty || is_duplicate {
            tunnel.pop();
        } else {
            break;
        }
    }
}

pub(crate) fn get_priority_with_scratch(
    grid: &Grid,
    color: u8,
    tunnel: &[Point],
    scratch: &mut SearchScratch,
) -> TunnelPriority {
    let mut n_color = 0u32;
    let mut n_less = 0u32;
    scratch.begin_grid_mark();

    for p in tunnel {
        let c = grid.get_color_safe(p.x, p.y);
        if c > 0 && scratch.mark_grid(p.x, p.y) {
            if c == color {
                n_color += 1;
            } else if c < color {
                n_less += u32::from(color - c);
            }
        }
    }

    TunnelPriority {
        less: n_less,
        color: n_color,
    }
}
