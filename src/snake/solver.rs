use std::collections::VecDeque;

use super::outside::Outside;
use super::pathfinding::{get_path_to_pose_with_scratch, get_path_to_tunnel_start};
use super::point::{AROUND4, Point};
use super::search::SearchScratch;
use super::tunnel::{
    TunnelPriority, get_best_tunnel_with_scratch, get_priority_with_scratch, get_tunnel_path,
};
use super::types::{Grid, Snake};

#[derive(Clone, Copy)]
struct BfsNode {
    snake: Snake,
    parent: Option<usize>,
}

fn get_path_to_next_point(
    grid: &Grid,
    snake0: Snake,
    color: u8,
    remaining_targets: &mut usize,
    scratch: &mut SearchScratch,
) -> Option<Vec<Snake>> {
    if *remaining_targets == 0 {
        return None;
    }

    scratch.begin_search();
    scratch.try_visit(snake0, 0);

    let mut nodes: Vec<BfsNode> = Vec::with_capacity(256);
    let mut queue: VecDeque<usize> = VecDeque::with_capacity(256);

    nodes.push(BfsNode {
        snake: snake0,
        parent: None,
    });
    queue.push_back(0);

    while let Some(curr_idx) = queue.pop_front() {
        let current_snake = nodes[curr_idx].snake;
        let head = current_snake.head();

        if scratch.unmark_grid(head.x, head.y) {
            *remaining_targets -= 1;

            let mut path = Vec::new();
            let mut curr = Some(curr_idx);
            while let Some(idx) = curr {
                if nodes[idx].parent.is_some() {
                    path.push(nodes[idx].snake);
                }
                curr = nodes[idx].parent;
            }
            path.reverse();
            return Some(path);
        }

        for a in AROUND4 {
            let nx = head.x + a.x;
            let ny = head.y + a.y;

            if grid.is_inside_large(2, nx, ny)
                && !current_snake.will_self_collide(a.x, a.y)
                && grid.get_color_safe(nx, ny) <= color
            {
                let nsnake = current_snake.next(a.x, a.y);
                if scratch.try_visit(nsnake, 0) {
                    let next_idx = nodes.len();
                    nodes.push(BfsNode {
                        snake: nsnake,
                        parent: Some(curr_idx),
                    });
                    queue.push_back(next_idx);
                }
            }
        }
    }

    None
}

fn mark_clean_tunnellable_points(
    grid: &Grid,
    outside: &Outside,
    color: u8,
    scratch: &mut SearchScratch,
) -> usize {
    let mut count = 0usize;
    let mut marked = vec![false; grid.width * grid.height];

    for x in 0..grid.width {
        for y in 0..grid.height {
            let c = grid.get_color(x, y);
            let idx = grid.get_index(x, y);
            if c > 0
                && c <= color
                && !marked[idx]
                && let Some(tunnel) =
                    get_best_tunnel_with_scratch(grid, outside, x as i32, y as i32, color, scratch)
            {
                for p in tunnel {
                    if grid.is_inside(p.x, p.y) && !grid.is_empty_safe(p.x, p.y) {
                        let p_idx = grid.get_index(p.x as usize, p.y as usize);
                        if !marked[p_idx] {
                            marked[p_idx] = true;
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    scratch.begin_grid_mark();
    for x in 0..grid.width {
        for y in 0..grid.height {
            if marked[grid.get_index(x, y)] {
                scratch.mark_grid(x as i32, y as i32);
            }
        }
    }

    count
}

fn clear_clean_colored_layer_with_scratch(
    grid: &mut Grid,
    outside: &mut Outside,
    snake0: Snake,
    color: u8,
    scratch: &mut SearchScratch,
) -> Vec<Snake> {
    let mut remaining_targets = mark_clean_tunnellable_points(grid, outside, color, scratch);
    let mut chain = Vec::new();
    let mut current_snake = snake0;

    while remaining_targets > 0 {
        if let Some(path) =
            get_path_to_next_point(grid, current_snake, color, &mut remaining_targets, scratch)
        {
            for snake in &path {
                grid.set_empty_safe(snake.head_x(), snake.head_y());
            }
            if let Some(last) = path.last() {
                current_snake = *last;
            }
            chain.extend(path);
        } else {
            break;
        }
    }

    outside.fill(grid, 0);
    chain
}

struct ResidualTunnel {
    point: Point,
    tunnel: Vec<Point>,
    priority: TunnelPriority,
}

fn get_residual_tunnels(
    grid: &Grid,
    outside: &Outside,
    color: u8,
    scratch: &mut SearchScratch,
) -> Vec<ResidualTunnel> {
    let mut tunnels = Vec::new();

    for x in 0..grid.width {
        for y in 0..grid.height {
            let c = grid.get_color(x, y);
            if c > 0 && c < color {
                let xi = x as i32;
                let yi = y as i32;
                if let Some(tunnel) =
                    get_best_tunnel_with_scratch(grid, outside, xi, yi, color, scratch)
                {
                    let priority = get_priority_with_scratch(grid, color, &tunnel, scratch);
                    tunnels.push(ResidualTunnel {
                        point: Point::new(xi, yi),
                        tunnel,
                        priority,
                    });
                }
            }
        }
    }

    tunnels
}

fn select_best_tunnel_index(tunnels: &[ResidualTunnel], head: Point) -> Option<usize> {
    let mut best_idx = None;
    let mut best_priority = TunnelPriority {
        less: 0,
        color: u32::MAX,
    };
    let mut min_dist = i32::MAX;

    for (i, t) in tunnels.iter().enumerate() {
        let Some(&first) = t.tunnel.first() else {
            continue;
        };
        let dist = head.distance_sq(first);

        let is_better = best_idx.is_none()
            || t.priority > best_priority
            || (t.priority == best_priority && dist < min_dist);

        if is_better {
            best_idx = Some(i);
            best_priority = t.priority;
            min_dist = dist;
        }
    }

    best_idx
}

fn clear_residual_colored_layer_with_scratch(
    grid: &mut Grid,
    outside: &mut Outside,
    snake0: Snake,
    color: u8,
    scratch: &mut SearchScratch,
) -> Vec<Snake> {
    let mut tunnels = get_residual_tunnels(grid, outside, color, scratch);
    let mut chain = Vec::new();
    let mut current_snake = snake0;

    while let Some(best_idx) = select_best_tunnel_index(&tunnels, current_snake.head()) {
        let chosen_tunnel = tunnels.swap_remove(best_idx).tunnel;
        let Some(&first_point) = chosen_tunnel.first() else {
            continue;
        };

        let Some(path_to_start) = get_path_to_tunnel_start(
            grid,
            current_snake,
            first_point,
            &chosen_tunnel[1..],
            scratch,
        ) else {
            continue;
        };

        if let Some(last) = path_to_start.last() {
            current_snake = *last;
        }
        chain.extend(path_to_start);

        let tunnel_path = get_tunnel_path(current_snake, &chosen_tunnel);
        if let Some(last) = tunnel_path.last() {
            current_snake = *last;
        }
        chain.extend(tunnel_path);

        for p in &chosen_tunnel {
            grid.set_empty_safe(p.x, p.y);
        }
        outside.fill(grid, 0);

        let mut i = 0;
        while i < tunnels.len() {
            let p = tunnels[i].point;
            if grid.is_empty_safe(p.x, p.y) {
                tunnels.swap_remove(i);
            } else if let Some(new_tunnel) =
                get_best_tunnel_with_scratch(grid, outside, p.x, p.y, color, scratch)
            {
                tunnels[i].priority = get_priority_with_scratch(grid, color, &new_tunnel, scratch);
                tunnels[i].tunnel = new_tunnel;
                i += 1;
            } else {
                tunnels.swap_remove(i);
            }
        }
    }

    chain
}

#[must_use]
pub fn get_best_route(grid: &mut Grid, initial_snake: Snake) -> Vec<Snake> {
    let mut outside = Outside::new(grid, 0);
    let mut scratch = SearchScratch::new(grid.width, grid.height);
    let mut chain = vec![initial_snake];
    let mut current_snake = initial_snake;
    let max_color = grid.max_color();

    for color in 1..=max_color {
        if color > 1 {
            let residual = clear_residual_colored_layer_with_scratch(
                grid,
                &mut outside,
                current_snake,
                color,
                &mut scratch,
            );
            if let Some(&last) = residual.last() {
                current_snake = last;
            }
            chain.extend(residual);
        }

        let clean = clear_clean_colored_layer_with_scratch(
            grid,
            &mut outside,
            current_snake,
            color,
            &mut scratch,
        );
        if let Some(&last) = clean.last() {
            current_snake = last;
        }
        chain.extend(clean);
    }

    if let Some(return_steps) =
        get_path_to_pose_with_scratch(current_snake, initial_snake, Some(grid), &mut scratch)
    {
        chain.extend(return_steps);
    }

    chain
}
