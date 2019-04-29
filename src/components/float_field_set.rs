use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct FloatFieldSet {
    human_name: String,
    input_name: String,
    value: f64,
    max: f64,
    on_input: Callback<f64>,
}

#[derive(Default, PartialEq, Clone)]
pub struct FloatFieldSetProps {
    pub human_name: String,
    pub input_name: String,
    pub value: f64,
    pub max: f64,
    pub on_input: Option<Callback<f64>>,
}

pub enum FloatFieldSetMessage {
    Changed(f64),
}

impl Component for FloatFieldSet {
    type Message = FloatFieldSetMessage;
    type Properties = FloatFieldSetProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        FloatFieldSet {
            human_name: props.human_name,
            input_name: props.input_name,
            value: props.value,
            max: props.max,
            on_input: match props.on_input {
                Some(x) => x,
                None => panic!("on_input must be specified"),
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        let should_change = self.human_name != props.human_name
            || self.input_name != props.input_name
            || self.value != props.value
            || self.max != props.max;
        self.human_name = props.human_name;
        self.input_name = props.input_name;
        self.value = props.value;
        self.max = props.max;
        self.on_input = match props.on_input {
            Some(x) => x,
            None => panic!("on_input must be specified"),
        };

        return should_change;
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FloatFieldSetMessage::Changed(v) => {
                self.value = v;
                self.on_input.emit(v);
            },
        };

        false
    }
}

impl Renderable<FloatFieldSet> for FloatFieldSet {
    fn view(&self) -> Html<FloatFieldSet> {
        return html! {
            <>
                <label>{&self.human_name}</label>
                <input
                    name={&self.input_name},
                    type="range",
                    min="0.0",
                    max={self.max},
                    value={self.value},
                    step={self.max/500.0},
                    oninput=|e| FloatFieldSetMessage::Changed(e.value.parse().unwrap()),
                    />
                <input
                    name={&self.input_name},
                    type="number",
                    min="0.0",
                    max={self.max},
                    value={self.value},
                    step={self.max/500.0},
                    oninput=|e| FloatFieldSetMessage::Changed(e.value.parse().unwrap()),
                    />
            </>
        };
    }
}
