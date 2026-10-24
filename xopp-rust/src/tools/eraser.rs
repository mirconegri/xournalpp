use crate::model::{Layer, Point};

// Mode of operation for the eraser tool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EraserMode {
    // Delete any stroke touched by the eraser
    Standard,
    // Whiteout over areas
    Whiteout,
}

// Handles erasing strokes from layers
#[derive(Debug, Clone)]
pub struct EraserHandler {
    // Eraser radius tolerance
    pub radius: f32,
    // Active erasing mode
    pub mode: EraserMode,
}

impl Default for EraserHandler {
    fn default() -> Self {
        Self {
            radius: 8.0,
            mode: EraserMode::Standard,
        }
    }
}

impl EraserHandler {
    // Erase strokes intersecting the given point from the layer
    pub fn erase_at(&self, layer: &mut Layer, point: &Point) -> bool {
        let before_count = layer.strokes.len();
        layer.strokes.retain(|stroke| !stroke.hits(point, self.radius));
        layer.strokes.len() != before_count
    }
}
