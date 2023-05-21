#! [allow(unused)]

use crate::model::ModelController;

pub use self::error::{Error, Result};
use std::collections::HashMap;
//testing to see if this workd, its wokring brobro 
//this is added from branch 1
use std::net::SocketAddr;
use axum::extract::Path;
use tokio::sync::broadcast;
use tower_cookies::CookieManagerLayer;
use std::ffi::OsStr;
use axum::{Router, middleware};
use axum::extract::Query;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use serde::Deserialize;
use tower_http::services::ServeDir;
use std::{
    collections::{HashSet},
    sync::{Arc, Mutex},
};
mod error;
mod model;
mod web;
mod ctx;

#[tokio::main]
async fn main() -> Result<()>{
    //initialize model controller
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_chats::routes(mc.clone()).route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    //Layers bottom to top, middlewares that require cookies need to be above cookiemanager.
    let routes_all: Router = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());
    
    // --- START SERVER
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTENING on {addr} \n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // --- 
    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}

fn routes_static() -> Router {
    Router::new()
    .route("/chat", get(index))
    .nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello",get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

// e.g., `/hello?name=Jen` argument of the function mapping to the URL query paramters - QUERY EXTRACTOR
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name: &str = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}


// e.g., '/hello2/mike' - value in the PATH not Query Parameter - PATH EXTRACTOR
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
	println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
	Html(format!("Hello2 <strong>{name}</strong>"))
}

async fn index() -> Html<&'static str> {
    Html(std::include_str!("../chat.html"))
}