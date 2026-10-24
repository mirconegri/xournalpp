use crate::model::{Color, Point, Stroke, StrokeStyle};

// Real-time ink stroke builder with smoothing
#[derive(Debug, Clone)]
pub struct PenHandler {
    // Current stroke being recorded
    current_stroke: Option<Stroke>,
    // Selected ink color
    pub color: Color,
    // Base stroke width
    pub width: f32,
    // Whether this pen operates as a highlighter
    pub is_highlighter: bool,
}

impl PenHandler {
    // Construct a pen handler with specific defaults
    pub fn new(color: Color, width: f32, is_highlighter: bool) -> Self {
        Self {
            current_stroke: None,
            color,
            width,
            is_highlighter,
        }
    }

    // Begin a new stroke at the given point
    pub fn start(&mut self, point: Point) {
        let mut stroke = Stroke::new(self.color, self.width, self.is_highlighter);
        stroke.add_point(point);
        self.current_stroke = Some(stroke);
    }

    // Add a point while dragging stylus or mouse
    pub fn update(&mut self, point: Point) {
        if let Some(ref mut stroke) = self.current_stroke {
            if let Some(last) = stroke.points.last() {
                // Ignore micro-jitter movements under 1.5 points
                if last.distance_to(&point) < 1.5 {
                    return;
                }
            }
            stroke.add_point(point);
        }
    }

    // Finish recording and return the completed stroke
    pub fn finish(&mut self) -> Option<Stroke> {
        self.current_stroke.take()
    }

    // Reference to the in-progress stroke for live preview
    pub fn current_stroke(&self) -> Option<&Stroke> {
        self.current_stroke.as_ref()
    }
}
