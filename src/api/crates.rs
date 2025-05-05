use gloo_net::http::Request;
use gloo_net::Error;
use serde::Deserialize;
use serde_json::json;

use super::app_host;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub name: String,
    pub code: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: String,
}

pub async fn api_crate_show(token: &String, id: i32) -> Result<Crate, Error> {
    let response = Request::get(&format!("{}/crates/{}", &app_host(), id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    response.json::<Crate>().await
}

pub async fn api_crates(token: &String) -> Result<Vec<Crate>, Error> {
    let response = Request::get(&format!("{}/crates", &app_host()))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    response.json::<Vec<Crate>>().await
}

pub async fn api_crate_create(
    token: &String,
    name: String,
    code: String,
    rustacean_id: i32,
    version: String,
    description: String,
) -> Result<Crate, Error> {
    let response = Request::post(&format!("{}/crates", &app_host()))
        .header("Authorization", &format!("Bearer {}", token))
        .header("Accept", "application/json")
        .json(&json!({
            "name": name,
            "code": code,
            "rustacean_id": rustacean_id,
            "version": version,
            "description": description
        }))?
        .send()
        .await?;

    response.json::<Crate>().await
}

pub async fn api_crate_update(
    token: &String,
    id: i32,
    name: String,
    code: String,
    rustacean_id: i32,
    version: String,
    description: String,
) -> Result<Crate, Error> {
    let response = Request::put(&format!("{}/crates/{}", &app_host(), id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "name": name,
            "code": code,
            "rustacean_id": rustacean_id,
            "version": version,
            "description": description
        }))?
        .send()
        .await?;

    response.json::<Crate>().await
}

pub async fn api_crate_delete(token: &String, id: i32) -> Result<(), Error> {
    let _ = Request::delete(&format!("{}/crates/{}", &app_host(), id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    Ok(())
}
