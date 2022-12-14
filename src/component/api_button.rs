use crate::dto::response_dto::{ResponseDto, HashValueDto};
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
const ACCOUNT_NAME: &'static str = "admin123@gmail.com";
const API_URL: &'static str = "http://211.73.81.185";

#[derive(PartialEq, Properties, Clone)]
pub struct ApiButtonProps {
    pub contract_address: UseStateHandle<String>,
    pub contract_key_hash: UseStateHandle<String>,
    pub contract_value_hash: UseStateHandle<String>,
    pub contract_msg: UseStateHandle<String>,
    pub quantity_key_hash: UseStateHandle<String>,
    pub quantity_value_hash: UseStateHandle<String>,
    pub quantity_msg: UseStateHandle<String>,
    pub cert_key_hash: UseStateHandle<String>,
    pub cert_value_hash: UseStateHandle<String>,
    pub cert_msg: UseStateHandle<String>,
    pub contract_class: UseStateHandle<Classes>,
    pub quantity_class: UseStateHandle<Classes>,
    pub cert_class: UseStateHandle<Classes>,
}

#[function_component(ApiButton)]
pub fn api_button(
    ApiButtonProps {
        contract_address,
        contract_key_hash,
        contract_value_hash,
        contract_msg,
        quantity_key_hash,
        quantity_value_hash,
        quantity_msg,
        cert_key_hash,
        cert_value_hash,
        cert_msg,
        contract_class,
        quantity_class,
        cert_class,
    }: &ApiButtonProps,
) -> Html {
    let on_check_click: Callback<MouseEvent> = {
        let contract_address = contract_address.clone();
        let contract_key_hash = contract_key_hash.clone();
        let contract_value_hash = contract_value_hash.clone();
        let contract_msg = contract_msg.clone();
        let quantity_key_hash = quantity_key_hash.clone();
        let quantity_value_hash = quantity_value_hash.clone();
        let quantity_msg = quantity_msg.clone();
        let cert_key_hash = cert_key_hash.clone();
        let cert_value_hash = cert_value_hash.clone();
        let cert_msg = cert_msg.clone();
        let contract_class = contract_class.clone();
        let quantity_class = quantity_class.clone();
        let cert_class = cert_class.clone();
        Callback::from(move |_| {
            let contract_address = contract_address.clone();
            let contract_address_clone2 = contract_address.clone();
            let contract_address_clone3 = contract_address.clone();
            let contract_key_hash = contract_key_hash.clone();
            let contract_value_hash = contract_value_hash.clone();
            let contract_msg = contract_msg.clone();
            let quantity_key_hash = quantity_key_hash.clone();
            let quantity_value_hash = quantity_value_hash.clone();
            let quantity_msg = quantity_msg.clone();
            let cert_key_hash = cert_key_hash.clone();
            let cert_value_hash = cert_value_hash.clone();
            let cert_msg = cert_msg.clone();
            let contract_class = contract_class.clone();
            let quantity_class = quantity_class.clone();
            let cert_class = cert_class.clone();
            spawn_local(async move {
                let contract_address = contract_address.clone();
                let contract_key_hash = contract_key_hash.clone();
                let contract_value_hash = contract_value_hash.clone();
                let auth_token: String = LocalStorage::get("Authorization").unwrap_or_default();
                let fetch_value_hash: ResponseDto<HashValueDto> = Request::get(&format!(
                    "{}/api/contract/getHash?contract_address={}&account_name={}&key={}",
                    API_URL,
                    (*contract_address).clone(),
                    ACCOUNT_NAME,
                    (*contract_key_hash).clone()
                ))
                .header("Authorization", &auth_token.replace("\"", ""))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
                log::info!("{:?}", fetch_value_hash);
                match fetch_value_hash.json {
                    Some(result) => {
                        if result.hash_value == (*contract_value_hash) {
                            contract_msg.set("???????????????".to_string());
                            contract_class.set(classes!(
                                "border",
                                "border-success",
                                "border-5",
                                "rounded-4",
                                "m-2",
                                "p-2"
                            ))
                        } else {
                            contract_msg.set("???????????????".to_string());
                            contract_class.set(classes!(
                                "border",
                                "border-danger",
                                "border-5",
                                "rounded-4",
                                "m-2",
                                "p-2"
                            ))
                        }
                    }
                    None => {
                        contract_msg.set("????????????".to_string());
                    }
                }
            });
            spawn_local(async move {
                let contract_address = contract_address_clone2.clone();
                let quantity_key_hash = quantity_key_hash.clone();
                let auth_token: String = LocalStorage::get("Authorization").unwrap_or_default();
                let fetch_value_hash: ResponseDto<HashValueDto> = Request::get(&format!(
                    "{}/api/contract/getHash?contract_address={}&account_name={}&key={}",
                    API_URL,
                    (*contract_address).clone(),
                    ACCOUNT_NAME,
                    (*quantity_key_hash).clone()
                ))
                .header("Authorization", &auth_token.replace("\"", ""))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
                log::info!("{:?}", fetch_value_hash);
                match fetch_value_hash.json {
                    Some(result) => {
                        if result.hash_value == (*quantity_value_hash) {
                            quantity_msg.set("???????????????".to_string());
                            quantity_class.set(classes!(
                                "border",
                                "border-success",
                                "border-5",
                                "rounded-4",
                                "m-2",
                                "p-2"
                            ))
                        } else {
                            quantity_msg.set("???????????????".to_string());
                            quantity_class.set(classes!(
                                "border",
                                "border-danger",
                                "border-5",
                                "rounded-4",
                                "m-2",
                                "p-2"
                            ))
                        }
                    }
                    None => {
                        quantity_msg.set("????????????".to_string());
                    }
                }
            });
            spawn_local(async move {
                let contract_address = contract_address_clone3.clone();
                let cert_key_hash = cert_key_hash.clone();
                let auth_token: String = LocalStorage::get("Authorization").unwrap_or_default();
                let fetch_value_hash: ResponseDto<HashValueDto> = Request::get(&format!(
                    "{}/api/contract/getHash?contract_address={}&account_name={}&key={}",
                    API_URL,
                    (*contract_address).clone(),
                    ACCOUNT_NAME,
                    (*cert_key_hash).clone()
                ))
                .header("Authorization", &auth_token.replace("\"", ""))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
                cert_msg.set(format!("{:?}", fetch_value_hash));
                log::info!("{:?}", fetch_value_hash);
                match fetch_value_hash.json {
                    Some(result) => {
                        if result.hash_value == (*cert_value_hash) {
                            cert_msg.set("???????????????".to_string());
                            cert_class.set(classes!(
                                "border",
                                "border-success",
                                "border-5",
                                "rounded-4",
                                "m-2",
                                "p-2"
                            ));
                        } else {
                            cert_msg.set("???????????????".to_string());
                            cert_class.set(classes!(
                                "border",
                                "border-danger",
                                "border-5",
                                "rounded-4",
                                "m-2",
                                "p-2"
                            ));
                        }
                    }
                    None => {
                        cert_msg.set("????????????".to_string());
                    }
                }
            })
        })
    };
    html! {
        <div class="mx-auto">
            <button class="btn btn-dark" onclick={on_check_click.clone()}>{"??????"}</button>
        </div>
    }
}
