#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn flipped(&self) -> Vector {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
