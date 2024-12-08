pub struct Vector {
    pub x: i32,
    pub y: i32,
}

#[allow(non_snake_case)]
pub fn Vector(x: i32, y: i32) -> Vector {
    Vector { x, y }
}

impl Vector {
    pub fn flipped(&self) -> Vector {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
