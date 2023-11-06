use crate::components::error_toast::ErrorToast;
use crate::components::svg_view::{svg_data_url, svg_view};
use crate::components::text_path_style_editor::TextPathStyleEditor;
use crate::drawing_style::{DrawingColors, DrawingStyle};
use crate::fig::diagram::Diagram;
use crate::fig::dot::Dot;
use crate::fig::text_path::ArcStyle;
use crate::fig::text_path::{TextPath, TextPathStyle};
use crate::log;
use crate::serializable_app_state::{get_state_from_document_string, DeserializedAppState};
use web_sys::{DataTransfer, DataTransferItem, File, HtmlTextAreaElement};
use yew::prelude::*;

pub struct App {
    style: DrawingStyle,
    diagram: Diagram,
    error_toasts: Vec<ErrorToast>,
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

    // UpdateBackgroundColor(String),
    // UpdateStrokeColor(String),
    UpdateDiagramText(String),

    TryDropDocument(DataTransfer),
    ConsumeDroppedDocument(Result<DeserializedAppState, String>),

    DismissErrorToast(usize),
}

// Gets a file from a data transfer object, assuming the
fn get_file_from_data_transfer(
    expected_mime: &str,
    data_transfer: DataTransfer,
) -> Result<File, ErrorToast> {
    if data_transfer.items().length() != 1 {
        log!("length not right");
        return Err(ErrorToast {
            title: String::from("Error in Drag/Drop"),
            body: format!(
                "Found {num} Data Transfer Items, expected 1",
                num = data_transfer.items().length()
            ),
        });
    }

    let transfer_item: DataTransferItem = data_transfer.items().get(0).unwrap();

    if transfer_item.type_() != expected_mime {
        return Err(ErrorToast {
            title: String::from("Error in Drag/Drop"),
            body: format!(
                "Dropped document had wrong mimetype ({mime}), expected \"{expected_mime}\"",
                mime = transfer_item.type_()
            ),
        });
    }

    if transfer_item.kind() != "file" {
        return Err(ErrorToast {
            title: String::from("Error in Drag/Drop"),
            body: format!("Dropped item was not a file"),
        });
    }

    return match transfer_item.get_as_file() {
        Ok(Some(file)) => Ok(file),
        Ok(None) => Err(ErrorToast {
            title: "Error in Drag/Drop".to_string(),
            body: format!("Failed converting DataTransferItem to file: got None file?"),
        }),
        Err(e) => Err(ErrorToast {
            title: "Error in Drag/Drop".to_string(),
            body: format!("Failed converting DataTransferItem to file: {e:#?}"),
        }),
    };
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
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
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
            // AppMsg::UpdateBackgroundColor(new_color) => {
            //     self.style.color.background_color = new_color;
            // }
            // AppMsg::UpdateStrokeColor(new_color) => {
            //     self.style.color.stroke_color = new_color;
            // }
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
                log!("TryDropDocument");
                let file = match get_file_from_data_transfer("image/svg+xml", data_transfer) {
                    Ok(f) => f,
                    Err(err_toast) => {
                        self.error_toasts.push(err_toast);
                        return true;
                    }
                };

                let cb = ctx.link().callback(|msg: AppMsg| msg);
                gloo_file::callbacks::read_as_text(&gloo_file::Blob::from(file), move |res| {
                    match res {
                        Ok(file_contents) => {
                            cb.emit(AppMsg::ConsumeDroppedDocument(
                                get_state_from_document_string(&file_contents),
                            ));
                        }
                        Err(read_err) => {
                            cb.emit(AppMsg::ConsumeDroppedDocument(Err(format!(
                                "Failed to get document body from reader body: {read_err:#?}",
                            ))));
                        }
                    }
                });
                return false;
            }
            AppMsg::ConsumeDroppedDocument(maybe_doc) => match maybe_doc {
                Ok(doc) => {
                    log!("Consume Drop Document");
                    self.diagram = doc.diagram;
                    self.style = doc.style;
                }
                Err(err_message) => {
                    log!("Fail to consume dropped document");
                    self.error_toasts.push(ErrorToast {
                        title: String::from("Error Parsing dropped document"),
                        body: err_message,
                    });
                }
            },
            AppMsg::DismissErrorToast(idx) => {
                if idx < self.error_toasts.len() {
                    self.error_toasts.remove(idx);
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let background_style = format!("background-color: {}", self.style.color.background_color);

        let path_styles = self.diagram.paths.iter().enumerate().map(|(index, path)| {
            html! {
                <TextPathStyleEditor
                    key={index}
                    header={format!{"\"{}\"", path.text}}
                    style={path.style.clone()}
                    on_zero_dot_updated={ctx.link().callback(move |dot| AppMsg::UpdatePathZeroDotStyle(index, dot))}
                    on_one_dot_updated={ctx.link().callback(move |dot| AppMsg::UpdatePathOneDotStyle(index, dot))}
                    on_arc_style_updated={ctx.link().callback(move |arc| AppMsg::UpdatePathArcStyle(index, arc))}

                    on_add_one_dot_override={ctx.link().callback(move |_| AppMsg::InitPathOneDotStyle(index))}
                    on_add_zero_dot_override={ctx.link().callback(move |_| AppMsg::InitPathZeroDotStyle(index))}
                    on_add_arc_style_override={ctx.link().callback(move |_| AppMsg::InitPathArcStyle(index))}
                    can_remove={true}
                    />
            }
        });

        let toasts = self.error_toasts.iter().enumerate().map(|(index, toast)| {
            html! {
                <div class="error-toast">
                    <span class="error-toast-title">{toast.title.clone()}</span>
                    <span class="error-toast-body">{toast.body.clone()}</span>
                    <button class="error-toast-dismiss-button" onclick={ctx.link().callback(move |_| AppMsg::DismissErrorToast(index))}>
                        {"x"}
                    </button>
                </div>
            }
        });

        let data_href: String = svg_data_url(&self.diagram, &self.style);
        return html! {
            <>
                <div class="error-toast-container">
                    {for toasts}
                </div>
                <div class="app-split"
                     style={background_style}
                     ondragover={|e: DragEvent| e.prevent_default()}
                     ondrop={ctx.link().callback(|e: DragEvent| {e.prevent_default(); AppMsg::TryDropDocument(e.data_transfer().unwrap())})}
                     >
                    {svg_view(&self.diagram, &self.style)}
                    <div class="control-bar">
                        <section class="fields-container">
                            <textarea
                                class="control-textarea"
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    Self::Message::UpdateDiagramText(e.target_dyn_into::<HtmlTextAreaElement>().expect("should be right value").value())
                                })}
                                value={App::get_paths_as_multiline_text(&self.diagram.paths)}/>
                            <TextPathStyleEditor
                                header="Defaults"
                                style={TextPathStyle {
                                    one_dot_style: Some(self.style.default_one_dot_style.clone()),
                                    zero_dot_style: Some(self.style.default_zero_dot_style.clone()),
                                    arc_style: Some(self.style.default_arc_style.clone()),
                                }}
                                on_zero_dot_updated={ctx.link().callback(|dot| AppMsg::UpdateDefaultZeroDotStyle(dot))}
                                on_one_dot_updated={ctx.link().callback(|dot| AppMsg::UpdateDefaultOneDotStyle(dot))}
                                on_arc_style_updated={ctx.link().callback(|arc| AppMsg::UpdateDefaultArcStyle(arc))}
                                />
                            <hr class="controls-divider" />
                            <>
                                // In the current version of yew, all {for elemes} should
                                // be wrapped in a v-list to prevent them from affecting the
                                // keying requirements of their peer nodes.
                                // See: https://github.com/yewstack/yew/issues/3262
                                {for path_styles}
                            </>
                            </section>
                        <section class="download-container">
                            <a
                                class="download-button"
                                download="radial-dots.svg"
                                href={data_href}
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
