use super::stroke::Stroke;

// A drawing layer containing strokes and annotations
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Layer {
    // List of strokes residing in this layer
    pub strokes: Vec<Stroke>,
    // Visibility toggle for this layer
    pub visible: bool,
}

impl Layer {
    // Construct a new active and visible layer
    pub fn new() -> Self {
        Self {
            strokes: Vec::new(),
            visible: true,
        }
    }

    // Add a stroke to the top of this layer
    pub fn add_stroke(&mut self, stroke: Stroke) {
        self.strokes.push(stroke);
    }

    // Delete a stroke at the given index
    pub fn remove_stroke(&mut self, index: usize) -> Option<Stroke> {
        if index < self.strokes.len() {
            Some(self.strokes.remove(index))
        } else {
            None
        }
    }
}
