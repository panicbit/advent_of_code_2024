use std::ops::{Add, AddAssign};

use crate::Direction;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

#[allow(non_snake_case)]
pub fn Position(x: i32, y: i32) -> Position {
    Position { x, y }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(mut self, direction: Direction) -> Self::Output {
        self += direction;
        self
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }
}
