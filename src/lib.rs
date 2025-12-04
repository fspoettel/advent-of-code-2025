pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
}

pub const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::N,
    Direction::E,
    Direction::S,
    Direction::W,
    Direction::NW,
    Direction::NE,
    Direction::SE,
    Direction::SW,
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    #[inline(always)]
    pub fn neighbor(&self, direction: &Direction) -> Point {
        match direction {
            Direction::N => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::E => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::S => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::W => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::NE => Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Direction::NW => Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Direction::SE => Point {
                x: self.x + 1,
                y: self.y + 1,
            },
            Direction::SW => Point {
                x: self.x - 1,
                y: self.y + 1,
            },
        }
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    pub cells: Vec<Vec<T>>,
    pub cols: usize,
    pub rows: usize,
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(cells: Vec<Vec<T>>) -> Self {
        let rows = cells.len();
        let cols = cells[0].len();
        Self { cells, rows, cols }
    }
}

impl<T: Copy> Grid<T> {
    pub fn get(&self, point: &Point) -> T {
        self.cells[point.y as usize][point.x as usize]
    }

    pub fn point_inside(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.cols as isize && point.y >= 0 && point.y < self.rows as isize
    }

    pub fn neighbor(&self, point: &Point, direction: &Direction) -> Option<Point> {
        let neighbor = point.neighbor(direction);

        if self.point_inside(&neighbor) {
            Some(neighbor)
        } else {
            None
        }
    }
}

pub fn is_even(val: usize) -> bool {
    val.is_multiple_of(2)
}

pub fn count_digits(n: usize) -> usize {
    (n.checked_ilog10().unwrap_or(0) + 1) as usize
}
