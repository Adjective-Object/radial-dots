use crate::components::float_field_set::FloatFieldSet;
use crate::components::svg_view::svg_view;
use crate::fig::text_path::{ArcPreviewStyle, ArcStyle};
use yew::prelude::*;

#[derive(PartialEq)]
pub struct ArcStyleEditor {}

#[derive(Default, PartialEq, Clone, Properties)]
pub struct ArcStyleEditorProps {
    pub arc_style: ArcStyle,
    pub on_updated: Callback<ArcStyle>,
}

pub enum ArcStyleEditorMsg {
    UpdateRadius(f64),
    UpdateArcPercentage(f64),
    UpdateArcOffsetPercentage(f64),
}

impl Component for ArcStyleEditor {
    type Message = ArcStyleEditorMsg;
    type Properties = ArcStyleEditorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ArcStyleEditor {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            ArcStyleEditorMsg::UpdateRadius(v) => props.on_updated.emit(ArcStyle {
                radius: v,
                arc_percentage: props.arc_style.arc_percentage,
                arc_offset_percentage: props.arc_style.arc_offset_percentage,
            }),
            ArcStyleEditorMsg::UpdateArcPercentage(v) => props.on_updated.emit(ArcStyle {
                radius: props.arc_style.radius,
                arc_percentage: v,
                arc_offset_percentage: props.arc_style.arc_offset_percentage,
            }),
            ArcStyleEditorMsg::UpdateArcOffsetPercentage(v) => props.on_updated.emit(ArcStyle {
                radius: props.arc_style.radius,
                arc_percentage: props.arc_style.arc_percentage,
                arc_offset_percentage: v,
            }),
        };

        false // update given in onChange in parent state
    }

    fn changed(&mut self, ctx: &Context<Self>, props: &Self::Properties) -> bool {
        return ctx.props().arc_style != props.arc_style;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props();
        return html! {
            <form class="arc-style-editor fieldset">
                <span class="fields">
                    <FloatFieldSet
                        human_name="Radius"
                        input_name="arc-radius"
                        value={props.arc_style.radius}
                        max={50.0}
                        on_input={link.callback(
                            |new_val| ArcStyleEditorMsg::UpdateRadius(new_val))}
                    />
                    <FloatFieldSet
                        human_name="Arc Span"
                        input_name="arc-span"
                        value={props.arc_style.arc_percentage}
                        max={1.0}
                        on_input={link.callback(
                            |new_val| ArcStyleEditorMsg::UpdateArcPercentage(new_val))}
                    />
                    <FloatFieldSet
                        human_name="Arc Offset"
                        input_name="arc-offset"
                        value={props.arc_style.arc_offset_percentage}
                        max={1.0}
                        on_input={link.callback(
                            |new_val| ArcStyleEditorMsg::UpdateArcOffsetPercentage(new_val))}
                    />
                </span>
                {svg_view(&props.arc_style, &ArcPreviewStyle {
                    color: &"#EEEEEE",
                    radius: 5.0,
                 })}
            </form>
        };
    }
}
