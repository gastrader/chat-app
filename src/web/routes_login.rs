use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

//JSON IS A BODY EXTRACTOR
async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    //TODO: Implement real db/auth logi

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    //TODO: Set Cookies

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