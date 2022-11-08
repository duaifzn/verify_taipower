use crate::util::on_input_text::on_input_text;
use crate::api::login_api::login_api;

use yew::prelude::*;
use yew_router::prelude::use_history;

#[function_component(Login)]
pub fn login() -> Html {
    let history = use_history().unwrap();
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let on_input_of_email = on_input_text(email.clone());
    let on_input_of_password = on_input_text(password.clone());
    let login_api = login_api(email.clone(), password.clone(), history);
    html! {
        <div class="login">
            <h1>{"區塊鏈驗證平台"}</h1>
            <div>
                <input type="text" name="u" placeholder="使用者名稱" oninput={on_input_of_email.clone()} required=true />
                <input type="password" name="p" placeholder="密碼" oninput={on_input_of_password.clone()} required=true />
                <button type="submit" class="btn btn-primary btn-block btn-large" onclick={login_api.clone()}>{ "登入" }</button>
            </div>
        </div>
    }
}
