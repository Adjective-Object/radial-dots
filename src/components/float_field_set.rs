use yew::prelude::*;

pub struct FloatFieldSet {
    human_name: String,
    input_name: String,
    value: f64,
    max: f64,
    on_input: Callback<f64>,
}

#[derive(Default, PartialEq, Clone, Properties)]
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

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
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

    fn changed(&mut self, ctx: &Context<Self>, props: &FloatFieldSetProps) -> bool {
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FloatFieldSetMessage::Changed(v) => {
                self.on_input.emit(v);
            }
        };

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx.link().callback(|e: InputEvent|
            FloatFieldSetMessage::Changed(
                e.data().unwrap_or("".to_string()).parse().unwrap_or(0.0)));

        return html! {
            <>
                <label>{&self.human_name}</label>
                <input
                    name={self.input_name}
                    type="range"
                    min="0.0"
                    max={self.max.to_string()}
                    value={self.value.to_string()}
                    step={(self.max/500.0).to_string()}
                    oninput={cb}
                    />
                <input
                    name={self.input_name}
                    type="number"
                    min="0.0"
                    max={self.max.to_string()}
                    value={self.value.to_string()}
                    step={(self.max/500.0).to_string()}
                    oninput={cb}
                    />
            </>
        };
    }
}
