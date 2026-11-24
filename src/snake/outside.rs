use std::collections::VecDeque;

use super::point::{AROUND4, Point};
use super::types::Grid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outside {
    pub width: usize,
    pub height: usize,
    /// 0 = outside / reachable, 1 = enclosed / blocked.
    pub data: Vec<u8>,
    queue: VecDeque<Point>,
}

impl Outside {
    #[must_use]
    pub fn new(grid: &Grid, color: u8) -> Self {
        let mut outside = Self {
            width: grid.width,
            height: grid.height,
            data: vec![1; grid.width * grid.height],
            queue: VecDeque::with_capacity(grid.width + grid.height),
        };
        outside.fill(grid, color);
        outside
    }

    #[inline]
    #[must_use]
    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    #[inline]
    #[must_use]
    pub fn is_outside(&self, x: i32, y: i32) -> bool {
        if self.is_inside(x, y) {
            self.data[x as usize * self.height + y as usize] == 0
        } else {
            true
        }
    }

    pub fn fill(&mut self, grid: &Grid, color: u8) {
        self.queue.clear();

        for x in 0..self.width {
            for y in 0..self.height {
                let idx = x * self.height + y;
                if self.data[idx] == 0 || grid.get_color(x, y) > color {
                    continue;
                }

                let xi = x as i32;
                let yi = y as i32;
                if AROUND4.iter().any(|a| self.is_outside(xi + a.x, yi + a.y)) {
                    self.data[idx] = 0;
                    self.queue.push_back(Point::new(xi, yi));
                }
            }
        }

        while let Some(p) = self.queue.pop_front() {
            for a in AROUND4 {
                let nx = p.x + a.x;
                let ny = p.y + a.y;

                if self.is_inside(nx, ny) {
                    let n_idx = (nx as usize) * self.height + (ny as usize);
                    if self.data[n_idx] != 0 && grid.get_color(nx as usize, ny as usize) <= color {
                        self.data[n_idx] = 0;
                        self.queue.push_back(Point::new(nx, ny));
                    }
                }
            }
        }
    }
}
