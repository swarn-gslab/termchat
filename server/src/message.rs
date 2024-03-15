use axum::{body::{self, Body}, Extension};
use rustc_serialize::json::Json;
/*
use std::{collections::HashMap, sync::{Arc, Mutex}};

use serde::{Deserialize,Serialize};
use serde_json::error;

// use self database::(UserDatabase,Response);

#[derive(Clone,Debug)]
pub struct Message {
    pub token: String,
    pub contents: String,
}

#[derive(Default)]
#[allow(dead_code)]
pub struct InMemoryDatabase {
    message: Arc<Mutex<HashMap<String, Vec<Message>>>>,
}

impl InMemoryDatabase {
    pub fn new()->Self {InMemoryDatabase{
        message: Arc::new(Mutex::new(HashMap::new())),
    }
}

    pub fn add_message(&self, token: String, contents: String) {
        let mut message = self.message.lock().unwrap();
        let user_message = message.entry(token.clone()).or_insert(Vec::new());
        user_message.push(Message {
            token,
            contents,
        });

    }

    fn get_messages(&self, token: String) -> Option<Vec<Message>> {
        let  message = self.message.lock().unwrap();
        message
           .get(&token)
           .map(|m| m.to_vec())

    }
}



pub fn create_message(
    username:&str,
    new_message:&Message,
    database:&InMemoryDatabase,
) ->Result<(),error::Error>{
     database.add_message(username.to_string(), new_message.clone().contents);
     Ok(())

}

// convert messsage struct into json format

pub fn to_json(new_message:&Message) ->String{
    format!(
        "{{\"token\": \"{}\", \"contents\":\"{}\"}}",
        new_message.token, new_message.contents,
    )

}
*/
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

pub async fn send_message(Extension(db):Extension<Arc<InMemoryDatabase>>,body:String) ->String{
    let message :Message = serde_json::from_str(&body).expect("failed to Deserialize message");
    db.send_message(message);
    json!({"message": "Message sent"}).to_string()
}
pub async fn get_receiver(Extension(db):Extension<Arc<InMemoryDatabase>>,sender:String)->String{
   if let Some(receiver) = db.get_receiver(&sender){
    json!({"receiver": receiver}).to_string()
    }else{
        json!({"error":"Receiver notfound"}).to_string()
    }
}