use std::rc::Rc;

use gloo_storage::{SessionStorage, Storage};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::{UseReducerHandle, Reducible};

use crate::api::user::{User, MeResponse, LoginResponse, api_me};

pub type CurrentUserContext = UseReducerHandle<CurrentUser>;

#[derive(PartialEq, Default)]
pub struct CurrentUser {
    pub user: Option<User>,
    pub token: Option<String>,
}

impl Reducible for CurrentUser {
    type Action = CurrentUserDispatchActions;

    /// The reducer function.
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action.action_type {
            CurrentUserActions::LoginSuccess => {
                let me_resp = action.me_response.expect("Missing login response");
                let login_resp = action.login_response.expect("Missing login response");
                let _ = SessionStorage::set("cr8s_token", login_resp.token.clone());
                Self {
                    user: Some(User {
                        id: me_resp.id,
                        username: me_resp.username,
                        created_at: me_resp.created_at
                    }),
                    token: Some(login_resp.token)
                }.into()
            },
            CurrentUserActions::LoginFail => {
                SessionStorage::clear();
                Self {
                    user: None,
                    token: None
                }.into()
            },
        }
    }
}

pub struct CurrentUserDispatchActions {
    pub action_type: CurrentUserActions,
    pub login_response: Option<LoginResponse>,
    pub me_response: Option<MeResponse>
}

pub enum CurrentUserActions {
    LoginSuccess,
    LoginFail,
}


#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(CurrentUserProvider)]
pub fn curent_user_provider(props: &Props) -> Html {
    let user = use_reducer(CurrentUser::default);

    if user.user.is_none() {
        if let Ok(token) = SessionStorage::get::<String>("cr8s_token")  {
            let cloned_user = user.clone();
            spawn_local(async move {
                match api_me(&token).await {
                    Ok(me_response) => cloned_user.dispatch(CurrentUserDispatchActions {
                        action_type: CurrentUserActions::LoginSuccess,
                        login_response: Some(LoginResponse { token }),
                        me_response: Some(me_response),
                    }),
                    Err(_) => SessionStorage::clear(),
                }
            });
        }
    }

    html! {
        <ContextProvider<CurrentUserContext> context={user}>
            {props.children.clone()}
        </ContextProvider<CurrentUserContext>>
    }
}
