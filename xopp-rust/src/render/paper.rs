use egui::{Color32, Painter, Pos2, Rect, Stroke as EguiStroke};
use crate::model::{Page, PageBackgroundType};

// Renders notebook paper rulings, margins, and page shadows
pub struct PaperRenderer;

impl PaperRenderer {
    // Draw the notebook sheet background and ruling pattern
    pub fn render(painter: &Painter, page: &Page, rect: Rect) {
        // Draw soft outer elevation shadow
        let shadow_rect = rect.expand(3.0).translate(egui::vec2(0.0, 3.0));
        painter.rect_filled(shadow_rect, 4.0, Color32::from_black_alpha(45));

        // Draw pure white paper surface
        painter.rect_filled(rect, 2.0, Color32::WHITE);

        // Draw crisp paper boundary outline
        painter.rect_stroke(rect, 2.0, EguiStroke::new(1.0, Color32::from_rgb(226, 232, 240)));

        // Render chosen background ruling
        match page.background.bg_type {
            PageBackgroundType::Plain => {}
            PageBackgroundType::Lined => Self::render_lines(painter, rect, false),
            PageBackgroundType::Ruled => Self::render_lines(painter, rect, true),
            PageBackgroundType::Grid => Self::render_grid(painter, rect),
            PageBackgroundType::Dotted => Self::render_dots(painter, rect),
        }
    }

    // Render horizontal ruling lines and optional notebook margins
    fn render_lines(painter: &Painter, rect: Rect, with_margins: bool) {
        let line_spacing = 24.0;
        let top_offset = if with_margins { 72.0 } else { 36.0 };
        let rule_stroke = EguiStroke::new(0.8, Color32::from_rgb(191, 219, 254));

        let mut y = rect.min.y + top_offset;
        while y < rect.max.y - 20.0 {
            painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], rule_stroke);
            y += line_spacing;
        }

        if with_margins {
            // Draw red horizontal header separator line
            let header_y = rect.min.y + top_offset;
            let header_stroke = EguiStroke::new(1.0, Color32::from_rgb(239, 68, 68));
            painter.line_segment([Pos2::new(rect.min.x, header_y), Pos2::new(rect.max.x, header_y)], header_stroke);

            // Draw pink vertical left margin line
            let margin_x = rect.min.x + 72.0;
            let margin_stroke = EguiStroke::new(1.0, Color32::from_rgb(244, 114, 182));
            painter.line_segment([Pos2::new(margin_x, rect.min.y), Pos2::new(margin_x, rect.max.y)], margin_stroke);
        }
    }

    // Render 5mm graph grid lines
    fn render_grid(painter: &Painter, rect: Rect) {
        let step = 14.17;
        let grid_stroke = EguiStroke::new(0.5, Color32::from_rgb(219, 234, 254));

        let mut x = rect.min.x + step;
        while x < rect.max.x {
            painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], grid_stroke);
            x += step;
        }

        let mut y = rect.min.y + step;
        while y < rect.max.y {
            painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], grid_stroke);
            y += step;
        }
    }

    // Render dot matrix pattern
    fn render_dots(painter: &Painter, rect: Rect) {
        let step = 14.17;
        let dot_color = Color32::from_rgb(148, 163, 184);

        let mut y = rect.min.y + step;
        while y < rect.max.y {
            let mut x = rect.min.x + step;
            while x < rect.max.x {
                painter.circle_filled(Pos2::new(x, y), 0.75, dot_color);
                x += step;
            }
            y += step;
        }
    }
}
