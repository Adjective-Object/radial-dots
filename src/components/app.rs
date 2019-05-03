use crate::components::svg_view::svg_view;
use crate::components::text_path_style_editor::TextPathStyleEditor;
use crate::drawing_style::{DrawingColors, DrawingStyle};
use crate::fig::diagram::Diagram;
use crate::fig::dot::Dot;
use crate::fig::text_path::ArcStyle;
use crate::fig::text_path::{TextPath, TextPathStyle};

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct App {
    style: DrawingStyle,
    diagram: Diagram,
}

pub enum AppMsg {
    UpdateDefaultOneDotStyle(Option<Dot>),
    UpdateDefaultZeroDotStyle(Option<Dot>),
    UpdateDefaultArcStyle(Option<ArcStyle>),

    UpdatePathOneDotStyle(usize, Option<Dot>),
    UpdatePathZeroDotStyle(usize, Option<Dot>),
    UpdatePathArcStyle(usize, Option<ArcStyle>),
    InitPathOneDotStyle(usize),
    InitPathZeroDotStyle(usize),
    InitPathArcStyle(usize),

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
                color: DrawingColors {
                    stroke_color: "#333333".to_string(),
                    background_color: "#EEEEEE".to_string(),
                },
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
                        style: TextPathStyle {
                            zero_dot_style: None,
                            one_dot_style: None,
                            arc_style: None,
                        },
                    },
                    TextPath {
                        text: "ll".to_string(),
                        style: TextPathStyle {
                            zero_dot_style: None,
                            one_dot_style: None,
                            arc_style: None,
                        },
                    },
                    TextPath {
                        text: "o".to_string(),
                        style: TextPathStyle {
                            zero_dot_style: None,
                            one_dot_style: None,
                            arc_style: None,
                        },
                    },
                ],
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::UpdateDefaultOneDotStyle(new_style) => match new_style {
                Some(x) => self.style.default_one_dot_style = x,
                None => panic!("default 1 style should not be updated"),
            },
            AppMsg::UpdateDefaultZeroDotStyle(new_style) => match new_style {
                Some(x) => self.style.default_zero_dot_style = x,
                None => panic!("default 0 style should not be updated"),
            },
            AppMsg::UpdateDefaultArcStyle(new_style) => match new_style {
                Some(x) => self.style.default_arc_style = x,
                None => panic!("default arc style should not be None"),
            },

            AppMsg::UpdatePathOneDotStyle(index, new_style) => {
                self.diagram.paths[index].style.one_dot_style = new_style;
            }
            AppMsg::UpdatePathZeroDotStyle(index, new_style) => {
                self.diagram.paths[index].style.zero_dot_style = new_style;
            }
            AppMsg::UpdatePathArcStyle(index, new_style) => {
                self.diagram.paths[index].style.arc_style = new_style;
            }
            AppMsg::InitPathOneDotStyle(index) => {
                self.diagram.paths[index].style.one_dot_style =
                    Some(self.style.default_one_dot_style.clone())
            }
            AppMsg::InitPathZeroDotStyle(index) => {
                self.diagram.paths[index].style.zero_dot_style =
                    Some(self.style.default_zero_dot_style.clone())
            }
            AppMsg::InitPathArcStyle(index) => {
                self.diagram.paths[index].style.arc_style =
                    Some(self.style.default_arc_style.clone())
            }

            AppMsg::UpdateBackgroundColor(new_color) => {
                self.style.color.background_color = new_color;
            }
            AppMsg::UpdateStrokeColor(new_color) => {
                self.style.color.stroke_color = new_color;
            }
            AppMsg::UpdateDiagramText(new_text) => {
                let mut new_text_paths: Vec<TextPath> = vec![];
                for (i, line) in new_text.split('\n').enumerate() {
                    if self.diagram.paths.len() > i {
                        new_text_paths.push(TextPath {
                            text: line.to_string(),
                            style: TextPathStyle {
                                zero_dot_style: match &self.diagram.paths[i].style.zero_dot_style {
                                    Some(x) => Some(x.clone()),
                                    None => None,
                                },
                                one_dot_style: match &self.diagram.paths[i].style.one_dot_style {
                                    Some(x) => Some(x.clone()),
                                    None => None,
                                },
                                arc_style: match &self.diagram.paths[i].style.arc_style {
                                    Some(x) => Some(x.clone()),
                                    None => None,
                                },
                            },
                        })
                    } else {
                        new_text_paths.push(TextPath {
                            text: line.to_string(),
                            style: TextPathStyle {
                                zero_dot_style: None,
                                one_dot_style: None,
                                arc_style: None,
                            },
                        })
                    }
                }
                self.diagram.paths = new_text_paths;
            }
        }
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        let background_style = format!("background-color: {}", self.style.color.background_color);

        let path_styles = self.diagram.paths.iter().enumerate().map(|(index, path)| {
            html! {
                <TextPathStyleEditor:
                    header={format!{"\"{}\"", path.text}},
                    style={path.style.clone()},
                    on_zero_dot_updated=move |dot| AppMsg::UpdatePathZeroDotStyle(index, dot),
                    on_one_dot_updated=move |dot| AppMsg::UpdatePathOneDotStyle(index, dot),
                    on_arc_style_updated=move |arc| AppMsg::UpdatePathArcStyle(index, arc),

                    on_add_one_dot_override=move |_| AppMsg::InitPathOneDotStyle(index),
                    on_add_zero_dot_override=move |_| AppMsg::InitPathZeroDotStyle(index),
                    on_add_arc_style_override=move |_| AppMsg::InitPathArcStyle(index),
                    can_remove={true},
                    />
            }
        });

        return html! {
            <>
                <link rel="stylesheet", type="text/css", href="./style.css", />
                <div class="app-split", style=background_style,>
                    {svg_view(&self.diagram, &self.style)}
                    <div class="control-bar",>
                        <textarea
                            class="control-textarea",
                            oninput=|e| AppMsg::UpdateDiagramText(e.value),>
                            {App::get_paths_as_multiline_text(
                                &self.diagram.paths,
                            )}
                        </textarea>
                        <TextPathStyleEditor:
                            header="Defaults",
                            style={TextPathStyle {
                                one_dot_style: Some(self.style.default_one_dot_style.clone()),
                                zero_dot_style: Some(self.style.default_zero_dot_style.clone()),
                                arc_style: Some(self.style.default_arc_style.clone()),
                            }},
                            on_zero_dot_updated=|dot| AppMsg::UpdateDefaultZeroDotStyle(dot),
                            on_one_dot_updated=|dot| AppMsg::UpdateDefaultOneDotStyle(dot),
                            on_arc_style_updated=|arc| AppMsg::UpdateDefaultArcStyle(arc),
                            />
                        <hr class="controls-divider", />
                        {for path_styles}
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
