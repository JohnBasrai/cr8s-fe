use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <h1>{"Page not found"}</h1>
    }
}
