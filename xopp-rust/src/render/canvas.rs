use egui::{Color32, Painter, Pos2, Rect, Stroke as EguiStroke};
use crate::model::{Page, Stroke};
use super::paper::PaperRenderer;

// Viewport transformation for panning and zooming
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    // Canvas pan offset
    pub pan: egui::Vec2,
    // Canvas zoom scale factor
    pub zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            pan: egui::vec2(60.0, 40.0),
            zoom: 1.0,
        }
    }
}

impl Viewport {
    // Transform a document point to canvas screen space
    pub fn doc_to_screen(&self, doc_point: Pos2) -> Pos2 {
        Pos2::new(
            doc_point.x * self.zoom + self.pan.x,
            doc_point.y * self.zoom + self.pan.y,
        )
    }

    // Transform a canvas screen position back to document coordinates
    pub fn screen_to_doc(&self, screen_pos: Pos2) -> Pos2 {
        Pos2::new(
            (screen_pos.x - self.pan.x) / self.zoom,
            (screen_pos.y - self.pan.y) / self.zoom,
        )
    }

    // Compute bounding screen rectangle for a document page
    pub fn page_screen_rect(&self, page: &Page) -> Rect {
        let min = self.doc_to_screen(Pos2::new(0.0, 0.0));
        let max = self.doc_to_screen(Pos2::new(page.width, page.height));
        Rect::from_min_max(min, max)
    }
}

// Canvas vector painter for document pages and ink strokes
pub struct CanvasRenderer;

impl CanvasRenderer {
    // Render an entire page including rulings and all visible layers
    pub fn render_page(painter: &Painter, page: &Page, viewport: &Viewport, preview_stroke: Option<&Stroke>) {
        let page_rect = viewport.page_screen_rect(page);

        // Draw paper rulings and margins
        PaperRenderer::render(painter, page, page_rect);

        // Draw strokes across all visible layers
        for layer in &page.layers {
            if !layer.visible {
                continue;
            }
            for stroke in &layer.strokes {
                Self::render_stroke(painter, stroke, viewport);
            }
        }

        // Draw the in-progress stroke currently being drawn
        if let Some(stroke) = preview_stroke {
            Self::render_stroke(painter, stroke, viewport);
        }
    }

    // Render a single vector stroke with rounded joins
    pub fn render_stroke(painter: &Painter, stroke: &Stroke, viewport: &Viewport) {
        if stroke.points.is_empty() {
            return;
        }

        let alpha = if stroke.is_highlighter { stroke.color.a } else { stroke.color.a };
        let color = Color32::from_rgba_unmultiplied(stroke.color.r, stroke.color.g, stroke.color.b, alpha);
        let screen_width = (stroke.width * viewport.zoom).max(0.5);
        let egui_stroke = EguiStroke::new(screen_width, color);

        if stroke.points.len() == 1 {
            // Render a single dab point as a filled circle
            let p = viewport.doc_to_screen(Pos2::new(stroke.points[0].x, stroke.points[0].y));
            painter.circle_filled(p, screen_width * 0.5, color);
            return;
        }

        // Render connected segments along stroke trajectory
        for i in 0..stroke.points.len() - 1 {
            let p1 = viewport.doc_to_screen(Pos2::new(stroke.points[i].x, stroke.points[i].y));
            let p2 = viewport.doc_to_screen(Pos2::new(stroke.points[i + 1].x, stroke.points[i + 1].y));
            painter.line_segment([p1, p2], egui_stroke);
            painter.circle_filled(p1, screen_width * 0.5, color);
        }

        if let Some(last) = stroke.points.last() {
            let p_last = viewport.doc_to_screen(Pos2::new(last.x, last.y));
            painter.circle_filled(p_last, screen_width * 0.5, color);
        }
    }
}
