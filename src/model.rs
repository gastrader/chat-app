
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{sync::{Arc, Mutex}, clone};

#[derive(Clone, Debug, Serialize)]
pub struct Chat {
    pub id: u64,
    pub title: String,
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
    pub async fn create_chat(&self, chat_fc:ChatForCreate) -> Result<Chat>{
        let mut store = self.chats_store.lock().unwrap();

        let id = store.len() as u64;
        let chat = Chat {
            id,
            title: chat_fc.title,
        };

        store.push(Some(chat.clone()));
        Ok(chat)
    }

    pub async fn list_chats(&self) -> Result<Vec<Chat>> {
        let store = self.chats_store.lock().unwrap();
        let chats = store.iter().filter_map(|t| t.clone()).collect();
        Ok(chats)
    }

    pub async fn delete_chat(&self, id: u64) -> Result<Chat> {
        let mut store = self.chats_store.lock().unwrap();
        let chat = store.get_mut(id as usize).and_then(|t| t.take());
        chat.ok_or(Error::TicketDeleteFailIdNotFound {id})
    }
}
