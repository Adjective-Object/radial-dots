use crate::components::text_path_style_editor::TextPathStyleEditor;
use crate::drawing_style::DrawingStyle;
use crate::fig::diagram::Diagram;
use crate::fig::dot::Dot;
use crate::fig::text_path::ArcStyle;
use crate::fig::text_path::{TextPath, TextPathStyle};
use crate::svg::svg_drawable::{SvgRenderer};

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub fn dots_diagram_view<T, TStyle>(svg_renderer: &SvgRenderer<TStyle>, style: &TStyle) -> Html<T> {
    let raw_svg_string: String = svg_renderer.as_standalone_svg(style);
    let img_base64_src: String = format!(
        "data:image/svg+xml;base64,{}",
        base64::encode_config(&raw_svg_string, base64::STANDARD)
    );

    return html! {
        <img class="dot-ring-img", src=img_base64_src, />
    };
}

