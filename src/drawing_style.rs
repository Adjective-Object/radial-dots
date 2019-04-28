use crate::fig::dot::Dot;
use crate::fig::text_path::ArcStyle;

#[derive(Debug)]
pub struct DrawingStyle {
    pub stroke_color: String,
    pub background_color: String,
    pub default_zero_dot_style: Dot,
    pub default_one_dot_style: Dot,
    pub default_arc_style: ArcStyle,
}
