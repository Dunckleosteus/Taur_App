use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub name: String, 
}
#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let onchange = Callback::from(|event: Event|{
        let target = event.target().unwrap();
        println!("{:?}", target);
    });

    html!{
        <input type={"text"} name={props.name.clone()} onchange={onchange}/>
    }
}