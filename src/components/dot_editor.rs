use crate::components::float_field_set::FloatFieldSet;
use crate::components::svg_view::svg_view;
use crate::drawing_style::DrawingColors;
use crate::fig::dot::Dot;
use yew::prelude::*;

pub struct DotEditor {}

#[derive(Default, PartialEq, Clone, Properties)]
pub struct DotEditorProps {
    pub dot: Dot,
    pub on_updated: Callback<Dot>,
    pub color_style: DrawingColors,
}

pub enum DotEditorMsg {
    CircleRadiusChange(f64),
    RingRadiusChange(f64),
    RingStrokeWidthChange(f64),
}

impl Component for DotEditor {
    type Message = DotEditorMsg;
    type Properties = DotEditorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        DotEditor {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            DotEditorMsg::RingRadiusChange(v) => props.on_updated.emit(Dot {
                ring_radius: v,
                ring_stroke_width: props.dot.ring_stroke_width,
                circle_radius: props.dot.circle_radius,
            }),
            DotEditorMsg::CircleRadiusChange(v) => props.on_updated.emit(Dot {
                ring_radius: props.dot.ring_radius,
                ring_stroke_width: props.dot.ring_stroke_width,
                circle_radius: v,
            }),
            DotEditorMsg::RingStrokeWidthChange(v) => props.on_updated.emit(Dot {
                ring_radius: props.dot.ring_radius,
                ring_stroke_width: v,
                circle_radius: props.dot.circle_radius,
            }),
        };

        false // update given in onChange in parent state
    }

    fn changed(&mut self, ctx: &Context<Self>, props: &Self::Properties) -> bool {
        let old_props = ctx.props();
        return props.dot != old_props.dot || props.color_style != old_props.color_style;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();
        return html! {
            <form class="dot-editor fieldset">
                <span class="fields">
                    <FloatFieldSet
                        human_name="Circle Radius"
                        input_name="circle_radius"
                        value={props.dot.circle_radius}
                        max={10.0}
                        on_input={link.callback(|new_val| DotEditorMsg::CircleRadiusChange(new_val))}
                    />
                    <FloatFieldSet
                        human_name="Ring Radius"
                        input_name="ring_radius"
                        value={props.dot.ring_radius}
                        max={10.0}
                        on_input={link.callback(|new_val| DotEditorMsg::RingRadiusChange(new_val))}
                    />
                    <FloatFieldSet
                        human_name="Ring Stoke Width"
                        input_name="ring_stroke_width"
                        value={props.dot.ring_stroke_width}
                        max={5.0}
                        on_input={link.callback(|new_val| DotEditorMsg::RingStrokeWidthChange(new_val))}
                    />
                </span>
                {svg_view(&props.dot, &"#EEEEEE" )}
            </form>
        };
    }
}
