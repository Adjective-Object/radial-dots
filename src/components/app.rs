use crate::components::dot_editor::DotEditor;
use crate::drawing_style::DrawingStyle;
use crate::fig::diagram::Diagram;
use crate::fig::dot::Dot;
use crate::fig::text_path::ArcStyle;
use crate::fig::text_path::TextPath;
use crate::svg::svg_drawable::SvgDrawable;

use yew::{events::ChangeData, html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct App {
    style: DrawingStyle,
    diagram: Diagram,
}

pub enum AppMsg {
    UpdateDefaultOneDotStyle(Dot),
    UpdateDefaultZeroDotStyle(Dot),
    UpdateDefaultArcStyle(ArcStyle),
    UpdateBackgroundColor(String),
    UpdateStrokeColor(String),
    UpdateDiagramText(String),
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            style: DrawingStyle {
                stroke_color: "#333333".to_string(),
                background_color: "#EEEEEE".to_string(),
                default_zero_dot_style: Dot {
                    circle_radius: 1.0,
                    ring_radius: 2.0,
                    ring_stroke_width: 0.1,
                },
                default_one_dot_style: Dot {
                    circle_radius: 0.5,
                    ring_radius: 2.0,
                    ring_stroke_width: 0.0,
                },
                default_arc_style: ArcStyle {
                    radius: 5.0,
                    arc_percentage: 1.0,
                    arc_offset_percentage: 0.0,
                },
            },
            diagram: Diagram {
                diagram_padding: 5.0,
                paths: vec![
                    TextPath {
                        text: "he".to_string(),
                        zero_dot_style: None,
                        one_dot_style: None,
                        arc_style: None,
                    },
                    TextPath {
                        text: "ll".to_string(),
                        zero_dot_style: None,
                        one_dot_style: None,
                        arc_style: None,
                    },
                    TextPath {
                        text: "o".to_string(),
                        zero_dot_style: None,
                        one_dot_style: None,
                        arc_style: None,
                    },
                ],
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::UpdateDefaultOneDotStyle(new_style) => {
                self.style.default_one_dot_style = new_style;
            }
            AppMsg::UpdateDefaultZeroDotStyle(new_style) => {
                self.style.default_zero_dot_style = new_style;
            }
            AppMsg::UpdateDefaultArcStyle(new_style) => {
                self.style.default_arc_style = new_style;
            }
            AppMsg::UpdateBackgroundColor(new_color) => {
                self.style.background_color = new_color;
            }
            AppMsg::UpdateStrokeColor(new_color) => {
                self.style.stroke_color = new_color;
            }
            AppMsg::UpdateDiagramText(new_text) => {
                let mut new_text_paths: Vec<TextPath> = vec![];
                for (i, line) in new_text.split('\n').enumerate() {
                    if self.diagram.paths.len() < i {
                        new_text_paths.push(TextPath {
                            text: line.to_string(),
                            zero_dot_style: match &self.diagram.paths[i].zero_dot_style {
                                Some(x) => Some(x.clone()),
                                None => None,
                            },
                            one_dot_style: match &self.diagram.paths[i].one_dot_style {
                                Some(x) => Some(x.clone()),
                                None => None,
                            },
                            arc_style: match &self.diagram.paths[i].arc_style {
                                Some(x) => Some(x.clone()),
                                None => None,
                            },
                        })
                    } else {
                        new_text_paths.push(TextPath {
                            text: line.to_string(),
                            zero_dot_style: None,
                            one_dot_style: None,
                            arc_style: None,
                        })
                    }
                }
                self.diagram.paths = new_text_paths;
            }
        }
        true
    }
}

pub fn dots_diagram_view(app: &App) -> Html<App> {
    let raw_svg_string: String = app.diagram.as_svg(&app.style);
    let img_base64_src: String = format!(
        "data:image/svg+xml;base64,{}",
        base64::encode_config(&raw_svg_string, base64::STANDARD)
    );

    return html! {
        <img class="dot-ring-img", src=img_base64_src, />
    };
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        let background_style = format!("background-color: {}", self.style.background_color);

        return html! {
        <>
            <link rel="stylesheet", type="text/css", href="./style.css", />
            <div class="app-split", style=background_style,>
                {dots_diagram_view(self)}
                <div class="control-bar",>
                    <textarea
                        onchange=|e| AppMsg::UpdateDiagramText(match e {
                            ChangeData::Value(v) => v,
                            _ => panic!("no Value in textarea onChange")
                        }),>
                        {App::get_paths_as_multiline_text(
                            &self.diagram.paths,
                        )}
                    </textarea>
                    <h3>{"One Dot"}</h3>
                    <DotEditor:
                        dot={self.style.default_one_dot_style.clone()},
                        on_updated=|dot| AppMsg::UpdateDefaultOneDotStyle(dot),
                        />
                    <h3>{"Zero Dot"}</h3>
                    <DotEditor:
                        dot={self.style.default_zero_dot_style.clone()},
                        on_updated=|dot| AppMsg::UpdateDefaultZeroDotStyle(dot),
                        />
                    <hr class="controls-divider", />
                    <hr class="controls-divider", />
                </div>
            </div>
        </>
        };
    }
}

impl App {
    fn get_paths_as_multiline_text(paths: &Vec<TextPath>) -> String {
        let lines: Vec<String> = paths.iter().map(|path| path.text.clone()).collect();
        return lines.join("\n");
    }
}