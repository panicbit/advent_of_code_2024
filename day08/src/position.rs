use std::ops::{Add, Sub};

use crate::vector::Vector;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[allow(non_snake_case)]
pub fn Position(x: i32, y: i32) -> Position {
    Position { x, y }
}

impl Position {
    fn vector_to(&self, other: Position) -> Vector {
        Vector {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
}

impl Add<Vector> for Position {
    type Output = Position;

    fn add(self, vector: Vector) -> Self::Output {
        Self {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }
}

impl Sub for Position {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
