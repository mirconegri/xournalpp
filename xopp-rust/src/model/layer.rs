use super::stroke::Stroke;

// A drawing layer containing strokes and annotations
#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub strokes: Vec<Stroke>,
    pub visible: bool,
}

impl Default for Layer {
    fn default() -> Self {
        Self::new()
    }
}

impl Layer {
    // Visible and empty by default
    pub fn new() -> Self {
        Self {
            strokes: Vec::new(),
            visible: true,
        }
    }

    pub fn add_stroke(&mut self, stroke: Stroke) {
        self.strokes.push(stroke);
    }

    pub fn remove_stroke(&mut self, index: usize) -> Option<Stroke> {
        if index < self.strokes.len() {
            Some(self.strokes.remove(index))
        } else {
            None
        }
    }
}
