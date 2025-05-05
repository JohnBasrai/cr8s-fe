use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::crate_form::CrateForm;
use crate::components::header::Header;
use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;
use crate::hooks::{use_crate, use_rustaceans};
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub crate_id: i32,
}

#[function_component(CratesEdit)]
pub fn crates_edit(props: &Props) -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

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
                                <CrateEditForm
                                    crate_id={props.crate_id}
                                    token={token.clone()}
                                />
                            </Suspense>
                        </div>
                    </div>
                </div>
            }
        }
        None => html! {
            <Redirect<Route> to={Route::Login} />
        },
    }
}

#[derive(Properties, PartialEq)]
struct CrateEditFormProps {
    pub crate_id: i32,
    pub token: AttrValue,
}

#[function_component(CrateEditForm)]
fn crate_edit_form(props: &CrateEditFormProps) -> HtmlResult {
    let cr8 = use_crate(props.token.as_str(), props.crate_id)?;
    let rustaceans = use_rustaceans(props.token.as_str())?;
    Ok(html! {
        <CrateForm cr8={cr8} authors={rustaceans} />
    })
}
