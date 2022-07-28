use web_sys::{Event, HtmlInputElement, InputEvent};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;

fn on_input_text(state: UseStateHandle<String>) -> Callback<InputEvent> {
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

#[derive(PartialEq, Properties, Clone)]
pub struct VerifyInputProps {
    pub title: String,
    pub key_hash: UseStateHandle<String>,
    pub value_hash: UseStateHandle<String>,
    pub msg: UseStateHandle<String>,
    pub change_class: UseStateHandle<Classes>
}

#[function_component(VerifyInput)]
pub fn verify_input(
    VerifyInputProps {
        title,
        key_hash,
        value_hash,
        msg,
        change_class,
    }: &VerifyInputProps,
) -> Html {
    let on_input_of_key_hash = on_input_text(key_hash.clone());
    let on_input_of_value_hash = on_input_text(value_hash.clone());
    let msg = (**msg).clone();
    html! {
        <div class={(**change_class).clone()}>
            <label for="first-name">{ title }</label>
            <input class="form-control" placeholder={"Key"} oninput={on_input_of_key_hash.clone()}/>
            <input class="form-control" placeholder={"Value"} oninput={on_input_of_value_hash.clone()}/>
            <p class="m-2">{msg.clone()}</p>
        </div>
    }
}
