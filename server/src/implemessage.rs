use crate::login::{Session, SessionDatabase};
use axum::body::to_bytes;
use axum::Extension;
use axum::{body::Body, extract::Request, http};
use axum_auth::AuthBearer;
use hyper::{Response, StatusCode};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
   sender:String,
   content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationRequest {
    receiver: String,
}

#[derive(Default, Debug)]
pub struct InMemoryDatabase {
    chat: Arc<Mutex<HashMap<String, String>>>,
    connections: Arc<Mutex<HashMap<String, HashSet<String>>>>,

}

impl InMemoryDatabase {
    pub fn new() -> Self {
        InMemoryDatabase {
            chat: Arc::new(Mutex::new(HashMap::new())),
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn send_message(&self, sender: &str, receiver: &str, message: &str) {
        let mut chat = self.chat.lock().unwrap();
        let key = format!("{}:{}", sender, receiver);

        chat.insert(key.clone(), message.to_string());
        let mut connections = self.connections.lock().unwrap();
        let entry = connections.entry(sender.to_string()).or_default();
        entry.insert(receiver.to_string());

        println!(
            "Message sent from {} to {}:{}",
            sender, receiver, message
        );

        self.send_response(sender, receiver, message);
    }

    fn send_response(&self, sender: &str, receiver: &str, message: &str) {
        let response = Message {
            sender:sender.to_string(),
            content: message.to_string(),
        };
        info!(
            "Message send from {} to {}:{}",
            sender, receiver, message
        );
        let _json_response = serde_json::to_string(&response).unwrap();
    }

    pub fn get_message(&self, sender: &str, receiver: &str) -> Option<String> {
        let chat = self.chat.lock().unwrap();
        let key = format!("{}:{}", sender, receiver);
        chat.get(&key).cloned()
    }

    pub fn initiate_conversation(&self, sender: &str, receiver: &str) {
        let mut chat = self.chat.lock().unwrap();
        chat.insert(sender.to_string(), receiver.to_string());
    }

    pub fn get_messages_for_receiver(&self, receiver: &str) -> Vec<Message> {
        let chat = self.chat.lock().unwrap();
        let mut messages = Vec::new();

        for (key, content) in chat.iter() {
            let parts: Vec<&str> = key.split(':').collect();
            if parts.len() == 2 && parts[1] == receiver {
                messages.push(Message {
                    sender: parts[0].to_string(),
                    content: content.to_string(),
                });
            }
        }

        messages
    }
    pub fn establish_connection(sender: &str, receiver: &str, db: &Arc<InMemoryDatabase>) {
        let mut connections = db.connections.lock().unwrap();
        let entry = connections.entry(receiver.to_string()).or_default();
        entry.insert(sender.to_string());
    }
    
    pub fn is_connected(sender: &str, receiver: &str, db: &Arc<InMemoryDatabase>) -> bool {
        let connections = db.connections.lock().unwrap();
        if let Some(receivers) = connections.get(sender) {
            return receivers.contains(receiver);
        }
        false
    }
}
pub fn validate_token(req: &Request<Body>, token: &str, session_db: &SessionDatabase) -> bool {
    info!(
        "Received request: method={}, path={}",
        req.method(),
        req.uri().path()
    );
    if req.method() == &http::Method::POST && req.uri().path() == "/login" {
        info!("Allowing login request without token validation");
        return true;
    }
    let session_token = session_db.lock().unwrap();
    if session_token.contains_key(token) {
        info!("Token validation Successful");
        return true;
    } else {
        warn!("token validation Failed");
        return false;
    }
}



pub async fn start_conversation(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    Extension(session_db): Extension<SessionDatabase>,
    req: Request<Body>,
    
) -> Result<Response<Body>, StatusCode> {
    let sender = &token;
    let receiver = String::from_utf8_lossy(&to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
        .into_owned();

        InMemoryDatabase::establish_connection(sender, &receiver, &db);


    Ok(Response::new(Body::empty()))
}

pub async fn send_message(
   
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    Extension(session_db): Extension<SessionDatabase>,
    req: Request<Body>,
    
) -> Result<Response<Body>, StatusCode> {
    if !validate_token(&req, &token, &session_db) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let sender = token; // Sender is the user who is logged in
    let body = to_bytes(req.into_body(), usize::MAX)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = String::from_utf8_lossy(&body).into_owned();
    let receiver = body.trim(); // Extract the receiver's name from the request body

    db.send_message(&sender, receiver, "Hello, receiver!");
    

    Ok(Response::new(Body::empty()))
}

pub async fn handle_conversation_request(
    // AuthBearer(token): AuthBearer,
    // Extension(token): Extension<String>,
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    Extension(session_db): Extension<SessionDatabase>,
    req: Request<Body>,
   
) -> Result<Response<Body>, StatusCode> {
    if !validate_token(&req, &token, &session_db) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let sender = token; // Sender is the user who is logged in
    let body = to_bytes(req.into_body(), usize::MAX)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let conversation_request: ConversationRequest = match serde_json::from_slice(&body) {
        Ok(request) => request,
        Err(err) => {
            error!("Failed to deserialize conversation request: {}", err);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let receiver = conversation_request.receiver.clone();

    db.initiate_conversation(&sender, &receiver);

    Ok(Response::new(Body::empty()))
}

pub async fn receive_message(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    Extension(session_db): Extension<SessionDatabase>,
    req: Request<Body>,
) -> Result<Response<Body>, StatusCode> {
    if !validate_token(&req, &token, &session_db) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let sender = &token; // Extract sender from the token
    let receiver = &token; // In this case, receiver is the same as sender

    // Retrieve the message from the database
    let message = db.get_message(sender, receiver);

    // Construct the response
    // match message {
    //     Some(content) => {
    //         let response = Message { content };
    //         let json_response = serde_json::to_string(&response)
    //             .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    //         Ok(Response::builder()
    //             .status(StatusCode::OK)
    //             .body(Body::from(json_response))
    //             .unwrap())
    //     }
    //     None => {
    //         Ok(Response::builder()
    //             .status(StatusCode::NOT_FOUND)
    //             .body(Body::empty())
    //             .unwrap())
    //     }
    // }
    let response = serde_json::to_string(&message)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Response::new(Body::from(response)))
}

