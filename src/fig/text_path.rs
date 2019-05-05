use crate::drawing_style::DrawingStyle;
use crate::fig::dot::Dot;
use crate::float_utils::fmax;
use crate::geom::{Rect, Vector2};
use crate::svg::svg_drawable::{SvgFragment, SvgRenderer};
use crate::svg::util::translate_svg;
use crate::utf_to_binary::text_to_binary;
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ArcStyle {
    pub radius: f64,
    pub arc_percentage: f64,
    pub arc_offset_percentage: f64,
}

pub struct ArcPreviewStyle<'a> {
    pub radius: f64,
    pub color: &'a str,
}

impl<'a> SvgFragment<ArcPreviewStyle<'a>> for ArcStyle {
    fn as_svg_fragment(&self, style: &ArcPreviewStyle<'a>) -> String {
        let center_x = 0.0;
        let center_y = 0.0;

        let start_angle = self.arc_offset_percentage * std::f64::consts::PI * 2.0;
        let start_x = center_x + style.radius * f64::cos(start_angle);
        let start_y = center_y + style.radius * f64::sin(start_angle);

        let end_angle =
            (self.arc_offset_percentage + self.arc_percentage) * std::f64::consts::PI * 2.0;
        let end_x = center_x + style.radius * f64::cos(end_angle);
        let end_y = center_y + style.radius * f64::sin(end_angle);

        format!(
            concat!(
                "<g stroke=\"{color}\" stroke-width=\"{ring_stroke_width}\" fill=\"none\">",
                "<path d=\"M{start_x},{start_y} A{radius},{radius} 0 {large_arc},{sweep} {end_x},{end_y}\"/>",

                // "<circle r=\"{point_radius}\" stroke=\"none\" fill=\"{color}\" cx=\"{start_x}\" cy=\"{start_y}\" />",
                // "<circle r=\"{point_radius}\" stroke=\"none\" fill=\"{color}\" cx=\"{end_x}\" cy=\"{end_y}\" />",
                // "<circle r=\"{point_radius}\" stroke=\"none\" fill=\"{color}\" cx=\"{center_x}\" cy=\"{center_y}\" />",
                "</g>",
            ),
            color = style.color,
            ring_stroke_width = 1,
            start_x = start_x,
            start_y = start_y,
            radius = style.radius,
            // center_x = center_x,
            // center_y = center_y,
            // point_radius = 1,
            end_x = end_x,
            end_y = end_y,
            large_arc = if self.arc_percentage >= 0.5 { 1 } else { 0 },
            sweep = 1,
        )
    }
}

impl<'a> SvgRenderer<ArcPreviewStyle<'a>> for ArcStyle {
    fn as_standalone_svg(&self, style: &ArcPreviewStyle<'a>) -> String {
        let bounds: Rect = Rect {
            x: 0.0,
            y: 0.0,
            width: (style.radius + 1.0) * 2.0,
            height: (style.radius + 1.0) * 2.0,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct TextPath {
    pub style: TextPathStyle,
    pub text: String,
}

#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize)]
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
