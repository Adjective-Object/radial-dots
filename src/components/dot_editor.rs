use crate::components::float_field_set::FloatFieldSet;
use crate::fig::dot::Dot;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct DotEditor {
    pub dot: Dot,
    pub on_updated: Callback<(Dot)>,
}

#[derive(Default, PartialEq, Clone)]
pub struct DotEditorProps {
    pub dot: Dot,
    // TODO I'm only wrapping this in option because Callback
    // doesn't derive Default, but Option<Callback> does.
    pub on_updated: Option<Callback<(Dot)>>,
}

pub enum DotEditorMsg {
    CircleRadiusChange(f64),
    RingRadiusChange(f64),
    RingStrokeWidthChange(f64),
}

impl Component for DotEditor {
    type Message = DotEditorMsg;
    type Properties = DotEditorProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        DotEditor {
            dot: props.dot,
            on_updated: match props.on_updated {
                Some(x) => x,
                None => panic!("on_updated must be specified"),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            DotEditorMsg::RingRadiusChange(v) => self.on_updated.emit(Dot {
                ring_radius: v,
                ring_stroke_width: self.dot.ring_stroke_width,
                circle_radius: self.dot.circle_radius,
            }),
            DotEditorMsg::CircleRadiusChange(v) => self.on_updated.emit(Dot {
                ring_radius: self.dot.ring_radius,
                ring_stroke_width: self.dot.ring_stroke_width,
                circle_radius: v,
            }),
            DotEditorMsg::RingStrokeWidthChange(v) => self.on_updated.emit(Dot {
                ring_radius: self.dot.ring_radius,
                ring_stroke_width: v,
                circle_radius: self.dot.circle_radius,
            }),
        };

        false // update given in onChange in parent state
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let should_render = props.dot != self.dot;
        self.dot = props.dot;
        self.on_updated = match props.on_updated {
            Some(x) => x,
            None => panic!("on_updated must be specified"),
        };

        return should_render;
    }
}

impl Renderable<DotEditor> for DotEditor {
    fn view(&self) -> Html<Self> {
        return html! {
            <form class="dot-editor fieldset",>
                <FloatFieldSet:
                    human_name="Circle Radius",
                    input_name="circle_radius",
                    value={self.dot.circle_radius},
                    max={10.0},
                    on_input=|new_val| DotEditorMsg::CircleRadiusChange(new_val),
                />
                <FloatFieldSet:
                    human_name="Ring Radius",
                    input_name="ring_radius",
                    value={self.dot.ring_radius},
                    max={10.0},
                    on_input=|new_val| DotEditorMsg::RingRadiusChange(new_val),
                />
                <FloatFieldSet:
                    human_name="Ring Stoke Width",
                    input_name="ring_stroke_width",
                    value={self.dot.ring_stroke_width},
                    max={5.0},
                    on_input=|new_val| DotEditorMsg::RingStrokeWidthChange(new_val),
                />
            </form>
        };
    }
}
