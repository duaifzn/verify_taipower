use yew;
use crate::component::app::App;

mod component;
mod dto;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
