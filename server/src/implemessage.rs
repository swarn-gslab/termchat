
use crate::login::SessionDatabase;
use axum::body::to_bytes;
use axum::{body::Body, extract::Request, http};
use axum:: Extension;
use axum_auth::AuthBearer;
use hyper::{Response, StatusCode};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    content: String,
    owner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    receiver_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiverUser {
    userid: String,
}

#[derive(Default, Debug)]
pub struct InMemoryDatabase {
    chat: Arc<Mutex<HashMap<String, Vec<Message>>>>, // key = conbination of sender and receiver id & value = pair of messages
    connections: Arc<Mutex<HashMap<String, String>>>, // key = receiver userid and value = sender userid
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
        info!("{:?}", chat);
        let key = format!("{}_{}", sender, receiver);
        let message_struct = Message {
            content: message.to_string(),
            owner: sender.to_string(),
        };
        chat.entry(key.clone())
            .or_insert(Vec::new())
            .push(message_struct);
        // Store messages in vector associated with key
        info!("message_struct is work");
    }

    pub fn send_response(&self, sender: &str, receiver: &str, content: &str) {
        let response_message = Message {
            content: content.to_string(),
            owner: sender.to_string(),
        };

        info!(
            "Response message sent from {} to {}: {}",
            sender, receiver, content
        );

        let json_response = serde_json::to_string(&response_message)
            .expect("Failed to serialize response message to JSON");

        println!("Response JSON: {}", json_response);
    }

    pub fn get_receiver(&self, receiver_id: &str) -> Vec<Message> {
        let mut messages = Vec::new();
        let chat = self.chat.lock().unwrap();
        for (key, value) in chat.iter() {
            let parts: Vec<&str> = key.split('_').collect();
            if parts.len() == 2 && parts[1] == receiver_id {
                messages.extend(value.clone());
            }
            info!("{:?}", parts);
        }
        messages
    }

}

pub async fn send_message(
    db: Extension<Arc<InMemoryDatabase>>,
    body: String,
    receiver: &str,
) -> String {
    let message: Message = match serde_json::from_str(&body) {
        Ok(message) => message,
        Err(err) => {
            error!("Failed to deserialize message: {}", err);
            return json!({"error": "Failed to deserialize message"}).to_string();
        }
    };
    info!("Received message: {:?}", message);

    db.send_message(&message.owner, receiver, &message.content);
    info!("{:?}", db);

    info!("Message sent successfully");

    json!({"message": "Message sent"}).to_string()
}

pub async fn get_receiver_msg(
    db: Extension<Arc<InMemoryDatabase>>,
    sender: String,
    receiver: String,
) -> Vec<Message> {
    // Construct the key to retrieve messages
    let key = format!("{}_{}", sender, receiver);

    // Lock the chat database and attempt to retrieve messages for the given key
    let chat = db.chat.lock().unwrap();
    if let Some(message_content) = chat.get(&key) {
        // Messages found, return them
        info!(
            "Messages found for sender {} and receiver {}: {:?}",
            sender, receiver, message_content
        );
        message_content.clone() // Assuming message_content is already a Vec<Message>
    } else {
        // No messages found for the given key
        error!(
            "No messages found for sender {} and receiver {}",
            sender, receiver
        );
        Vec::new() // Return an empty vector if no messages are found
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
// fn extract_receiver_id()
// this is for the extracting the userid from the token
fn extract_sender_id(token: &str, session_db: &SessionDatabase) -> Option<String> {
    let session_token = session_db.lock().unwrap();
    session_token
        .get(token)
        .map(|session| session.userid.clone())
}

/*
fn check_user_online(userid: &str, sessions: &MutexGuard<HashMap<String, Session>>) -> Option<String> {
    // let sessions = sessions.lock().unwrap();
    sessions.values()
        .find(|session| session.userid == userid)
        .map(|session| session.userid.clone())
}
pub async fn start_conversation(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    Extension(session_db): Extension<SessionDatabase>,
    Json(request): Json<Conversation>,
) -> Result<Json<String>, StatusCode> {
    let mut sender_id= String::new();
    info!("{:?}", sender_id);

    let session_db = session_db.lock().unwrap();
    if let Some(session) = session_db.get(&token) {
        let userid = &session.userid;
        sender_id = userid.clone();
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }

let receiver_userid = &request.receiver_id;
    let is_receiver_online = check_user_online(receiver_userid, &session_db);

    match is_receiver_online {
        Some(userid) => {
            insert_connection(&sender_id, receiver_userid, &db);

            Ok(Json(format!("User {} is Online.", userid)))
        },
        None => {
            Ok(Json("User is Offline.".to_string()))
        }
    }
}

fn insert_connection(sender_id: &str, receiver_id: &str, in_memory_db: &InMemoryDatabase) {
    let mut connections = in_memory_db.connections.lock().unwrap();
    connections.insert(receiver_id.to_string(), sender_id.to_string());
    info!("{:?}",connections);
    info!("Inserted connection: {} -> {}", receiver_id, sender_id);
}

*/
// the below is written  by arslan
pub async fn start_conversation(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    Extension(session_db): Extension<SessionDatabase>,
    req: Request<Body>,
) -> Result<Response<Body>, StatusCode> {
    info!("{:?}", session_db);
    let mut sender = String::new();

    let body_bytes = to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let session_db_lock = session_db.lock().unwrap();
    for session in session_db_lock.values() {
        if session.token == *token {
            sender = session.userid.clone();
        }
    }
    info!("{}", sender);

    let receiver: ReceiverUser =
        serde_json::from_slice(&body_bytes).map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut db_lock = db.connections.lock().unwrap();

    // Check if the receiver_user's user_id matches any user_id in the SessionDatabase
    let receiver_id = receiver.userid.clone();

    let valid_receiver = session_db_lock
        .values()
        .any(|session| session.userid == receiver_id);
    info!("{:?}", valid_receiver);

    if valid_receiver == true {
        match db_lock.insert(receiver_id, sender.to_string()) {
            Some(_) => {
                println!("Key already exists in the database");
                info!("{:?}", db_lock);
                Err(StatusCode::SEE_OTHER)
            }
            None => {
                println!("Successfully stored userid and receiverid in the database");
                info!("{:?}", db_lock);
                Ok(Response::new(Body::from("Success")))
            }
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn handle_send_message(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    Extension(session_db): Extension<SessionDatabase>,
    req: Request<Body>,
) -> Result<Response<Body>, StatusCode> {
    if !validate_token(&req, &token, &session_db) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    info!("validate_token");
    match req.method() {
        &http::Method::POST => {
            let body = to_bytes(req.into_body(), usize::MAX)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let body_str = String::from_utf8_lossy(&body).into_owned();
            info!("Received POST request from sender: {}", body_str);

            // Send message
            let response = send_message(Extension(db.clone()), body_str, "receiver_id").await;
            Ok(Response::new(Body::from(response)))
        }
        _ => {
            error!("Received unsupported HTTP method for sender route");
            Err(StatusCode::METHOD_NOT_ALLOWED)
        }
    }
}

pub async fn handle_receiver_message(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    Extension(session_db): Extension<SessionDatabase>,
    req: Request<Body>,
) -> Result<Response<Body>, StatusCode> {
    // Validate token
    if !validate_token(&req, &token, &session_db) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Extract receiver ID from request

    let receiver_id = session_db.lock().unwrap().get(&token).cloned();
    let receiver_id = match receiver_id {
        Some(receiver_id) => receiver_id,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    let receiver_id_str = receiver_id.to_string();
    let sender_id = match extract_sender_id(&token, &session_db) {
        Some(sender_id) => sender_id,
        None => {
            error!("Failed to extract sender ID");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let messages = get_receiver_msg(Extension(db.clone()), sender_id, receiver_id_str).await;

    // Serialize messages to JSON
    let response_body = serde_json::to_string(&messages).map_err(|err| {
        error!("Failed to serialize messages to JSON: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Create and return response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(response_body))
        .unwrap())
}
