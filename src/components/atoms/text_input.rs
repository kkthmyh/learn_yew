use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

use gloo::{console::log, net::websocket::Message};

#[derive(PartialEq,Properties)]
pub struct Props{
    pub name :String,
    pub hand_onchange: Callback<String>,
}
#[function_component]
pub fn TextInput(props:&Props) -> Html{
    let handle_onchange = props.hand_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event.target()
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .value();
        handle_onchange.emit(value);
    });
    html!{
        <input type= "text" name = {props.name.clone()} onchange = {onchange}  />
    }
        
}