use crate::components::error_toast::ErrorToast;
use crate::components::svg_view::{svg_data_url, svg_view};
use crate::components::text_path_style_editor::TextPathStyleEditor;
use crate::drawing_style::{DrawingColors, DrawingStyle};
use crate::fig::diagram::Diagram;
use crate::fig::dot::Dot;
use crate::fig::text_path::ArcStyle;
use crate::fig::text_path::{TextPath, TextPathStyle};
use crate::serializable_app_state::{get_state_from_document_string, DeserializedAppState};
use stdweb::web::event::{DataTransfer, DataTransferItem, IDragEvent, IEvent};

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct App {
    style: DrawingStyle,
    diagram: Diagram,
    error_toasts: Vec<ErrorToast>,
    link: ComponentLink<App>,
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

    TryDropDocument(DataTransfer),
    ConsumeDroppedDocument(Result<DeserializedAppState, String>),
    // All event handlers are required to return a message.
    //
    // In some cases, we don't want to generate a message and instead want to
    // interact only with the event (e.g. preventing default for ondragover
    // to allow for drag/drop events).
    //
    // Here we add a doNothing event so that we can satisfy that requirement
    // without changing the data model
    DoNothing,
}


static mut global_app: Option<&App> = None;

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
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
            error_toasts: Vec::new(),
            link: link,
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
            AppMsg::TryDropDocument(data_transfer) => {
                if data_transfer.items().len() != 1 {
                    self.error_toasts.push(ErrorToast {
                        title: String::from("Error in Drag/Drop"),
                        body: String::from("More than one DataTransferItem on DataTransfer"),
                    });
                    return true;
                }

                let transfer_item: DataTransferItem = data_transfer.items().index(0).unwrap();

                // I'm stumped by the static lifetime requirement on this callback.
                // I get why it exists, but the safer version depends on futures, which
                // doesn't compile against my rust for whatever reason.
                global_app = unsafe{ std::mem::transumte(Some(self) };
                transfer_item.get_as_string(|value: String| {
                    let maybe_state = get_state_from_document_string(&value);
                    match global_app {
                        Some(app) => app.link.send_self(AppMsg::ConsumeDroppedDocument(maybe_state)),
                        None => {},
                    };
                });
                return false;
            }
            AppMsg::ConsumeDroppedDocument(maybe_doc) => match maybe_doc {
                Ok(doc) => {
                    self.diagram = doc.diagram;
                    self.style = doc.style;
                }
                Err(err_message) => self.error_toasts.push(ErrorToast {
                    title: String::from("Error Parsing dropped document"),
                    body: err_message,
                }),
            },
            AppMsg::DoNothing => {
                return false;
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

        let toasts = self.error_toasts.iter().map(|toast| html!{
            <div class="error-toast",>
                <span class="error-toast-title",>{toast.title.clone()}</span>
                <span class="error-toast-body",>{toast.body.clone()}</span>
            </div>
        });

        let data_href: String = svg_data_url(&self.diagram, &self.style);

        return html! {
            <>
                <link rel="stylesheet", type="text/css", href="./style.css", />
                <div class="error-toast-container",>
                    {for toasts}
                </div>
                <div class="app-split",
                     style=background_style,
                     ondragover=|e| {e.prevent_default(); AppMsg::DoNothing},
                     ondrop=|e| {e.prevent_default(); AppMsg::TryDropDocument(e.data_transfer().unwrap())},
                     >
                    {svg_view(&self.diagram, &self.style)}
                    <div class="control-bar",>
                        <section class="fields-container",>
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
                            </section>
                        <section class="download-container",>
                            <a
                                class="download-button",
                                download="radial-dots.svg",
                                href={data_href},
                                >
                                {"Download"}
                            </a>
                        </section>
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
