use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::components::header::Header;
use crate::components::rustacean_list::RustaceanList;
use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;

#[function_component(Rustaceans)]
pub fn rustaceans() -> Html {
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
                                <RustaceanList token={token.clone()} />
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