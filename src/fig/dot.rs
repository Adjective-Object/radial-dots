use crate::float_utils::fmax;
use crate::geom::{Rect, Vector2};
use crate::svg::svg_drawable::{SvgFragment, SvgRenderer};
use crate::svg::util::translate_svg;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct Dot {
    pub circle_radius: f64,
    pub ring_radius: f64,
    pub ring_stroke_width: f64,
}

impl Dot {
    pub fn get_bounding_radius(&self) -> f64 {
        let ring_radius: f64 = self.ring_radius + self.ring_stroke_width / 2.0;
        return fmax(&self.circle_radius, &ring_radius);
    }
}

impl SvgFragment<&str> for Dot {
    fn as_svg_fragment(&self, style: &&str) -> String {
        format!(
            concat!(
                "<circle r=\"{circle_radius}\" fill=\"{stroke_color}\" />",
                "<circle r=\"{ring_radius}\" ",
                "fill=\"transparent\" ",
                "stroke=\"{stroke_color}\" ",
                "stroke-width=\"{ring_stroke_width}\" />",
            ),
            circle_radius = self.circle_radius,
            ring_radius = self.ring_radius,
            ring_stroke_width = self.ring_stroke_width,
            stroke_color = style,
        )
    }
}

impl<'a> SvgRenderer<&str> for Dot {
    fn as_standalone_svg(&self, style: &&str) -> String {
        let radius = self.get_bounding_radius();
        let bounds: Rect = Rect {
            x: 0.0,
            y: 0.0,
            width: radius * 2.0,
            height: radius * 2.0,
        };
        let center: Vector2 = bounds.center();
        let base_svg: String = self.as_svg_fragment(style);

        return format!(
            "<svg xmlns='http://www.w3.org/2000/svg' viewBox='{} {} {} {}'>{}</svg>",
            bounds.x,
            bounds.y,
            bounds.width,
            bounds.height,
            translate_svg(&base_svg, center.x, center.y,),
        );
    }
}

impl std::fmt::Display for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(
            f,
            "Dot({}, {}, {})",
            self.circle_radius, self.ring_radius, self.ring_stroke_width
        );
    }
}
