use crate::fig::dot::Dot;
use crate::fig::text_path::ArcStyle;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct DrawingColors {
    pub stroke_color: String,
    pub background_color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrawingStyle {
    pub color: DrawingColors,
    pub default_zero_dot_style: Dot,
    pub default_one_dot_style: Dot,
    pub default_arc_style: ArcStyle,
}
