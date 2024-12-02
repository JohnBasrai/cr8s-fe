use crate::components::button::Button;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::api::user::{api_login, api_me, LoginResponse, MeResponse};
use crate::components::alert::Alert;
use crate::components::input::Input;
use crate::contexts::{CurrentUserActions, CurrentUserContext, CurrentUserDispatchActions};
use crate::Route;

async fn login(
    username: String,
    password: String,
) -> Result<(LoginResponse, MeResponse), gloo_net::Error> {
    let login_reponse = api_login(username, password).await?;
    let me_response = api_me(&login_reponse.token).await?;
    Ok((login_reponse, me_response))
}

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let navigator = use_navigator().expect("Navigator not available");
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    let username_handle = use_state(String::default);
    let username = (*username_handle).clone();
    let password_handle = use_state(String::default);
    let password = (*password_handle).clone();
    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    let username_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            username_handle.set(input.value());
        }
    });

    let password_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            password_handle.set(input.value());
        }
    });

    let cloned_username = username.clone();
    let cloned_password = password.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        let cloned_username = cloned_username.clone();
        let cloned_password = cloned_password.clone();
        let cloned_error_handle = error_message_handle.clone();
        let cloned_navigator = navigator.clone();
        let cloned_user_ctx = current_user_ctx.clone();
        spawn_local(async move {
            match login(cloned_username.clone(), cloned_password.clone()).await {
                Ok(responses) => {
                    cloned_user_ctx.dispatch(CurrentUserDispatchActions {
                        action_type: CurrentUserActions::LoginSuccess,
                        login_response: Some(responses.0),
                        me_response: Some(responses.1),
                    });
                    cloned_navigator.push(&Route::Home);
                }
                Err(e) => cloned_error_handle.set(e.to_string()),
            }
        });
    });

    html! {
        <form onsubmit={onsubmit}>
            if error_message.len() > 0 {
                <Alert alert_type={"danger"} message={error_message} />
            }
            <div class="mb-3">
                <Input
                    input_type="text"
                    name="username"
                    label="Username"
                    value={username}
                    onchange={username_changed}
                />
            </div>
            <div class="mb-3">
                <Input
                    input_type="password"
                    name="password"
                    label="Password"
                    value={password}
                    onchange={password_changed}
                />
            </div>
            <Button button_type="primary" label="Login" />
        </form>
    }
}
