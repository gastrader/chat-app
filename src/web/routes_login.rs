use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};

use crate::{Error, Result, web};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

//JSON IS A BODY EXTRACTOR
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    //TODO: Implement real db/auth logi

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    //TODO: Implement real auth-token generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    //Success Body.
    let body = Json(json!({
        "result": {
            "success":true
        }
    }));
    Ok(body)
    
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}