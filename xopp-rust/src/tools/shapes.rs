use crate::model::{Color, Point, Stroke};

// Builds clean geometric shapes from start and end points
pub struct ShapeBuilder;

impl ShapeBuilder {
    // Generate a straight line stroke
    pub fn build_line(start: Point, end: Point, color: Color, width: f32) -> Stroke {
        let mut stroke = Stroke::new(color, width, false);
        stroke.add_point(start);
        stroke.add_point(end);
        stroke
    }

    // Generate an arrow stroke with a directional arrowhead
    pub fn build_arrow(start: Point, end: Point, color: Color, width: f32) -> Stroke {
        let mut stroke = Stroke::new(color, width, false);
        stroke.add_point(start);
        stroke.add_point(end);

        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let angle = dy.atan2(dx);
        let head_len = (width * 6.0).max(12.0);
        let arrow_angle = std::f32::consts::PI / 6.0;

        let left = Point::new(
            end.x - head_len * (angle - arrow_angle).cos(),
            end.y - head_len * (angle - arrow_angle).sin(),
            1.0,
        );
        let right = Point::new(
            end.x - head_len * (angle + arrow_angle).cos(),
            end.y - head_len * (angle + arrow_angle).sin(),
            1.0,
        );

        // Append arrowhead path
        stroke.add_point(left);
        stroke.add_point(end);
        stroke.add_point(right);
        stroke
    }

    // Generate a rectangle box stroke
    pub fn build_rectangle(p1: Point, p2: Point, color: Color, width: f32) -> Stroke {
        let mut stroke = Stroke::new(color, width, false);
        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_y = p1.y.max(p2.y);

        stroke.add_point(Point::new(min_x, min_y, 1.0));
        stroke.add_point(Point::new(max_x, min_y, 1.0));
        stroke.add_point(Point::new(max_x, max_y, 1.0));
        stroke.add_point(Point::new(min_x, max_y, 1.0));
        stroke.add_point(Point::new(min_x, min_y, 1.0));
        stroke
    }

    // Generate an ellipse stroke approximated by segments
    pub fn build_ellipse(p1: Point, p2: Point, color: Color, width: f32) -> Stroke {
        let mut stroke = Stroke::new(color, width, false);
        let cx = (p1.x + p2.x) * 0.5;
        let cy = (p1.y + p2.y) * 0.5;
        let rx = ((p2.x - p1.x) * 0.5).abs();
        let ry = ((p2.y - p1.y) * 0.5).abs();

        let steps = 48;
        for i in 0..=steps {
            let theta = (i as f32 / steps as f32) * std::f32::consts::TAU;
            stroke.add_point(Point::new(cx + rx * theta.cos(), cy + ry * theta.sin(), 1.0));
        }
        stroke
    }
}
