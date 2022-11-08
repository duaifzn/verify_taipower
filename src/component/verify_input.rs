use yew::prelude::*;
use crate::util::on_input_text::on_input_text;


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
