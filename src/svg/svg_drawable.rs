use crate::drawing_style::DrawingStyle;

pub trait SvgDrawable {
    fn as_svg(&self, stroke_color: &DrawingStyle) -> String;
}
