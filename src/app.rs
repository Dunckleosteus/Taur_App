use yew::prelude::*;
pub enum Msg{
    ChangePage,
}
pub enum Pages{
    Main,
    Env,
}
pub struct App {
   page: Pages
}
impl Component for App {
    type Message = Msg; // set to enum
    type Properties = ();

    fn create (_ctx: &Context<Self>)->Self{
        Self {page: Pages::Main}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message)->bool{
    // handles messages set to the component
        match msg{
            _=>{
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // describes how a component should be rendered
        let link = ctx.link();
        match self.page{
            Pages::Main => {
                html!{
                    <div class={"dropdown"}>
                    <button class={"dropbtn"}>{"Dropdown"}</button>
                        <div class={"dropdown-content"}>
                            <a onclick=link.callback(||)>{"Link 1"}</a>
                            <a href="#">{"Link 2"}</a>
                            <a href="#">{"Link 3"}</a>
                        </div>
                    </div>
                }
            },
            Pages::Env => {
                html!{
                    <h1> {"Environment Page"} </h1>
                }
            },
        }
    }
}
