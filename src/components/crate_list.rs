use crate::hooks::use_crates;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token: AttrValue,
}

#[function_component(CrateList)]
pub fn crate_list(props: &Props) -> HtmlResult {
    let crates = use_crates(&props.token)?;

    Ok(html! {
        <>
            <p>
                <Link<Route> to={Route::CratesAdd}>
                    {"+ Add new crate"}
                </Link<Route>>
            </p>
            <table class="table">
                <thead>
                    <th>{"ID"}</th>
                    <th>{"Code"}</th>
                    <th>{"Name"}</th>
                    <th>{"Rustacean ID"}</th>
                    <th>{"Version"}</th>
                    <th>{"Description"}</th>
                    <th>{"Created at"}</th>
                    <th>{"Operations"}</th>
                </thead>
                <tbody>
                {
                    crates.into_iter().map(|cr8| {
                        html! {
                            <tr>
                                <td>{cr8.id}</td>
                                <td>{cr8.code}</td>
                                <td>{cr8.name}</td>
                                <td>{cr8.rustacean_id}</td>
                                <td>{cr8.version}</td>
                                <td>{cr8.description}</td>
                                <td>
                                    <Link<Route>
                                        to={Route::CratesEdit { id: cr8.id }}
                                        classes="link-secondary"
                                    >
                                        {"edit"}
                                    </Link<Route>>
                                    <span class="mx-1">{"/"}</span>
                                    <Link<Route>
                                        to={Route::CratesDelete { id: cr8.id }}
                                        classes="link-danger"
                                    >
                                        {"delete"}
                                    </Link<Route>>
                                </td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
                </tbody>
            </table>
        </>
    })
}
