use gloo::{console::log, net::websocket::Message};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use stylist::{yew::styled_component};

pub mod components;

use components::atoms::main_titles;
use components::molecules::custom_form;

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    address: String,
}

#[styled_component]
pub fn App() -> Html {

    let main_title_load = Callback::from(|message:String| log!(message));

        
    let name = "Brooks";
    log!(name);

    let obj = MyObj {
        name: String::from("Tom"),
        address: String::from("USA"),
    };
    log!(serde_json::to_string_pretty(&obj).unwrap());
    let class = "my_titles";
    let tasks = vec![html! {<li>{"record video"}</li>},html! {<li>{"shopping"}</li>}, html! {<li>{"pet Xilbe"}</li>}];
    html! {
        <>
        <main_titles::Main_titles title = "hi there 222666" on_load = {main_title_load}/>
        <custom_form::Custom_form />
        if class == "my_titles"{
            <p>{"Hi there"}</p>
        }
        <ul>
            {tasks}
        </ul>
        </>
    }
}
