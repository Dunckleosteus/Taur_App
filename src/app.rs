use yew::prelude::*;
use crate::text::*; 
pub enum Msg{
    GotoMain,
    GotoEnv,
}
pub enum Pages{
    Main,
    Env,
}
pub struct App {
   page: Pages
}

// test start 
#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    match props.is_loading {
        true => html!{<p>{"Hello, you have chose a true value"}</p>},
        false =>html!{<p>{"You have chose a false value"}</p>},
    }
    //html! { <>{"Am I loading? - "}{props.is_loading.clone()}</> }
}
// test end 

impl Component for App {
    type Message = Msg; // set to enum
    type Properties = ();

    fn create (_ctx: &Context<Self>)->Self{
        Self {page: Pages::Main}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message)->bool{
    // handles messages set to the component
        match msg{
            Msg::GotoEnv =>{
                self.page = Pages::Env;
                true
            },
            Msg::GotoMain => {
                self.page = Pages::Main;
                true
            }, 
            _=>{
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let head = html!{
            <div class={"dropdown"}>
            <button class={"dropbtn"}>{"Dropdown"}</button>
                <div class={"dropdown-content"}>
                    <a onclick={link.callback(|_| Msg::GotoMain)}>{"HOME"}</a>
                    <a onclick={link.callback(|_| Msg::GotoEnv)}>{"ENV"}</a>
                    <a href="#">{"Link 3"}</a>
                </div>
            </div>
        };
        // describes how a component should be rendered
        
        match self.page{
            Pages::Main => {
                html!{
                    <div>
                        <h1> {"Main Page"} </h1>
                        {head}
                        <HelloWorld is_loading={false}/>
                        { TextInput }
                    </div>
                }
            },
            Pages::Env => {
                html!{
                    <div>
                        <h1> {"Environment Page"} </h1>
                        {head}
                    </div>
                }
            },
        }
    }
}
