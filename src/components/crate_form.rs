use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::api::crates::{api_crate_create, api_crate_update, Crate};
use crate::api::rustaceans::Rustacean;
use crate::components::alert::Alert;
use crate::components::input::Input;
use crate::components::select::Select;
use crate::components::textarea::Textarea;
use crate::contexts::CurrentUserContext;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub cr8: Option<Crate>,
    pub authors: Vec<Rustacean>,
}

#[function_component(CrateForm)]
pub fn crate_form(props: &Props) -> Html {
    let navigator = use_navigator().expect("Navigator not available");
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    let name_handle = use_state(|| {
        if let Some(c) = &props.cr8 {
            return c.name.clone();
        }
        String::default()
    });
    let name = (*name_handle).clone();
    let code_handle = use_state(|| {
        if let Some(c) = &props.cr8 {
            return c.code.clone();
        }
        String::default()
    });
    let code = (*code_handle).clone();
    let rustacean_id_handle = use_state(|| {
        if let Some(c) = &props.cr8 {
            return c.rustacean_id.to_string();
        }
        String::default()
    });
    let rustacean_id = (*rustacean_id_handle).clone();
    let version_handle = use_state(|| {
        if let Some(c) = &props.cr8 {
            return c.version.clone();
        }
        String::default()
    });
    let version = (*version_handle).clone();
    let description_handle = use_state(|| {
        if let Some(c) = &props.cr8 {
            if let Some(description) = c.description.clone() {
                return description;
            }
        }
        String::default()
    });
    let description = (*description_handle).clone();
    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    let name_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            name_handle.set(input.value());
        }
    });

    let code_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            code_handle.set(input.value());
        }
    });

    let rustacean_id_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlSelectElement>();
        if let Some(input) = target {
            rustacean_id_handle.set(input.value());
        }
    });

    let version_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            version_handle.set(input.value());
        }
    });

    let description_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();
        if let Some(input) = target {
            description_handle.set(input.value());
        }
    });

    // The undscore names are cloned values to moving into the blocks below.
    //
    let name_ = name.clone();
    let code_ = code.clone();
    let version_ = version.clone();
    let rustacean_id_ = rustacean_id.clone();
    let description_ = description.clone();
    let crate_ = props.cr8.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        let name_ = name_.clone();
        let code_ = code_.clone();
        let crate_ = crate_.clone();
        let version_ = version_.clone();
        let description_ = description_.clone();
        let error_handle_ = error_message_handle.clone();
        let navigator_ = navigator.clone();
        let user_ctx_ = current_user_ctx.clone();
        match &user_ctx_.token {
            Some(token) => {
                let rustacean_id = rustacean_id_.parse::<i32>();
                let token = token.clone();
                match rustacean_id {
                    Ok(rustacean_id) => spawn_local(async move {
                        if let Some(cr8) = crate_ {
                            match api_crate_update(
                                &token,
                                cr8.id.clone(),
                                name_,
                                code_,
                                rustacean_id,
                                version_,
                                description_,
                            )
                            .await
                            {
                                Ok(_) => navigator_.push(&Route::Crates),
                                Err(e) => error_handle_.set(e.to_string()),
                            }
                        } else {
                            match api_crate_create(
                                &token,
                                name_,
                                code_,
                                rustacean_id,
                                version_,
                                description_,
                            )
                            .await
                            {
                                Ok(_) => navigator_.push(&Route::Crates),
                                Err(e) => error_handle_.set(e.to_string()),
                            }
                        }
                    }),
                    Err(_) => error_handle_.set("Cannot parse rustacean ID".to_string()),
                }
            }
            None => error_handle_.set("Session expired. Please login again".to_string()),
        }
    });

    let options = props
        .authors
        .iter()
        .map(|r| {
            (
                AttrValue::from(r.id.to_string()),
                AttrValue::from(r.name.clone()),
            )
        })
        .collect::<Vec<(AttrValue, AttrValue)>>();
    html! {
        <form onsubmit={onsubmit}>
            if error_message.len() > 0 {
                <Alert alert_type={"danger"} message={error_message} />
            }
            <div class="mb-3">
                <Input
                    input_type="text"
                    name="code"
                    label="Code"
                    value={code}
                    onchange={code_changed}
                />
            </div>
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
                    input_type="text"
                    name="version"
                    label="Version"
                    value={version}
                    onchange={version_changed}
                />
            </div>
            <div class="mb-3">
                <Select
                    name="author"
                    label="Author"
                    value={rustacean_id}
                    onchange={rustacean_id_changed}
                    options={options}
                />
            </div>
            <div class="mb-3">
                <Textarea
                    name="description"
                    label="Description"
                    value={description}
                    onchange={description_changed}
                />
            </div>
            <button type="submit" class="btn btn-primary">{"Save"}</button>
        </form>
    }
}
