use axum::{
    body::{self, Body},
    Extension,
};
use rustc_serialize::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{mpsc::Sender, Arc, Mutex},
};
extern crate serde_json;
#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
    sender: String,
    receiver: String,
    content: String,
}

#[derive(Default)]
pub struct InMemoryDatabase {
    chat: Arc<Mutex<HashMap<String, String>>>,
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        InMemoryDatabase {
            chat: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn send_message(&self, message: Message) {
        let mut chat = self.chat.lock().unwrap();
        let sender = &message.sender;
        let receiver = &message.receiver;
        chat.insert(sender.clone(), receiver.clone());

        println!(
            "Message sent from {} to {}:{}",
            sender, receiver, message.content
        );

        self.send_response(message);
    }
    fn send_response(&self, message: Message) {
        let response = Message {
            sender: message.receiver.clone(),
            receiver: message.sender.clone(),
            content: "your messsage is to received".to_string(),
        };
        println!(
            "Message is send from {} to {}:{}",
            response.sender, response.receiver, response.content
        );
        let _json_response = serde_json::to_string(&response).unwrap();
    }
    pub fn get_receiver(&self, sender: &str) -> Option<String> {
        let mut chat = self.chat.lock().unwrap();
        chat.get(sender).map(|receiver| receiver.clone())
    }
}

pub async fn send_message(Extension(db): Extension<Arc<InMemoryDatabase>>, body: String) -> String {
    let message: Message = serde_json::from_str(&body).expect("failed to Deserialize message");
    db.send_message(message);
    json!({"message": "Message sent"}).to_string()
}
pub async fn get_receiver_msg(
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    sender: String,
) -> String {
    if let Some(receiver) = db.get_receiver(&sender) {
        json!({"receiver": receiver}).to_string()
    } else {
        json!({"error":"Receiver notfound"}).to_string()
    }
}
