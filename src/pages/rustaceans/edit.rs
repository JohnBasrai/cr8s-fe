use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::components::header::Header;
use crate::components::rustacean_form::RustaceanForm;
use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;
use crate::hooks::use_rustacean;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub rustacean_id: i32,
}

#[function_component(RustaceansEdit)]
pub fn rustaceans_edit(props: &Props) -> Html {
    let current_user_ctx = use_context::<CurrentUserContext>().expect("Current user context is missing");

    match &current_user_ctx.token {
        Some(token) => {
            let loading = html! { <p>{"Loading ..."}</p> };

            html! {
                <div class="container">
                    <div class="row">
                        <div class="col-sm-auto">
                            <Sidebar />
                        </div>
                        <div class="col mt-3">
                            <Header />
                            <Suspense fallback={loading}>
                                <RustaceanEditForm 
                                    rustacean_id={props.rustacean_id}
                                    token={token.clone()}
                                />
                            </Suspense>
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


#[derive(Properties, PartialEq)]
struct RustaceanEditFormProps {
    pub rustacean_id: i32,
    pub token: AttrValue,
}

#[function_component(RustaceanEditForm)]
fn rustaceans_edit_form(props: &RustaceanEditFormProps) -> HtmlResult {
    let rustacean = use_rustacean(props.token.as_str(), props.rustacean_id.clone())?;

    Ok(html! {
        <RustaceanForm rustacean={rustacean} />
    })
}