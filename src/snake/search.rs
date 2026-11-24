use std::collections::HashMap;

use super::point::Point;
use super::types::{Grid, SNAKE_LEN, Snake};

const MARGIN: usize = 2;
const BODY_STATES: usize = 125;

pub(crate) struct SearchScratch {
    grid_width: usize,
    grid_height: usize,
    padded_height: usize,
    gen_stamp: Vec<u32>,
    best_cost: Vec<i32>,
    current_gen: u32,
    fallback: HashMap<Snake, i32>,
    grid_stamp: Vec<u32>,
    grid_gen: u32,
    pub(crate) temp_grid: Grid,
}

impl SearchScratch {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        let padded_w = width + MARGIN * 2;
        let padded_h = height + MARGIN * 2;
        let total_states = padded_w * padded_h * BODY_STATES;
        let grid_cells = width * height;

        Self {
            grid_width: width,
            grid_height: height,
            padded_height: padded_h,
            gen_stamp: vec![0; total_states],
            best_cost: vec![0; total_states],
            current_gen: 0,
            fallback: HashMap::new(),
            grid_stamp: vec![0; grid_cells],
            grid_gen: 0,
            temp_grid: Grid::new(width, height),
        }
    }

    pub(crate) fn begin_search(&mut self) {
        self.current_gen = self.current_gen.wrapping_add(1);
        if self.current_gen == 0 {
            self.gen_stamp.fill(0);
            self.current_gen = 1;
        }
        self.fallback.clear();
    }

    #[inline]
    pub(crate) fn try_visit(&mut self, snake: Snake, cost: i32) -> bool {
        if let Some(idx) = self.state_index(snake) {
            if self.gen_stamp[idx] != self.current_gen || cost < self.best_cost[idx] {
                self.gen_stamp[idx] = self.current_gen;
                self.best_cost[idx] = cost;
                true
            } else {
                false
            }
        } else {
            match self.fallback.get_mut(&snake) {
                Some(prev) if cost >= *prev => false,
                Some(prev) => {
                    *prev = cost;
                    true
                }
                None => {
                    self.fallback.insert(snake, cost);
                    true
                }
            }
        }
    }

    #[inline]
    pub(crate) fn best_cost_of(&self, snake: Snake) -> Option<i32> {
        if let Some(idx) = self.state_index(snake) {
            (self.gen_stamp[idx] == self.current_gen).then_some(self.best_cost[idx])
        } else {
            self.fallback.get(&snake).copied()
        }
    }

    pub(crate) fn begin_grid_mark(&mut self) {
        self.grid_gen = self.grid_gen.wrapping_add(1);
        if self.grid_gen == 0 {
            self.grid_stamp.fill(0);
            self.grid_gen = 1;
        }
    }

    #[inline]
    pub(crate) fn mark_grid(&mut self, x: i32, y: i32) -> bool {
        if x >= 0 && y >= 0 && (x as usize) < self.grid_width && (y as usize) < self.grid_height {
            let idx = (x as usize) * self.grid_height + (y as usize);
            if self.grid_stamp[idx] == self.grid_gen {
                false
            } else {
                self.grid_stamp[idx] = self.grid_gen;
                true
            }
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn unmark_grid(&mut self, x: i32, y: i32) -> bool {
        if x >= 0 && y >= 0 && (x as usize) < self.grid_width && (y as usize) < self.grid_height {
            let idx = (x as usize) * self.grid_height + (y as usize);
            if self.grid_stamp[idx] == self.grid_gen {
                self.grid_stamp[idx] = 0;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    #[inline]
    fn state_index(&self, snake: Snake) -> Option<usize> {
        let margin = MARGIN as i32;
        let ox = snake.head_x() + margin;
        let oy = snake.head_y() + margin;
        let max_x = (self.grid_width + MARGIN * 2) as i32;
        let max_y = (self.grid_height + MARGIN * 2) as i32;

        if ox < 0 || oy < 0 || ox >= max_x || oy >= max_y {
            return None;
        }

        let mut body_code = 0usize;
        let mut mul = 1usize;
        for i in 1..SNAKE_LEN {
            let d = delta_code(Point::new(
                snake.cells[i].x - snake.cells[i - 1].x,
                snake.cells[i].y - snake.cells[i - 1].y,
            ))?;
            body_code += d * mul;
            mul *= 5;
        }

        let cell_idx = (ox as usize) * self.padded_height + (oy as usize);
        Some(cell_idx * BODY_STATES + body_code)
    }
}

#[inline]
const fn delta_code(d: Point) -> Option<usize> {
    match (d.x, d.y) {
        (0, 0) => Some(0),
        (1, 0) => Some(1),
        (-1, 0) => Some(2),
        (0, 1) => Some(3),
        (0, -1) => Some(4),
        _ => None,
    }
}
