use super::layer::Layer;
use super::stroke::Color;

// Background ruling format of a notebook page
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageBackgroundType {
    // Blank white sheet
    Plain,
    // Horizontal lined paper
    Lined,
    // Ruled lined paper with header and margin
    Ruled,
    // Grid paper with squares
    Grid,
    // Dotted pattern grid
    Dotted,
}

// Background specification for a document page
#[derive(Debug, Clone, PartialEq)]
pub struct PageBackground {
    // Ruling type of the page
    pub bg_type: PageBackgroundType,
    // Paper base tint color
    pub color: Color,
    // Optional path to PDF page or background image
    pub filename: Option<String>,
}

impl Default for PageBackground {
    // Default to classic notebook ruled paper
    fn default() -> Self {
        Self {
            bg_type: PageBackgroundType::Ruled,
            color: Color::rgb(255, 255, 255),
            filename: None,
        }
    }
}

// A single page inside the notebook document
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    // Page width in points (standard A4 is 595.275)
    pub width: f32,
    // Page height in points (standard A4 is 841.889)
    pub height: f32,
    // Page background ruling setup
    pub background: PageBackground,
    // Stack of drawing layers on this page
    pub layers: Vec<Layer>,
    // Index of the currently active layer
    pub active_layer_index: usize,
}

impl Page {
    // Standard ISO A4 dimensions in points
    pub const A4_WIDTH: f32 = 595.275;
    pub const A4_HEIGHT: f32 = 841.889;

    // Create a new blank A4 page with a default layer
    pub fn new_a4(bg_type: PageBackgroundType) -> Self {
        Self {
            width: Self::A4_WIDTH,
            height: Self::A4_HEIGHT,
            background: PageBackground {
                bg_type,
                color: Color::rgb(255, 255, 255),
                filename: None,
            },
            layers: vec![Layer::new()],
            active_layer_index: 0,
        }
    }

    // Get mutable access to the active drawing layer
    pub fn active_layer_mut(&mut self) -> &mut Layer {
        if self.layers.is_empty() {
            self.layers.push(Layer::new());
        }
        if self.active_layer_index >= self.layers.len() {
            self.active_layer_index = self.layers.len() - 1;
        }
        &mut self.layers[self.active_layer_index]
    }
}
