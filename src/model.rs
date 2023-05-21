
use crate::{Error, Result, ctx::Ctx};
use axum::Json;
use serde::{Deserialize, Serialize};
use tower_cookies::cookie::time::PrimitiveDateTime;
use std::{sync::{Arc, Mutex}, clone};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: u64,
    pub cid: u64,
    pub title: String,
    pub message: Vec<Message>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    sender: String,
    content: String,
}

#[derive(Deserialize)]
pub struct ChatForCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    chats_store: Arc<Mutex<Vec<Option<Chat>>>>
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self>{
        Ok(Self {
            chats_store: Arc::default()
        })
    }
}

// CRUD Implementation 
impl ModelController {
    pub async fn create_chat(&self, ctx: Ctx, chat_fc:ChatForCreate) -> Result<Chat>{
        let mut store = self.chats_store.lock().unwrap();

        let id = store.len() as u64;
        let chat = Chat {
            id,
            cid: ctx.user_id(),
            title: chat_fc.title,
            message: Vec::new(),
        };

        store.push(Some(chat.clone()));
        Ok(chat)
    }

    pub async fn list_chats(&self, _ctx: Ctx) -> Result<Vec<Chat>> {
        let store = self.chats_store.lock().unwrap();
        let chats = store.iter().filter_map(|t| t.clone()).collect();
        Ok(chats)
    }

    pub async fn delete_chat(&self, _ctx: Ctx, id: u64) -> Result<Chat> {
        let mut store = self.chats_store.lock().unwrap();
        let chat = store.get_mut(id as usize).and_then(|t| t.take());
        chat.ok_or(Error::TicketDeleteFailIdNotFound {id})
    }
}
