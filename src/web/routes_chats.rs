use axum::{extract::{State, Path}, Json, Router, routing::{post, delete}};

use crate::model::{ModelController, Chat, ChatForCreate};
use crate::Result;

pub fn routes(mc: ModelController ) -> Router {
    Router::new()
        .route("/chats", post(create_chat).get(list_chats))
        .route("/chats/:id", delete(delete_chat))
        .with_state(mc)

}

async fn create_chat(State(mc): State<ModelController>, Json(chat_fc): Json<ChatForCreate>) -> Result<Json<Chat>> {
    println!("->> {:<12} - create_chat", "HANDLER");
    let chat = mc.create_chat(chat_fc).await?;
    Ok(Json(chat))
}

async fn list_chats (State(mc): State<ModelController>) -> Result<Json<Vec<Chat>>> {
    println!("->> {:<12} - list_chats", "HANDLER");
    let chats: Vec<Chat> = mc.list_chats().await?;
    Ok(Json(chats))
}

async fn delete_chat(State(mc): State<ModelController>, Path(id): Path<u64>) -> Result<Json<Chat>> {
    println!(">>> {:<15} - delete_chat", "HANDLER");
    let chat = mc.delete_chat(id).await?;
    Ok(Json(chat))
}