use crate::fig::dot::Dot;
use crate::fig::text_path::ArcStyle;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
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
