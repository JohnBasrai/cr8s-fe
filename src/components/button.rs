use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub button_type: AttrValue,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let classes = classes!("btn", format!("btn-{}", props.button_type));
    match props.onclick.clone() {
        Some(callback) => html! {
            <button type="submit" class={classes} onclick={callback}>
                {props.label.clone()}
            </button>
        },
        None => html! {
            <button type="submit" class={classes}>
                {props.label.clone()}
            </button>
        },
    }
}
