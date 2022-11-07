use crate::component::{login::Login, verify::Verify};
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/verify/login")]
    Login,
    #[at("/verify/verify")]
    Verify,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => html! { <Login />},
        Route::Verify => {
            let auth_token: String = LocalStorage::get("Authorization").unwrap_or_default();
            if auth_token.len() == 0 {
                return html! {
                    <Redirect<Route> to={Route::Login}/>
                };
            } else {
                html! {
                    <Verify />
                }
            }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}
