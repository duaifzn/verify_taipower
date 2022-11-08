use crate::dto::request_dto::LoginDto;
use crate::dto::response_dto::{TokenDto, ResponseDto};
use crate::component::app::Route;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::{AnyHistory, History};
const API_URL: &'static str = "http://211.73.81.185";

pub fn login_api(
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