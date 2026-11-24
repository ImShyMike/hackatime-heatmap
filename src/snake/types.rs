use super::point::Point;

pub const SNAKE_LEN: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl Grid {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    #[inline]
    #[must_use]
    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    #[inline]
    #[must_use]
    pub fn is_inside_large(&self, m: i32, x: i32, y: i32) -> bool {
        x >= -m
            && y >= -m
            && x < (self.width as i32 + m)
            && y < (self.height as i32 + m)
    }

    #[inline]
    #[must_use]
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        x * self.height + y
    }

    #[inline]
    #[must_use]
    pub fn get_color(&self, x: usize, y: usize) -> u8 {
        self.data[self.get_index(x, y)]
    }

    #[inline]
    #[must_use]
    pub fn get_color_safe(&self, x: i32, y: i32) -> u8 {
        if self.is_inside(x, y) {
            self.data[self.get_index(x as usize, y as usize)]
        } else {
            0
        }
    }

    #[inline]
    pub fn set_color(&mut self, x: usize, y: usize, color: u8) {
        let idx = self.get_index(x, y);
        self.data[idx] = color;
    }

    #[inline]
    pub fn set_empty_safe(&mut self, x: i32, y: i32) {
        if self.is_inside(x, y) {
            let idx = self.get_index(x as usize, y as usize);
            self.data[idx] = 0;
        }
    }

    #[inline]
    #[must_use]
    pub fn is_empty_safe(&self, x: i32, y: i32) -> bool {
        self.get_color_safe(x, y) == 0
    }

    #[must_use]
    pub fn max_color(&self) -> u8 {
        self.data.iter().copied().max().unwrap_or(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Snake {
    pub cells: [Point; SNAKE_LEN],
}

impl Snake {
    #[must_use]
    pub const fn from_cells(cells: [Point; SNAKE_LEN]) -> Self {
        Self { cells }
    }

    #[inline]
    #[must_use]
    pub const fn head(&self) -> Point {
        self.cells[0]
    }

    #[inline]
    #[must_use]
    pub const fn head_x(&self) -> i32 {
        self.cells[0].x
    }

    #[inline]
    #[must_use]
    pub const fn head_y(&self) -> i32 {
        self.cells[0].y
    }

    #[inline]
    #[must_use]
    pub fn next(&self, dx: i32, dy: i32) -> Self {
        let mut copy = *self;
        copy.cells.copy_within(0..SNAKE_LEN - 1, 1);
        copy.cells[0] = Point::new(self.cells[0].x + dx, self.cells[0].y + dy);
        copy
    }

    #[inline]
    #[must_use]
    pub fn will_self_collide(&self, dx: i32, dy: i32) -> bool {
        let nx = self.cells[0].x + dx;
        let ny = self.cells[0].y + dy;

        // Tail vacates on move.
        self.cells[1..SNAKE_LEN - 1]
            .iter()
            .any(|cell| cell.x == nx && cell.y == ny)
    }
}

#[must_use]
pub const fn default_snake() -> Snake {
    Snake::from_cells([
        Point::new(0, -1),
        Point::new(1, -1),
        Point::new(2, -1),
        Point::new(3, -1),
    ])
}
