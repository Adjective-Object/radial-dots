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

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.dot = props.dot;
        self.on_updated = match props.on_updated {
            Some(x) => x,
            None => panic!("on_updated must be specified"),
        };

        true
    }
}

impl Renderable<DotEditor> for DotEditor {
    fn view(&self) -> Html<Self> {
        return html! {
            <form class="dot-editor",>
                <DotFieldSet:
                    human_name="Circle Radius",
                    input_name="circle_radius",
                    value={self.dot.circle_radius},
                    on_input=|new_val| DotEditorMsg::CircleRadiusChange(new_val),
                />
                <DotFieldSet:
                    human_name="Ring Radius",
                    input_name="ring_radius",
                    value={self.dot.ring_radius},
                    on_input=|new_val| DotEditorMsg::RingRadiusChange(new_val),
                />
                <DotFieldSet:
                    human_name="Ring Stoke Width",
                    input_name="ring_stroke_width",
                    value={self.dot.ring_stroke_width},
                    on_input=|new_val| DotEditorMsg::RingStrokeWidthChange(new_val),
                />
            </form>
        };
    }
}

struct DotFieldSet {
    human_name: String,
    input_name: String,
    value: f64,
    on_input: Callback<f64>,
}

#[derive(Default, PartialEq, Clone)]
struct DotFieldSetProps {
    human_name: String,
    input_name: String,
    value: f64,
    on_input: Option<Callback<f64>>,
}

enum DotFieldSetMessage {
    Changed(f64),
}

impl Component for DotFieldSet {
    type Message = DotFieldSetMessage;
    type Properties = DotFieldSetProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        DotFieldSet {
            human_name: props.human_name,
            input_name: props.input_name,
            value: props.value,
            on_input: match props.on_input {
                Some(x) => x,
                None => panic!("on_input must be specified"),
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.human_name = props.human_name;
        self.input_name = props.input_name;
        self.value = props.value;
        self.on_input = match props.on_input {
            Some(x) => x,
            None => panic!("on_input must be specified"),
        };

        return true;
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            DotFieldSetMessage::Changed(v) => self.on_input.emit(v),
        };

        true
    }
}

impl Renderable<DotFieldSet> for DotFieldSet {
    fn view(&self) -> Html<DotFieldSet> {
        return html! {
            <>
                <label>{&self.human_name}</label>
                <input
                    name={&self.input_name},
                    type="range",
                    step="0.1",
                    min="0.0",
                    max="10.0",
                    value={self.value},
                    oninput=|e| DotFieldSetMessage::Changed(e.value.parse().unwrap()),
                    />
                <input
                    name={&self.input_name},
                    type="number",
                    step="0.1",
                    min="0.0",
                    max="10.0",
                    value={self.value},
                    oninput=|e| DotFieldSetMessage::Changed(e.value.parse().unwrap()),
                    />
            </>
        };
    }
}
