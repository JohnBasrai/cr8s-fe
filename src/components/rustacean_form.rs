use web_sys::HtmlInputElement;
use yew::{prelude::*, platform::spawn_local};
use yew_router::prelude::*;

use crate::Route;
use crate::api::rustaceans::{api_rustacean_create, Rustacean, api_rustacean_update};
use crate::components::alert::Alert;
use crate::components::button::Button;
use crate::components::input::Input;
use crate::contexts::CurrentUserContext;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub rustacean: Option<Rustacean>
}

#[function_component(RustaceanForm)]
pub fn rustacean_form(props: &Props) -> Html {
    let navigator = use_navigator().expect("Navigator not available");
    let current_user_ctx = use_context::<CurrentUserContext>().expect("Current user context is missing");

    let name_handle = use_state(|| {
        if let Some(r) = &props.rustacean {
            return r.name.clone();
        }
        String::default()
    });
    let name = (*name_handle).clone();
    let email_handle = use_state(|| {
        if let Some(r) = &props.rustacean {
            return r.email.clone();
        }
        String::default()
    });
    let email = (*email_handle).clone();
    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    let name_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            name_handle.set(input.value());
        }
    });

    let email_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            email_handle.set(input.value());
        }
    });

    let cloned_name = name.clone();
    let cloned_email = email.clone();
    let cloned_rustacean = props.rustacean.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        let cloned_name = cloned_name.clone();
        let cloned_email = cloned_email.clone();
        let cloned_rustacean = cloned_rustacean.clone();
        let cloned_error_handle = error_message_handle.clone();
        let cloned_navigator = navigator.clone();
        let cloned_user_ctx = current_user_ctx.clone();
        match &cloned_user_ctx.token {
            Some(token) => {
                let cloned_token = token.clone();
                spawn_local(async move {
                    if let Some(rustacean) = cloned_rustacean {
                        match api_rustacean_update(
                            &cloned_token, 
                            rustacean.id.clone(), 
                            cloned_name.clone(), 
                            cloned_email.clone()
                        ).await {
                            Ok(_) => cloned_navigator.push(&Route::Rustaceans),
                            Err(e) => cloned_error_handle.set(e.to_string()),
                        }
                    } else {
                        match api_rustacean_create(
                            &cloned_token, 
                            cloned_name.clone(), 
                            cloned_email.clone()
                        ).await {
                            Ok(_) => cloned_navigator.push(&Route::Rustaceans),
                            Err(e) => cloned_error_handle.set(e.to_string()),
                        }
                    }
                });
            },
            None => cloned_error_handle.set("Session expired. Please login again".to_string()),
        }
    });

    html! {
        <form onsubmit={onsubmit}>
            if error_message.len() > 0 {
                <Alert alert_type={"danger"} message={error_message} />
            }
            <div class="mb-3">
                <Input
                    input_type="text"
                    name="name"
                    label="Name"
                    value={name}
                    onchange={name_changed}
                />
            </div>
            <div class="mb-3">
                <Input
                    input_type="email"
                    name="email"
                    label="E-mail"
                    value={email}
                    onchange={email_changed}
                />
            </div>
            <Button button_type="primary" label="Save" />  
        </form>
    }
}
