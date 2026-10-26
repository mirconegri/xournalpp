use crate::model::{Color, Point, Stroke};

// Real-time ink stroke builder with jitter filtering
#[derive(Debug, Clone)]
pub struct PenHandler {
    current_stroke: Option<Stroke>,
    pub color: Color,
    pub width: f32,
    pub is_highlighter: bool,
}

impl PenHandler {
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

    // Append point if far enough from the previous one
    pub fn update(&mut self, point: Point) {
        if let Some(ref mut stroke) = self.current_stroke {
            if let Some(last) = stroke.points.last() {
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

    // Borrow the in-progress stroke for live preview rendering
    pub fn current_stroke(&self) -> Option<&Stroke> {
        self.current_stroke.as_ref()
    }
}
