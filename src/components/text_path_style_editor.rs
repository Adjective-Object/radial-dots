use crate::components::{arc_style_editor::ArcStyleEditor, dot_editor::DotEditor};
use crate::drawing_style::DrawingColors;
use crate::fig::dot::Dot;
use crate::fig::text_path::{ArcStyle, TextPathStyle};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct TextPathStyleEditor {
    pub style: TextPathStyle,
    pub header: String,
    pub on_one_dot_updated: Callback<Option<Dot>>,
    pub on_zero_dot_updated: Callback<Option<Dot>>,
    pub on_arc_style_updated: Callback<Option<ArcStyle>>,

    pub on_add_one_dot_override: Option<Callback<()>>,
    pub on_add_zero_dot_override: Option<Callback<()>>,
    pub on_add_arc_style_override: Option<Callback<()>>,

    pub can_remove: bool,
    pub collapsed: bool,
}

#[derive(Default, PartialEq, Clone)]
pub struct TextPathStyleEditorProps {
    pub style: TextPathStyle,
    pub header: String,

    pub on_one_dot_updated: Option<Callback<Option<Dot>>>,
    pub on_zero_dot_updated: Option<Callback<Option<Dot>>>,
    pub on_arc_style_updated: Option<Callback<Option<ArcStyle>>>,

    pub on_add_one_dot_override: Option<Callback<()>>,
    pub on_add_zero_dot_override: Option<Callback<()>>,
    pub on_add_arc_style_override: Option<Callback<()>>,

    pub can_remove: bool,
}

pub enum TextPathStyleEditorMsg {
    OneDotUpdated(Option<Dot>),
    ZeroDotUpdated(Option<Dot>),
    ArcStyleUpdated(Option<ArcStyle>),
    ToggleCollapsed,

    OnAddOneDot,
    OnAddZeroDot,
    OnAddArcStyle,
}

impl Component for TextPathStyleEditor {
    type Message = TextPathStyleEditorMsg;
    type Properties = TextPathStyleEditorProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TextPathStyleEditor {
            style: props.style,
            header: props.header,

            on_one_dot_updated: match props.on_one_dot_updated {
                Some(x) => x,
                None => panic!("on_one_dot_updated must be specified"),
            },
            on_zero_dot_updated: match props.on_zero_dot_updated {
                Some(x) => x,
                None => panic!("on_zero_dot_updated must be specified"),
            },
            on_arc_style_updated: match props.on_arc_style_updated {
                Some(x) => x,
                None => panic!("on_arc_style_updated must be specified"),
            },

            collapsed: false,

            on_add_one_dot_override: props.on_add_one_dot_override,
            on_add_zero_dot_override: props.on_add_zero_dot_override,
            on_add_arc_style_override: props.on_add_arc_style_override,
            can_remove: props.can_remove,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TextPathStyleEditorMsg::OneDotUpdated(dot) => self.on_one_dot_updated.emit(dot),
            TextPathStyleEditorMsg::ZeroDotUpdated(dot) => self.on_zero_dot_updated.emit(dot),
            TextPathStyleEditorMsg::ArcStyleUpdated(arc_style) => {
                self.on_arc_style_updated.emit(arc_style)
            }
            TextPathStyleEditorMsg::ToggleCollapsed => {
                self.collapsed = !self.collapsed;
                return true;
            }
            TextPathStyleEditorMsg::OnAddOneDot => match &self.on_add_one_dot_override {
                Some(x) => x.emit(()),
                None => {}
            },
            TextPathStyleEditorMsg::OnAddZeroDot => match &self.on_add_zero_dot_override {
                Some(x) => x.emit(()),
                None => {}
            },
            TextPathStyleEditorMsg::OnAddArcStyle => match &self.on_add_arc_style_override {
                Some(x) => x.emit(()),
                None => {}
            },
        };

        false // update given in onChange in parent state
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let should_render = props.style != self.style
            || props.header != self.header
            || props.can_remove != self.can_remove;
        if props.style != self.style {
            self.style = props.style;
        }
        self.header = props.header;

        self.on_one_dot_updated = match props.on_one_dot_updated {
            Some(x) => x,
            None => panic!("on_one_dot_updated must be specified"),
        };
        self.on_zero_dot_updated = match props.on_zero_dot_updated {
            Some(x) => x,
            None => panic!("on_zero_dot_updated must be specified"),
        };
        self.on_arc_style_updated = match props.on_arc_style_updated {
            Some(x) => x,
            None => panic!("on_arc_style_updated must be specified"),
        };

        self.on_add_one_dot_override = props.on_add_one_dot_override;
        self.on_add_zero_dot_override = props.on_add_zero_dot_override;
        self.on_add_arc_style_override = props.on_add_arc_style_override;
        self.can_remove = props.can_remove;

        return should_render;
    }
}

impl Renderable<TextPathStyleEditor> for TextPathStyleEditor {
    fn view(&self) -> Html<Self> {
        let zero_dot_dom = match &self.style.zero_dot_style {
            Some(dot) => html! {<section>
                {if self.can_remove {
                    html!{
                        <button class="remove-override", onclick=|_| TextPathStyleEditorMsg::ZeroDotUpdated(None), >
                            {"x"}
                        </button>
                    }
                } else {
                    html! {
                        <></>
                    }
                }}
                <h3>{"Zero Dot"}</h3>
                <DotEditor:
                    dot={dot.clone()},
                    on_updated=|new_dot| TextPathStyleEditorMsg::ZeroDotUpdated(Some(new_dot)),
                    color_style={DrawingColors {
                        stroke_color: "#333333".to_string(),
                        background_color: "#EEEEEE".to_string(),
                    }},
                    />
                </section>
            },
            _ => {
                html! {
                    <button
                        class="add-override-fallback",
                        onclick=|_| TextPathStyleEditorMsg::OnAddZeroDot,
                        >
                        {"⊕ override [0] dot"}
                    </button>
                }
            }
        };

        let one_dot_dom = match &self.style.one_dot_style {
            Some(dot) => html! {<section>
                {if self.can_remove {
                    html!{
                        <button class="remove-override", onclick=|_| TextPathStyleEditorMsg::OneDotUpdated(None), >
                            {"x"}
                        </button>
                    }
                } else {
                    html! {
                        <></>
                    }
                }}
                <h3>{"One Dot"}</h3>
                <DotEditor:
                    dot={dot.clone()},
                    on_updated=|new_dot| TextPathStyleEditorMsg::OneDotUpdated(Some(new_dot)),
                    color_style={DrawingColors {
                        stroke_color: "#333333".to_string(),
                        background_color: "#EEEEEE".to_string(),
                    }},
                    />
                </section>
            },
            _ => {
                html! {
                    <button
                        class="add-override-fallback",
                        onclick=|_| TextPathStyleEditorMsg::OnAddOneDot,
                        >
                        {"⊕ override [1] dot "}
                    </button>
                }
            }
        };

        let arc_dom = match &self.style.arc_style {
            Some(arc) => html! {<section>
                {if self.can_remove {
                    html!{
                        <button class="remove-override", onclick=|_| TextPathStyleEditorMsg::ArcStyleUpdated(None), >
                            {"x"}
                        </button>
                    }
                } else {
                    html! {
                        <></>
                    }
                }}
                <h3>{"Arc Style"}</h3>
                <ArcStyleEditor:
                    arc_style={arc.clone()},
                    on_updated=|new_arc| TextPathStyleEditorMsg::ArcStyleUpdated(Some(new_arc)),
                    />
                </section>
            },
            _ => {
                html! {
                    <button
                        class="add-override-fallback",
                        onclick=|_| TextPathStyleEditorMsg::OnAddArcStyle,
                        >
                        {"⊕ override arc style"}
                    </button>
                }
            }
        };

        return html! {
            <section class="text-path-style-editor",>
                <button class="toggle-collapsed", onclick=|_| TextPathStyleEditorMsg::ToggleCollapsed, >
                    {if self.collapsed {"▼"} else {"▲"}}
                </button>
                <h2 class="text-path-header",>{&self.header}</h2>
                {if self.collapsed {
                    html!{<></>}
                } else {html!{
                    <>
                        {zero_dot_dom}
                        {one_dot_dom}
                        {arc_dom}
                    </>
                }}}
            </section>
        };
    }
}
