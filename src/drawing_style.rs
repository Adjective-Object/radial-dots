use crate::fig::dot::Dot;
use crate::fig::text_path::ArcStyle;

#[derive(Debug)]
pub struct DrawingColors {
    pub stroke_color: String,
    pub background_color: String,
}

#[derive(Debug)]
pub struct DrawingStyle {
    pub color: DrawingColors,
    pub default_zero_dot_style: Dot,
    pub default_one_dot_style: Dot,
    pub default_arc_style: ArcStyle,
}


/// Render SVG fragment
pub trait HasStrokeColor {
    fn get_stroke_color(&self) -> String;
}

impl HasStrokeColor for DrawingColors {
    fn get_stroke_color(&self) {
        self.stroke_color;
    }
}

impl HasStrokeColor for DrawingStyle {
    fn get_stroke_color(&self) {
        self.color.stroke_color;
    }
}

impl HasStrokeColor for string {
    fn get_stroke_color(&self) {
        return self;
    }
}
