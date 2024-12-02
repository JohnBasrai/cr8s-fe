use gloo_net::http::Request;
use gloo_net::Error;
use serde::Deserialize;
use serde_json::json;

use super::APP_HOST;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

pub async fn api_rustaceans(token: &String) -> Result<Vec<Rustacean>, Error> {
    let response = Request::get(&format!("{}/rustaceans", APP_HOST))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    response.json::<Vec<Rustacean>>().await
}

pub async fn api_rustacean_show(token: &String, id: i32) -> Result<Rustacean, Error> {
    let response = Request::get(&format!("{}/rustaceans/{}", APP_HOST, id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    response.json::<Rustacean>().await
}

pub async fn api_rustacean_create(
    token: &String,
    name: String,
    email: String,
) -> Result<Rustacean, Error> {
    let response = Request::post(&format!("{}/rustaceans", APP_HOST))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "name": name,
            "email": email
        }))?
        .send()
        .await?;

    response.json::<Rustacean>().await
}

pub async fn api_rustacean_update(
    token: &String,
    id: i32,
    name: String,
    email: String,
) -> Result<Rustacean, Error> {
    let response = Request::put(&format!("{}/rustaceans/{}", APP_HOST, id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "name": name,
            "email": email
        }))?
        .send()
        .await?;

    response.json::<Rustacean>().await
}

pub async fn api_rustacean_delete(token: &String, id: i32) -> Result<(), Error> {
    let _ = Request::delete(&format!("{}/rustaceans/{}", APP_HOST, id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    Ok(())
}
