use crate::geom::Rect;

pub fn translate_svg(svg_string: &str, x: f64, y: f64) -> String {
    format!(
        "<g transform=\"translate({x}, {y})\">{content}</g>",
        x = x,
        y = y,
        content = svg_string
    )
}

pub fn rect_svg(rect: &Rect, fill: &str) -> String {
    return format!(
        concat!(
            "<rect ",
            "x=\"{}\" y=\"{}\" ",
            "width=\"{}\" height=\"{}\" ",
            "fill=\"{}\"/>",
        ),
        rect.x, rect.y, rect.width, rect.height, fill
    );
}