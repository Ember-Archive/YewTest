use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let onchange = Callback::from(|event: Event| {
        let target = event.target().unwrap();
        let input = target.unchecked_into::<HtmlInputElement>();
        log!(input.value());
    });

    html! {
        <input type="text" name={props.name.clone()} onchange={onchange} />
    }
}