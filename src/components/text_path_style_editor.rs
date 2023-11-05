use crate::components::{dot_editor::DotEditor, arc_style_editor::ArcStyleEditor};
use crate::drawing_style::DrawingColors;
use crate::fig::dot::Dot;
use crate::fig::text_path::{ArcStyle, TextPathStyle};
use yew::prelude::*;

pub struct TextPathStyleEditor {
    pub collapsed: bool,
}

#[derive(Default, PartialEq, Clone, Properties)]
pub struct TextPathStyleEditorProps {
    pub style: TextPathStyle,
    pub header: String,

    pub on_one_dot_updated: Callback<Option<Dot>>,
    pub on_zero_dot_updated: Callback<Option<Dot>>,
    pub on_arc_style_updated: Callback<Option<ArcStyle>>,

    #[prop_or(None)]
    pub on_add_one_dot_override: Option<Callback<()>>,
    #[prop_or(None)]
    pub on_add_zero_dot_override: Option<Callback<()>>,
    #[prop_or(None)]
    pub on_add_arc_style_override: Option<Callback<()>>,

    #[prop_or(false)]
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

    fn create(_ctx: &Context<Self>) -> Self {
        return Self{
            collapsed: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            TextPathStyleEditorMsg::OneDotUpdated(dot) => props.on_one_dot_updated.emit(dot),
            TextPathStyleEditorMsg::ZeroDotUpdated(dot) => props.on_zero_dot_updated.emit(dot),
            TextPathStyleEditorMsg::ArcStyleUpdated(arc_style) => {
                props.on_arc_style_updated.emit(arc_style)
            }
            TextPathStyleEditorMsg::ToggleCollapsed => {
                self.collapsed = !self.collapsed;
                return true;
            }
            TextPathStyleEditorMsg::OnAddOneDot => match &props.on_add_one_dot_override {
                Some(x) => x.emit(()),
                None => {},
            }
            TextPathStyleEditorMsg::OnAddZeroDot => match &props.on_add_zero_dot_override {
                Some(x) => x.emit(()),
                None => {},
            }
            TextPathStyleEditorMsg::OnAddArcStyle => match &props.on_add_arc_style_override {
                Some(x) => x.emit(()),
                None => {},
            }
        };

        false // update given in onChange in parent state
    }

    fn changed(&mut self, ctx: &Context<Self>, props: &Self::Properties) -> bool {
        let old_props = ctx.props();
        let should_render = props.style != old_props.style
            || props.header != old_props.header
            || props.can_remove != old_props.can_remove;
        return should_render;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();
        let zero_dot_dom = match &props.style.zero_dot_style {
            Some(dot) => html! {<section>
                {if props.can_remove {
                    html!{
                        <button
                            class="remove-override"
                            onclick={link.callback(|_| TextPathStyleEditorMsg::ZeroDotUpdated(None))}>
                            {"x"}
                        </button>
                    }
                } else {
                    html! {
                        <></>
                    }
                }}
                <h3>{"Zero Dot"}</h3>
                <DotEditor
                    dot={dot.clone()}
                    on_updated={ctx.link().callback(|new_dot| TextPathStyleEditorMsg::ZeroDotUpdated(Some(new_dot)))}
                    color_style={DrawingColors {
                        stroke_color: "#333333".to_string(),
                        background_color: "#EEEEEE".to_string()
                    }}
                    />
                </section>
            },
            _ => {
                html! {
                    <button
                        class="add-override-fallback"
                        onclick={link.callback(|_| TextPathStyleEditorMsg::OnAddZeroDot)}
                        >
                        {"⊕ override [0] dot"}
                    </button>
                }
            }
        };

        let one_dot_dom = match &props.style.one_dot_style {
            Some(dot) => html! {<section>
                {if props.can_remove {
                    html!{
                        <button
                            class="remove-override"
                            onclick={link.callback(|_| TextPathStyleEditorMsg::OneDotUpdated(None))}>
                            {"x"}
                        </button>
                    }
                } else {
                    html! {
                        <></>
                    }
                }}
                <h3>{"One Dot"}</h3>
                <DotEditor
                    dot={dot.clone()}
                    on_updated={ctx.link().callback(|new_dot| TextPathStyleEditorMsg::OneDotUpdated(Some(new_dot)))}
                    color_style={DrawingColors {
                        stroke_color: "#333333".to_string(),
                        background_color: "#EEEEEE".to_string(),
                    }}
                    />
                </section>
            },
            _ => {
                html! {
                    <button
                        class="add-override-fallback"
                        onclick={link.callback(|_| TextPathStyleEditorMsg::OnAddOneDot)}
                        >
                        {"⊕ override [1] dot "}
                    </button>
                }
            }
        };

        let arc_dom = match &props.style.arc_style {
            Some(arc) => html! {<section>
                {if props.can_remove {
                    html!{
                        <button class="remove-override" onclick={link.callback(|_| TextPathStyleEditorMsg::ArcStyleUpdated(None))}>
                            {"x"}
                        </button>
                    }
                } else {
                    html! {
                        <></>
                    }
                }}
                <h3>{"Arc Style"}</h3>
                <ArcStyleEditor
                    arc_style={arc.clone()}
                    on_updated={ctx.link().callback(|new_arc| TextPathStyleEditorMsg::ArcStyleUpdated(Some(new_arc)))}
                    />
                </section>
            },
            _ => {
                html! {
                    <button
                        class="add-override-fallback"
                        onclick={link.callback(|_| TextPathStyleEditorMsg::OnAddArcStyle)}
                        >
                        {"⊕ override arc style"}
                    </button>
                }
            }
        };

        return html! {
            <section class="text-path-style-editor">
                <button
                    class="toggle-collapsed"
                    onclick={link.callback(|_| TextPathStyleEditorMsg::ToggleCollapsed)}>
                    {if self.collapsed {"▼"} else {"▲"}}
                </button>
                <h2 class="text-path-header">{&props.header}</h2>
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
