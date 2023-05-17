#! [allow(unused)]

use std::net::SocketAddr;
use axum::extract::Path;
use std::ffi::OsStr;
use axum::Router;
use axum::extract::Query;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello: Router = Router::new()
    .route("/hello",get(handler_hello))
    .route("/hello2/:name", get(handler_hello2));

    // --- START SERVER

    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTENING on {addr} \n");
    axum::Server::bind(&addr).serve(routes_hello.into_make_service())
    .await
    .unwrap();
    // --- 

}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

// e.g., `/hello?name=Jen`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name: &str = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}


// e.g., '/hello2/mike'
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
	println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
	Html(format!("Hello2 <strong>{name}</strong>"))
}

