use yew::prelude::*;
use crate::component::{verify_input::VerifyInput, api_button::ApiButton};

#[function_component(App)]
pub fn app() -> Html {
    let contract_key_hash = use_state(|| "".to_string());
    let contract_value_hash = use_state(|| "".to_string());
    let contract_msg = use_state(|| "".to_string());
    let contract_class = use_state(|| classes!("border", "border-dark", "border-5", "rounded-4", "m-2", "p-2"));

    let quantity_key_hash = use_state(|| "".to_string());
    let quantity_value_hash = use_state(|| "".to_string());
    let quantity_msg = use_state(|| "".to_string());
    let quantity_class = use_state(|| classes!("border", "border-dark", "border-5", "rounded-4", "m-2", "p-2"));

    let cert_key_hash = use_state(|| "".to_string());
    let cert_value_hash = use_state(|| "".to_string());
    let cert_msg = use_state(|| "".to_string());
    let cert_class = use_state(|| classes!("border", "border-dark", "border-5", "rounded-4", "m-2", "p-2"));
   
    html! {
        <div class="container mt-3 p-3 rounded-5 bg-primary bg-opacity-10">
            <h1 class="text-center fw-semibold">{"區塊鏈驗證平台"}</h1>
            <VerifyInput
                title={"契約編號 key && value".to_string()}
                key_hash={contract_key_hash.clone()}
                value_hash={contract_value_hash.clone()}
                msg={contract_msg.clone()}
                change_class={contract_class.clone()}
                />
            <VerifyInput 
                title={"轉供度數 key && value".to_string()} 
                key_hash={quantity_key_hash.clone()} 
                value_hash={quantity_value_hash.clone()}
                msg={quantity_msg.clone()}
                change_class={quantity_class.clone()}
                />
            <VerifyInput 
                title={"憑證移轉資料 key && value".to_string()} 
                key_hash={cert_key_hash.clone()} 
                value_hash={cert_value_hash.clone()}
                msg={cert_msg.clone()}
                change_class={cert_class.clone()}
                />
            <ApiButton
                contract_key_hash={contract_key_hash.clone()}
                contract_value_hash={contract_value_hash.clone()}
                contract_msg={contract_msg.clone()}
                quantity_key_hash={quantity_key_hash.clone()}
                quantity_value_hash={quantity_value_hash.clone()}
                quantity_msg={quantity_msg.clone()}
                cert_key_hash={cert_key_hash.clone()}
                cert_value_hash={cert_value_hash.clone()}
                cert_msg={cert_msg.clone()}
                contract_class={contract_class.clone()}
                quantity_class={quantity_class.clone()}
                cert_class={cert_class.clone()}/>
                
        </div>
    }
}
