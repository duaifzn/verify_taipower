use yew;
use crate::component::app::App;

mod component;
mod dto;
mod util;
mod api;
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
