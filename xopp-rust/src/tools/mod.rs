pub mod pen;
pub mod eraser;
pub mod shapes;

// Active interactive tool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTool {
    // Standard pen
    Pen,
    // Translucent highlighter
    Highlighter,
    // Stroke and area eraser
    Eraser,
    // Straight line
    Line,
    // Arrow with head
    Arrow,
    // Rectangle box
    Rectangle,
    // Ellipse or circle
    Ellipse,
    // Selection tool
    Select,
}

impl Default for ActiveTool {
    fn default() -> Self {
        Self::Pen
    }
}
