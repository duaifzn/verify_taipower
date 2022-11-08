use crate::dto::request_dto::LoginDto;
use crate::dto::response_dto::{TokenDto, ResponseDto};
use crate::component::app::Route;
use crate::util::on_input_text::on_input_text;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::{use_history, AnyHistory, History};

const API_URL: &'static str = "http://211.73.81.185";

fn login_api(
    email: UseStateHandle<String>,
    password: UseStateHandle<String>,
    history: AnyHistory,
) -> Callback<MouseEvent> {
    let email = email.clone();
    let password = password.clone();
    Callback::once(move |_| {
        let email = email.clone();
        let password = password.clone();
        spawn_local(async move {
            let email = email.clone();
            let password = password.clone();
            let response = Request::post(&format!("{}/api/signin", API_URL))
                .header("Content-Type", "application/json")
                .json(&LoginDto {
                    email: (*email).to_string(),
                    password: (*password).to_string(),
                })
                .unwrap()
                .send()
                .await;
            match response {
                Ok(res) => {
                    let temp = res.json::<ResponseDto<TokenDto>>().await;
                    match temp {
                        Ok(data) => {
                            let _ = LocalStorage::set("Authorization", &data.json.unwrap().token);
                            history.push(Route::Verify)
                        }
                        Err(err) => log::info!("{:?}", err),
                    }
                }
                Err(err) => log::info!("{:?}", err),
            }
        })
    })
}

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
