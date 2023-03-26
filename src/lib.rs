use gloo::console::log;
use yew::prelude::*;
use stylist::{yew::styled_component, Style};

mod components;

use components::atoms::main_title::{MainTitle, Color};
use components::molecules::custom_form::CustomForm;

const STYLESOURCE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet: Style = Style::new(STYLESOURCE).unwrap();

    let main_title_load: Callback<String> = Callback::from(|message: String| log!(message));

    html! {
        <div class={stylesheet}>
            <MainTitle title="hi there!" color={Color::Error} on_load={main_title_load} />
            <CustomForm />
        </div>
    }
}