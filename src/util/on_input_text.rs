use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

pub fn on_input_text(state: UseStateHandle<String>) -> Callback<InputEvent> {
    let state = state.clone();
    Callback::from(move |e: InputEvent| {
        let state = state.clone();
        let event: Event = e.dyn_into().unwrap_throw();
        let event_target = event.target().unwrap_throw();
        let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
        //web_sys::console::log_1(&target.value().into());
        state.set(target.value());
    })
}