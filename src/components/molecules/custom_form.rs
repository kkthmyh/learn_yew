use yew::prelude::*;
use crate::components::atoms::text_input::TextInput;
use crate::components::atoms::custom_button::Custom_button;
use gloo::console::log;
#[function_component]
pub fn Custom_form() -> Html{
    let username_state = use_state(|| "no username set".to_owned());
    let cloned_username_state = username_state.clone();
    let username = Callback::from(move |username| {
        cloned_username_state.set(username);
    });
    html!{
        <from>
        <TextInput name = "username" hand_onchange = {username}/>
        <Custom_button label = "submit"/>
        <p>{"username: "}{&*username_state}</p>
        </from>
    }
}