use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct Props{
    pub title: String,
    pub on_load: Callback<String>,
}

#[function_component]
pub fn Main_titles(props:&Props) -> Html {
    props.on_load.emit("66666".to_owned());
    html!{
        <h1>{props.title.to_string()}</h1>
    }
          
}
    
