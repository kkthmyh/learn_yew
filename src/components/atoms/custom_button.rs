use yew::prelude::*;

#[derive(PartialEq,Properties)]
pub struct Props {
    pub label :String,
}

#[function_component]
pub fn Custom_button(props:&Props) ->Html {

    html!{
        <button>{&props.label}</button>
    }


}