use yew::prelude::*;

use crate::components::header::Header;
use crate::components::sidebar::Sidebar;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
            <div class="row">
                <div class="col-sm-auto">
                    <Sidebar />
                </div>
                <div class="col mt-3">
                    <Header />
                    {"Have a great day!"}
                </div>
            </div>
        </div>
    }
}