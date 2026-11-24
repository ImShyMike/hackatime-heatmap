use std::cmp::Ordering;
use std::collections::BinaryHeap;

use super::point::{AROUND4, Point};
use super::search::SearchScratch;
use super::tunnel::get_tunnel_path;
use super::types::{Grid, SNAKE_LEN, Snake};

#[derive(Clone, Copy)]
struct AStarNode {
    snake: Snake,
    parent: Option<usize>,
}

#[derive(Eq, PartialEq)]
struct HeapEntry {
    f: i32,
    w: i32,
    index: usize,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f).then_with(|| other.w.cmp(&self.w))
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn unwind_path(nodes: &[AStarNode], end_index: usize, final_snake: Option<Snake>) -> Vec<Snake> {
    let mut path = Vec::new();
    if let Some(s) = final_snake {
        path.push(s);
    }

    let mut curr = Some(end_index);
    while let Some(idx) = curr {
        if nodes[idx].parent.is_some() {
            path.push(nodes[idx].snake);
        }
        curr = nodes[idx].parent;
    }
    path.reverse();
    path
}

fn can_follow_prefix(mut snake: Snake, next_points: &[Point]) -> bool {
    for &p in next_points.iter().take(SNAKE_LEN - 1) {
        let dx = p.x - snake.head_x();
        let dy = p.y - snake.head_y();
        if dx.abs() + dy.abs() != 1 || snake.will_self_collide(dx, dy) {
            return false;
        }
        snake = snake.next(dx, dy);
    }
    true
}

pub(crate) fn get_path_to_tunnel_start(
    grid: &Grid,
    snake0: Snake,
    target: Point,
    tunnel_tail: &[Point],
    scratch: &mut SearchScratch,
) -> Option<Vec<Snake>> {
    if snake0.head() == target && can_follow_prefix(snake0, tunnel_tail) {
        return Some(Vec::new());
    }

    scratch.begin_search();
    scratch.try_visit(snake0, 0);

    let mut nodes: Vec<AStarNode> = Vec::with_capacity(256);
    let mut open_set = BinaryHeap::with_capacity(256);

    let h0 = (snake0.head_x() - target.x).abs() + (snake0.head_y() - target.y).abs();
    nodes.push(AStarNode {
        snake: snake0,
        parent: None,
    });
    open_set.push(HeapEntry {
        f: h0,
        w: 0,
        index: 0,
    });

    while let Some(HeapEntry { w, index, .. }) = open_set.pop() {
        let current_snake = nodes[index].snake;
        if scratch.best_cost_of(current_snake) != Some(w) {
            continue;
        }

        let cx = current_snake.head_x();
        let cy = current_snake.head_y();

        for a in AROUND4 {
            let nx = cx + a.x;
            let ny = cy + a.y;

            if current_snake.will_self_collide(a.x, a.y) {
                continue;
            }

            if nx == target.x && ny == target.y {
                let final_snake = current_snake.next(a.x, a.y);
                // Ensure entry angle does not collide with the first tunnel steps.
                if can_follow_prefix(final_snake, tunnel_tail) {
                    return Some(unwind_path(&nodes, index, Some(final_snake)));
                }
                continue;
            }

            if grid.is_inside_large(2, nx, ny) && grid.is_empty_safe(nx, ny) {
                let nsnake = current_snake.next(a.x, a.y);
                let next_w = w + 1;
                if scratch.try_visit(nsnake, next_w) {
                    let h = (nx - target.x).abs() + (ny - target.y).abs();
                    let next_idx = nodes.len();
                    nodes.push(AStarNode {
                        snake: nsnake,
                        parent: Some(index),
                    });
                    open_set.push(HeapEntry {
                        f: next_w + h,
                        w: next_w,
                        index: next_idx,
                    });
                }
            }
        }
    }

    None
}

pub(crate) fn get_path_to_pose_with_scratch(
    snake0: Snake,
    target: Snake,
    grid: Option<&Grid>,
    scratch: &mut SearchScratch,
) -> Option<Vec<Snake>> {
    if snake0 == target {
        return Some(Vec::new());
    }

    let mut target_cells: [Point; SNAKE_LEN] = target.cells;
    target_cells.reverse();

    let snake_n = SNAKE_LEN as i32;
    let min_x = snake0.head_x().min(target.head_x()) - snake_n - 1;
    let max_x = snake0.head_x().max(target.head_x()) + snake_n + 1;
    let min_y = snake0.head_y().min(target.head_y()) - snake_n - 1;
    let max_y = snake0.head_y().max(target.head_y()) + snake_n + 1;

    let t0 = target_cells[0];
    let forbidden = &target_cells[1..];

    scratch.begin_search();
    scratch.try_visit(snake0, 0);

    let mut nodes: Vec<AStarNode> = Vec::with_capacity(256);
    let mut open_set = BinaryHeap::with_capacity(256);

    let h0 = (snake0.head_x() - t0.x).abs() + (snake0.head_y() - t0.y).abs();
    nodes.push(AStarNode {
        snake: snake0,
        parent: None,
    });
    open_set.push(HeapEntry {
        f: h0,
        w: 0,
        index: 0,
    });

    while let Some(HeapEntry { w, index, .. }) = open_set.pop() {
        let current_snake = nodes[index].snake;
        if scratch.best_cost_of(current_snake) != Some(w) {
            continue;
        }

        let head = current_snake.head();
        if head == t0 && can_follow_prefix(current_snake, forbidden) {
            let mut path = unwind_path(&nodes, index, None);
            path.extend(get_tunnel_path(current_snake, &target_cells));
            return Some(path);
        }

        for a in AROUND4 {
            let nx = head.x + a.x;
            let ny = head.y + a.y;

            let in_bounds = if let Some(g) = grid {
                g.is_inside_large(2, nx, ny) && g.is_empty_safe(nx, ny)
            } else {
                nx >= min_x && nx <= max_x && ny >= min_y && ny <= max_y
            };

            if in_bounds
                && !current_snake.will_self_collide(a.x, a.y)
                && !forbidden.iter().any(|p| p.x == nx && p.y == ny)
            {
                let nsnake = current_snake.next(a.x, a.y);
                let next_w = w + 1;
                if scratch.try_visit(nsnake, next_w) {
                    let h = (nx - t0.x).abs() + (ny - t0.y).abs();
                    let next_idx = nodes.len();
                    nodes.push(AStarNode {
                        snake: nsnake,
                        parent: Some(index),
                    });
                    open_set.push(HeapEntry {
                        f: next_w + h,
                        w: next_w,
                        index: next_idx,
                    });
                }
            }
        }
    }

    None
}
