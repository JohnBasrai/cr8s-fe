use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::api::rustaceans::{api_rustacean_create, api_rustacean_update, Rustacean};
use crate::components::alert::Alert;
use crate::components::button::Button;
use crate::components::input::Input;
use crate::contexts::CurrentUserContext;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub rustacean: Option<Rustacean>,
}

#[function_component(RustaceanForm)]
pub fn rustacean_form(props: &Props) -> Html {
    // ---
    let navigator = use_navigator().expect("Navigator not available");
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

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

    let name_ = name.clone();
    let email_ = email.clone();
    let rustacean_ = props.rustacean.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        let name_ = name_.clone();
        let email_ = email_.clone();
        let rustacean_ = rustacean_.clone();
        let error_handle_ = error_message_handle.clone();
        let navigator_ = navigator.clone();
        let user_ctx_ = current_user_ctx.clone();

        match &user_ctx_.token {
            Some(token) => {
                let token = token.clone();
                spawn_local(async move {
                    if let Some(rustacean) = rustacean_ {
                        match api_rustacean_update(&token, rustacean.id, name_, email_).await {
                            Ok(_) => navigator_.push(&Route::Rustaceans),
                            Err(e) => error_handle_.set(e.to_string()),
                        }
                    } else {
                        match api_rustacean_create(&token, name_, email_).await {
                            Ok(_) => navigator_.push(&Route::Rustaceans),
                            Err(e) => error_handle_.set(e.to_string()),
                        }
                    }
                });
            }
            None => error_handle_.set("Session expired. Please login again".to_string()),
        }
    });

    html! {
        <form onsubmit={onsubmit}>
            if !error_message.is_empty() {
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
