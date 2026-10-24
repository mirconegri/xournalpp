// Represents a single 2D sampled point along an ink trajectory
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    // Horizontal coordinate in page points
    pub x: f32,
    // Vertical coordinate in page points
    pub y: f32,
    // Stylus pressure normalized between 0.0 and 1.0
    pub pressure: f32,
}

impl Point {
    // Create a new point with standard pressure
    pub fn new(x: f32, y: f32, pressure: f32) -> Self {
        Self { x, y, pressure }
    }

    // Euclidean distance to another point
    pub fn distance_to(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
