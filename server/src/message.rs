use axum::body::to_bytes;
use axum::Extension;
use axum::{body::Body, extract::Request, http};
use hyper::{Response, StatusCode};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
extern crate serde_json;
#[derive(Debug, Clone, Serialize, Deserialize)]
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
            content: "your messsage is received".to_string(),
        };
        info!(
            "Message send from {} to {}:{}",
            response.sender, response.receiver, response.content
        );
        let _json_response = serde_json::to_string(&response).unwrap();
    }
    pub fn get_receiver(&self, sender: &str) -> Option<String> {
        let chat = self.chat.lock().unwrap();
        chat.get(sender).map(|receiver| receiver.clone())
    }
}

pub async fn send_message(Extension(db): Extension<Arc<InMemoryDatabase>>, body: String) -> String {
    let message: Message = match serde_json::from_str(&body) {
        Ok(message) => message,
        Err(err) => {
            error!("Failed to deserialize message: {}", err);
            return json!({"error": "Failed to deserialize message"}).to_string();
        }
    };

    info!("Received message: {:?}", message);

    db.send_message(message.clone());

    info!("Message sent successfully");

    serde_json::json!({"message": "Message sent"}).to_string()
}
pub async fn get_receiver_msg(
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    sender: String,
) -> String {
    if let Some(receiver) = db.get_receiver(&sender) {
        info!("Receiver found for sender {}: {}", sender, receiver);
        serde_json::json!({"receiver": receiver}).to_string()
    } else {
        error!("Receiver not found for sender {}", sender);
        json!({"error":"Receiver notfound"}).to_string()
    }
}

pub async fn handle_sender_request(
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    req: Request<Body>,
) -> Result<Response<Body>, StatusCode> {
    match req.method() {
        &http::Method::POST => {
            let body = to_bytes(req.into_body(), usize::MAX)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let body_str = String::from_utf8_lossy(&body).into_owned();
            info!("Received POST request from sender: {}", body_str);

            // Send message
            let response = send_message(Extension(db.clone()), body_str).await;
            Ok(Response::new(Body::from(response)))
        }
        _ => {
            error!("Received unsupported HTTP method for sender route");
            Err(StatusCode::METHOD_NOT_ALLOWED)
        }
    }
}

pub async fn handle_receiver_request(
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    req: Request<Body>,
) -> Result<Response<Body>, StatusCode> {
    match req.method() {
        &http::Method::GET => {
            let sender = req.uri().path().trim_start_matches("/receiver/");
            info!("Received GET request for receiver: {}", sender);

            let response = get_receiver_msg(Extension(db.clone()), sender.to_string()).await;
            info!("Sending response for receiver: {}", response);

            Ok(Response::new(Body::from(response)))
        }
        _ => {
            error!("Received unsupported HTTP method for receiver route");
            Err(StatusCode::METHOD_NOT_ALLOWED)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_message() {
        let db = InMemoryDatabase::new();
        let message = Message {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            content: "Hello, Bob!".to_string(),
        };

        db.send_message(message.clone());

        let chat = db.chat.lock().unwrap();
        assert_eq!(chat.len(), 1);
        assert_eq!(chat.get(&message.sender), Some(&message.receiver));
    }

    #[test]
    fn test_get_receiver_existing() {
        let db = InMemoryDatabase::new();
        let sender = "Alice";
        let receiver = "Bob";
        let message = Message {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            content: "Hello, Bob!".to_string(),
        };

        {
            let mut chat = db.chat.lock().unwrap();
            chat.insert(sender.to_string(), receiver.to_string());
        }

        let result = db.get_receiver(sender);
        assert_eq!(result, Some(receiver.to_string()));
    }

    #[test]
    fn test_get_receiver_non_existing() {
        let db = InMemoryDatabase::new();
        let sender = "Alice";

        let result = db.get_receiver(sender);
        assert_eq!(result, None);
    }
}
