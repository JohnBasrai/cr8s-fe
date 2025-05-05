use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub alert_type: AttrValue,
    pub message: AttrValue,
}

#[function_component(Alert)]
pub fn alert(props: &Props) -> Html {
    html! {
        <div class={format!("alert alert-{}", props.alert_type)} role="alert">
            {props.message.clone()}
        </div>
    }
}
