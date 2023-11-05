use crate::components::float_field_set::FloatFieldSet;
use crate::components::svg_view::svg_view;
use crate::fig::text_path::{ArcPreviewStyle, ArcStyle};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

#[derive(PartialEq)]
pub struct ArcStyleEditor {
    pub arc_style: ArcStyle,
    pub on_updated: Callback<ArcStyle>,
}

#[derive(Default, PartialEq, Clone)]
pub struct ArcStyleEditorProps {
    pub arc_style: ArcStyle,
    // TODO I'm only wrapping this in option because Callback
    // doesn't derive Default, but Option<Callback> does.
    pub on_updated: Option<Callback<ArcStyle>>,
}

pub enum ArcStyleEditorMsg {
    UpdateRadius(f64),
    UpdateArcPercentage(f64),
    UpdateArcOffsetPercentage(f64),
}

impl Component for ArcStyleEditor {
    type Message = ArcStyleEditorMsg;
    type Properties = ArcStyleEditorProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ArcStyleEditor {
            arc_style: props.arc_style,
            on_updated: match props.on_updated {
                Some(x) => x,
                None => panic!("on_updated must be specified"),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ArcStyleEditorMsg::UpdateRadius(v) => self.on_updated.emit(ArcStyle {
                radius: v,
                arc_percentage: self.arc_style.arc_percentage,
                arc_offset_percentage: self.arc_style.arc_offset_percentage,
            }),
            ArcStyleEditorMsg::UpdateArcPercentage(v) => self.on_updated.emit(ArcStyle {
                radius: self.arc_style.radius,
                arc_percentage: v,
                arc_offset_percentage: self.arc_style.arc_offset_percentage,
            }),
            ArcStyleEditorMsg::UpdateArcOffsetPercentage(v) => self.on_updated.emit(ArcStyle {
                radius: self.arc_style.radius,
                arc_percentage: self.arc_style.arc_percentage,
                arc_offset_percentage: v,
            }),
        };

        false // update given in onChange in parent state
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let should_render = props.arc_style != self.arc_style;
        self.arc_style = props.arc_style;
        self.on_updated = match props.on_updated {
            Some(x) => x,
            None => panic!("on_updated must be specified"),
        };

        return should_render;
    }
}

impl Renderable<ArcStyleEditor> for ArcStyleEditor {
    fn view(&self) -> Html<Self> {
        return html! {
            <form class="arc-style-editor fieldset",>
                <span class="fields",>
                    <FloatFieldSet:
                        human_name="Radius",
                        input_name="arc-radius",
                        value={self.arc_style.radius},
                        max={50.0},
                        on_input=|new_val| ArcStyleEditorMsg::UpdateRadius(new_val),
                    />
                    <FloatFieldSet:
                        human_name="Arc Span",
                        input_name="arc-span",
                        value={self.arc_style.arc_percentage},
                        max={1.0},
                        on_input=|new_val| ArcStyleEditorMsg::UpdateArcPercentage(new_val),
                    />
                    <FloatFieldSet:
                        human_name="Arc Offset",
                        input_name="arc-offset",
                        value={self.arc_style.arc_offset_percentage},
                        max={1.0},
                        on_input=|new_val| ArcStyleEditorMsg::UpdateArcOffsetPercentage(new_val),
                    />
                </span>
                {svg_view(&self.arc_style, &ArcPreviewStyle {
                    color: &"#EEEEEE",
                    radius: 5.0,
                 })}
            </form>
        };
    }
}
