mod app;
mod text; 


use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
