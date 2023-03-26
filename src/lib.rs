use yew::prelude::*;
use stylist::{yew::styled_component, style, Style};

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet: Style = style!(
        r#"
            h1 {
                color: yellow;
            }

            p {
                color: white;
            }
        "#
    ).unwrap();

    html! {
        <div class={stylesheet}>
            <h1>{ "Hello World!" }</h1>
            <p>{ "Yew is cool" }</p>
        </div>
    }
}