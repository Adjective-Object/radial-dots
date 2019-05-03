use crate::drawing_style::DrawingStyle;
use crate::fig::dot::Dot;
use crate::float_utils::fmax;
use crate::svg::svg_drawable::SvgFragment;
use crate::svg::util::translate_svg;
use crate::utf_to_binary::text_to_binary;
// use wasm_bindgen::prelude::*;

// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ArcStyle {
    pub radius: f64,
    pub arc_percentage: f64,
    pub arc_offset_percentage: f64,
}

#[derive(Debug)]
pub struct TextPath {
    pub style: TextPathStyle,
    pub text: String,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TextPathStyle {
    pub zero_dot_style: Option<Dot>,
    pub one_dot_style: Option<Dot>,
    pub arc_style: Option<ArcStyle>,
}

impl<'style_and_self_lifetime> TextPath {
    fn get_arc_style(
        &'style_and_self_lifetime self,
        style: &'style_and_self_lifetime DrawingStyle,
    ) -> &'style_and_self_lifetime ArcStyle {
        match &self.style.arc_style {
            Some(style) => &style,
            None => &style.default_arc_style,
        }
    }

    fn get_zero_dot_style(
        &'style_and_self_lifetime self,
        style: &'style_and_self_lifetime DrawingStyle,
    ) -> &'style_and_self_lifetime Dot {
        match &self.style.zero_dot_style {
            Some(style) => &style,
            None => &style.default_zero_dot_style,
        }
    }

    fn get_one_dot_style(
        &'style_and_self_lifetime self,
        style: &'style_and_self_lifetime DrawingStyle,
    ) -> &'style_and_self_lifetime Dot {
        match &self.style.one_dot_style {
            Some(style) => &style,
            None => &style.default_one_dot_style,
        }
    }

    pub fn get_bounding_radius(&self, style: &DrawingStyle) -> f64 {
        let arc_style: &ArcStyle = self.get_arc_style(style);
        let zero_dot_style: &Dot = self.get_zero_dot_style(style);
        let one_dot_style: &Dot = self.get_one_dot_style(style);

        return arc_style.radius
            + fmax(
                &one_dot_style.get_bounding_radius(),
                &zero_dot_style.get_bounding_radius(),
            ) * 2.0;
    }
}

impl SvgFragment<DrawingStyle> for TextPath {
    /// Builds an svg for the text path
    ///
    /// The text path is radial and centered on the point (0,0)
    fn as_svg_fragment(&self, style: &DrawingStyle) -> String {
        let mut text_binary = match text_to_binary(&self.text) {
            Some(text_binary) => text_binary,
            None => vec![],
        };

        let zero_dot_string: String = self
            .get_zero_dot_style(style)
            .as_svg_fragment(&style.color.stroke_color.as_str());
        let one_dot_string: String = self
            .get_one_dot_style(style)
            .as_svg_fragment(&style.color.stroke_color.as_str());

        let mut dots: Vec<String> = Vec::with_capacity(text_binary.len());
        let arc_style = self.get_arc_style(style);

        let initial_angle = arc_style.arc_offset_percentage * std::f64::consts::PI * 2.0;
        let arc_range_angle = arc_style.arc_percentage * std::f64::consts::PI * 2.0;
        let num_dots = text_binary.len();

        for (index, current) in text_binary.iter_mut().enumerate() {
            let dot: &str = if *current {
                &one_dot_string
            } else {
                &zero_dot_string
            };

            let arc_percent = (index + 1) as f64 / (num_dots) as f64;
            let angle = initial_angle + arc_percent * arc_range_angle;

            let x = arc_style.radius * f64::cos(angle);
            let y = arc_style.radius * f64::sin(angle);

            let moved_dot = translate_svg(&dot, x, y);
            dots.push(moved_dot);
        }

        return dots.join("");
    }
}
