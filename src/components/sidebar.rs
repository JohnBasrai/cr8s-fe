use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let current_route = use_route::<Route>().expect("No current route defined");
    let home_classes = {
        if current_route == Route::Home {
            classes!("nav-link", "active")
        } else {
            classes!("nav-link")
        }
    };
    let rustacean_classes = {
        if current_route == Route::Rustaceans {
            classes!("nav-link", "active")
        } else {
            classes!("nav-link")
        }
    };
    let crate_classes = {
        if current_route == Route::Crates {
            classes!("nav-link", "active")
        } else {
            classes!("nav-link")
        }
    };

    html! {
        <nav class="navbar navbar-light">
            <ul class="nav navbar-nav">
                <li class="nav-item">
                    <Link<Route> to={Route::Home} classes={home_classes}>
                        {"Home"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Rustaceans} classes={rustacean_classes}>
                        {"Rustaceans"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Crates} classes={crate_classes}>
                        {"Crates"}
                    </Link<Route>>
                </li>
            </ul>
        </nav>
    }
}
