use yew::prelude::*;
pub enum Msg{
    Msg1,
}

pub struct App {
    name:String
}
impl Component for App {
    type Message = Msg; // set to enum
    type Properties = ();
    fn create (_ctx: &Context<Self>)->Self{
        Self {name: "Hello".into()}
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
        let link = ctx.link();{
            html!{
                
                }
            }
    }
}
