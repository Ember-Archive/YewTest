use yew::prelude::*;
use stylist::{yew::styled_component, Style};

const STYLESOURCE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLESOURCE).unwrap();

    html! {
        <div class={stylesheet}>
            <h1>{ "Hello World!" }</h1>
            <p>{ "Yew is cool" }</p>
        </div>
    }
}