pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Rect {
    pub fn center(&self) -> Vector2 {
        return Vector2 {
            x: self.x + self.width / 2.0,
            y: self.y + self.height / 2.0,
        };
    }
}
