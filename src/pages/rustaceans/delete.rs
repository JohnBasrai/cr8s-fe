use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::api::rustaceans::api_rustacean_delete;
use crate::components::alert::Alert;
use crate::components::header::Header;
use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub rustacean_id: i32,
}

#[function_component(RustaceansDelete)]
pub fn rustaceans_delete(props: &Props) -> Html {
    let navigator = use_navigator().expect("Navigator not available");
    let current_user_ctx = use_context::<CurrentUserContext>().expect("Current user context is missing");

    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    match &current_user_ctx.token {
        Some(token) => {
            let rustacean_id = props.rustacean_id.clone();
            let token = token.to_owned();
            let onclick = Callback::from(move |e: MouseEvent| {
                e.prevent_default();

                // Clone these things so the can me 'move'd into the spawn local closeure below
                let token = token.clone();
                let navigator = navigator.clone();
                let error_message_handle = error_message_handle.clone();
                spawn_local(async move {
                    match api_rustacean_delete(&token, rustacean_id).await {
                        Ok(()) => navigator.push(&Route::Rustaceans),
                        Err(e) => error_message_handle.set(e.to_string()),
                    } 
                });
            });
            html! {
                <div class="container">
                    <div class="row">
                        <div class="col-sm-auto">
                            <Sidebar />
                        </div>
                        <div class="col mt-3">
                            <Header />
                            if error_message.len() > 0 {
                                <Alert alert_type={"danger"} message={error_message} />
                            }
                            <p>
                                {"Are you sure you want to delete rustacean #"}
                                {props.rustacean_id.clone()}
                            </p>
                            <button onclick={onclick} class="btn btn-danger">{"Delete"}</button>
                        </div>
                    </div>
                </div>
            }
        },
        None => html! {
            <Redirect<Route> to={Route::Login} />
        }
    }
}

