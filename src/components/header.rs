use crate::components::button::Button;
use crate::{
    contexts::{CurrentUserActions, CurrentUserContext, CurrentUserDispatchActions},
    Route,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    match &current_user_ctx.user {
        Some(user) => {
            let cloned_user_ctx = current_user_ctx.clone();
            let onclick = Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                cloned_user_ctx.dispatch(CurrentUserDispatchActions {
                    action_type: CurrentUserActions::LoginFail,
                    login_response: None,
                    me_response: None,
                });
            });
            html! {
                <div class="text-end">
                    <p>
                        <span class="pe-1">{"Welcome "}{user.username.clone()}</span>
                        <Button button_type="danger" onclick={onclick} label="Logout" />
                    </p>
                </div>
            }
        }
        None => html! {
            <Redirect<Route> to={Route::Login} />
        },
    }
}
