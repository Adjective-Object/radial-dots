use crate::drawing_style::DrawingStyle;
use crate::fig::text_path::*;
use crate::float_utils::fmax;
use crate::geom::*;
use crate::svg::svg_drawable::{SvgFragment, SvgRenderer};
use crate::svg::util::*;
use crate::serializable_app_state::{SerializableAppState, serialize};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Diagram {
    pub paths: Vec<TextPath>,
    pub diagram_padding: f64,
}

impl Diagram {
    fn get_bounding_rect(&self, style: &DrawingStyle) -> Rect {
        let mut radius: f64 = 1.0;
        for path in self.paths.iter() {
            radius = fmax(&radius, &path.get_bounding_radius(style));
        }

        return Rect {
            x: 0.0,
            y: 0.0,
            width: (radius + self.diagram_padding) * 2.0,
            height: (radius + self.diagram_padding) * 2.0,
        };
    }
}

impl SvgRenderer<DrawingStyle> for Diagram {
    fn as_standalone_svg(&self, style: &DrawingStyle) -> String {
        let mut svg_parts: Vec<String> = Vec::with_capacity(self.paths.len() + 1);
        let diagram_bounds: Rect = self.get_bounding_rect(style);
        let diagram_center: Vector2 = diagram_bounds.center();
        svg_parts.push(rect_svg(&diagram_bounds, &style.color.background_color));

        for path in self.paths.iter() {
            svg_parts.push(translate_svg(
                &path.as_svg_fragment(style),
                diagram_center.x,
                diagram_center.y,
            ));
        }

        let serialized_content = serialize(&SerializableAppState {
            diagram: self,
            style: style,
        });


        return format!(
            concat!(
                "<svg ",
                "xmlns='http://www.w3.org/2000/svg' ",
                "xmlns:dots='dots' dots:config='{}'",
                "viewBox='{} {} {} {}'>{}</svg>",
            ),
            // TODO escape the serialized content for safe insert into XML.
            // serialized_content
            diagram_bounds.x,
            diagram_bounds.y,
            diagram_bounds.width,
            diagram_bounds.height,
            svg_parts.join("")
        );
    }
}
