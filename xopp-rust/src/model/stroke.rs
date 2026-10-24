use super::point::Point;

// Drawing style of the vector stroke
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrokeStyle {
    // Continuous solid line
    Solid,
    // Regular dashed line
    Dashed,
    // Dotted sequence
    Dotted,
}

// Color representation with 8-bit RGBA channels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    // Construct an opaque RGB color
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    // Construct an RGBA color with transparency
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    // Standard black ink
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    // Primary blue ink
    pub const BLUE: Self = Self::rgb(37, 99, 235);
    // Alert red ink
    pub const RED: Self = Self::rgb(220, 38, 38);
    // Forest green ink
    pub const GREEN: Self = Self::rgb(22, 163, 74);
    // Yellow highlighter tint
    pub const HIGHLIGHTER_YELLOW: Self = Self::rgba(250, 204, 21, 110);
    // Cyan highlighter tint
    pub const HIGHLIGHTER_CYAN: Self = Self::rgba(6, 182, 212, 110);

    // Convert from a standard 32-bit hex value (#RRGGBB or #AARRGGBB)
    pub fn from_hex_rgb(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Self::rgb(r, g, b)
    }

    // Convert to a 32-bit integer formatted as 0xRRGGBB
    pub fn to_hex_rgb(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

// Vector stroke comprising points, color, width, and tool style
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    // Sequence of recorded stroke coordinates
    pub points: Vec<Point>,
    // Display color of the stroke
    pub color: Color,
    // Base stroke width in points
    pub width: f32,
    // Stroke pattern style
    pub style: StrokeStyle,
    // Whether this stroke is a translucent highlighter
    pub is_highlighter: bool,
}

impl Stroke {
    // Create a new empty stroke with the given attributes
    pub fn new(color: Color, width: f32, is_highlighter: bool) -> Self {
        Self {
            points: Vec::new(),
            color,
            width,
            style: StrokeStyle::Solid,
            is_highlighter,
        }
    }

    // Append a point to the stroke trajectory
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    // Compute bounding box as (min_x, min_y, max_x, max_y)
    pub fn bounding_box(&self) -> Option<(f32, f32, f32, f32)> {
        if self.points.is_empty() {
            return None;
        }
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for p in &self.points {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }

        let radius = self.width * 0.5;
        Some((min_x - radius, min_y - radius, max_x + radius, max_y + radius))
    }

    // Check if a point hits near any segment of this stroke
    pub fn hits(&self, target: &Point, tolerance: f32) -> bool {
        let max_dist = (self.width * 0.5 + tolerance).powi(2);
        if let Some((min_x, min_y, max_x, max_y)) = self.bounding_box() {
            if target.x < min_x - tolerance || target.x > max_x + tolerance
                || target.y < min_y - tolerance || target.y > max_y + tolerance {
                return false;
            }
        }

        for i in 0..self.points.len().saturating_sub(1) {
            let p1 = &self.points[i];
            let p2 = &self.points[i + 1];
            if dist_to_segment_squared(target, p1, p2) <= max_dist {
                return true;
            }
        }
        false
    }
}

// Calculates squared distance from a point to a line segment
fn dist_to_segment_squared(p: &Point, a: &Point, b: &Point) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let len_sq = dx * dx + dy * dy;
    if len_sq == 0.0 {
        return p.distance_to(a).powi(2);
    }
    let t = ((p.x - a.x) * dx + (p.y - a.y) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);
    let proj_x = a.x + t * dx;
    let proj_y = a.y + t * dy;
    (p.x - proj_x).powi(2) + (p.y - proj_y).powi(2)
}
