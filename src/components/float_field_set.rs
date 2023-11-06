use crate::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct FloatFieldSet {}

#[derive(Default, PartialEq, Clone, Properties)]
pub struct FloatFieldSetProps {
    pub human_name: String,
    pub input_name: String,
    pub value: f64,
    pub max: f64,
    pub on_input: Callback<f64>,
}

pub enum FloatFieldSetMessage {
    Changed(String),
}

impl Component for FloatFieldSet {
    type Message = FloatFieldSetMessage;
    type Properties = FloatFieldSetProps;

    fn create(_ctx: &Context<Self>) -> Self {
        FloatFieldSet {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FloatFieldSetMessage::Changed(v) => {
                let props = ctx.props();
                match v.parse::<f64>() {
                    Ok(val) => props.on_input.emit(val),
                    Err(e) => {
                        log!("failed to parse {v:#?} as f64: {e}")
                    }
                }
            }
        };

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let cb = ctx.link().callback(|e: InputEvent| {
            FloatFieldSetMessage::Changed(
                e.target_dyn_into::<HtmlInputElement>()
                    .expect("target of callback should be an HtmlInputElement")
                    .value(),
            )
        });

        return html! {
            <>
                <label>{&props.human_name}</label>
                <input
                    name={props.input_name.to_string()}
                    type="range"
                    min="0.0"
                    max={props.max.to_string()}
                    value={props.value.to_string()}
                    step={(props.max/500.0).to_string()}
                    oninput={&cb}
                    />
                <input
                    name={props.input_name.to_string()}
                    type="number"
                    min="0.0"
                    max={props.max.to_string()}
                    value={props.value.to_string()}
                    step={(props.max/500.0).to_string()}
                    oninput={&cb}
                    />
            </>
        };
    }
}
