// Core document and vector inking data model
pub mod point;
pub mod stroke;
pub mod layer;
pub mod page;
pub mod document;

pub use point::Point;
pub use stroke::{Color, Stroke, StrokeStyle};
pub use layer::Layer;
pub use page::{Page, PageBackground, PageBackgroundType};
pub use document::Document;
