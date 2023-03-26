use gloo::console::log;
use yew::prelude::*;



#[function_component(App)]
pub fn app() -> Html {
    
    let tasks: Vec<&str> = vec!["clean room", "grocery shopping", "write Rust"];

    html! {
        <>
            <h1>{ "Hello World!" }</h1>
            <ul>
                {list_to_html(tasks)}
            </ul>
        </>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|task| html!{<li>{task}</li>}).collect()
}