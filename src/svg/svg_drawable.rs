/// Render SVG fragment
pub trait SvgFragment<TStyle> {
    fn as_svg_fragment(&self, stroke_color: &TStyle) -> String;
}

/// Render to
pub trait SvgRenderer<TStyle> {
    fn as_standalone_svg(&self, stroke_color: &TStyle) -> String;
}
