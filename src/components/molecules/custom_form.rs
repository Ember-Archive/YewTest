use yew::prelude::*;
use crate::components::atoms::text_input::TextInput;
use crate::components::atoms::custom_button::CustomButton;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    html! {
        <div>
            <TextInput name="username" />
            <CustomButton label="Submit" />
        </div>
    }
}