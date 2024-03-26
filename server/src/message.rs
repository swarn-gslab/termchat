use crate::login::{Session, SessionDatabase};
use axum::{body::Body, extract::Request, http};
use axum::{Extension, Json};
use axum_auth::AuthBearer;
use hyper::{Response, StatusCode};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    content: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
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
    chat: Arc<Mutex<HashMap<String, Vec<StoredMessage>>>>, // key = conbination of sender and receiver id & value = pair of messages
    connections: Arc<Mutex<HashMap<String, String>>>, // key = receiver userid and value = sender userid
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        InMemoryDatabase {
            chat: Arc::new(Mutex::new(HashMap::new())),
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_message(&self, sender_id: &str, receiver_id: &str, content: String) {
        let mut ids = vec![sender_id, receiver_id];
        ids.sort();
        let key = format!("{}_{}", ids[0], ids[1]);
        info!("key: {:?}", key);
        let message = StoredMessage {
            content,
            owner: sender_id.to_string(),
        };
        info!("message: {:?}", message);

        let mut chat = self.chat.lock().await;
        info!("chat: {:?}", *chat);
        chat.entry(key).or_insert_with(Vec::new).push(message);
        info!("chat: {:?}", *chat);
    }

    pub async fn get_messages(&self, user_id: &str) -> Result<Vec<String>, StatusCode> {
        let mut messages = Vec::new();
        let connections = self.connections.lock().await;
        info!("message is {:?}", messages);
        // Construct keys for both sent and received messages
        let sent_key_prefix = format!("{}_", user_id);
        info!("Sent key is {:?}", sent_key_prefix);
        let received_key_suffix = format!("_{}", user_id);
        info!("Received key is {:?}", received_key_suffix);
        let chat = self.chat.lock().await;
        info!("Chat is {:?}", chat);
        for (key, value) in chat.iter() {
            // Check if the conversation is still active
            let is_active = connections.contains_key(key.as_str())
                || connections.contains_key(key.split('_').rev().collect::<String>().as_str());
            info!("connection is active: {:?}", is_active);

            if is_active {
                for msg in value {
                    if key.ends_with(&received_key_suffix) && msg.owner != user_id {
                        messages.push(msg.content.clone());
                    }
                    // Add sent messages from the other user
                    else if key.starts_with(&sent_key_prefix) && msg.owner != user_id {
                        messages.push(msg.content.clone());
                    }
                }
            }
        }

        Ok(messages)
    }
}

pub async fn validate_token(
    req: &Request<Body>,
    token: &str,
    session_db: &SessionDatabase,
) -> bool {
    info!(
        "Received request: method={}, path={}",
        req.method(),
        req.uri().path()
    );
    if req.method() == &http::Method::POST && req.uri().path() == "/login" {
        info!("Allowing login request without token validation");
        return true;
    }
    let session_token = session_db.lock().await;
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
async fn extract_sender_id(token: &str, session_db: &SessionDatabase) -> Option<String> {
    let session_token = session_db.lock().await;
    session_token
        .get(token)
        .map(|session| session.userid.clone())
}

async fn check_user_online(
    userid: &str,
    sessions: &MutexGuard<'_, HashMap<String, Session>>,
) -> Option<String> {
    // let sessions = sessions.lock().unwrap();
    sessions
        .values()
        .find(|session| session.userid == userid)
        .map(|session| session.userid.clone())
}
pub async fn start_conversation(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    Extension(session_db): Extension<SessionDatabase>,
    Json(request): Json<Conversation>,
) -> Result<Json<String>, StatusCode> {
    let mut sender_id = String::new();
    info!("sender_id {:?}", sender_id);

    let session_db = session_db.lock().await;
    info!("session_db {:?}", session_db);
    if let Some(session) = session_db.get(&token) {
        let userid = &session.userid;
        sender_id = userid.clone();
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let receiver_userid = &request.receiver_id;
    info!("receiver_userid{:?}", receiver_userid);
    let is_receiver_online = check_user_online(receiver_userid, &session_db).await;
    info!("is_receiver_online{:?}", is_receiver_online);

    match is_receiver_online {
        Some(userid) => {
            let mut connections = db.connections.lock().await;

            let key = format!("{}_{}", sender_id, receiver_userid);
            info!("key: {:?}", key);
            let reverse_key = format!("{}_{}", receiver_userid, sender_id);
            info!("reverse_key: {:?}", reverse_key);

            if connections.contains_key(&key) || connections.contains_key(&reverse_key) {
                Ok(Json(format!(
                    "User: {} is Online and connection exists.",
                    userid
                )))
            } else {
                connections.insert(key, receiver_userid.to_string());
                connections.insert(reverse_key, sender_id.clone());
                Ok(Json(format!("User is Online and connection established.",)))
            }
        }
        None => Ok(Json("User is Offline.".to_string())),
    }
}

pub async fn insert_connection(
    sender_id: &str,
    receiver_id: &str,
    in_memory_db: &InMemoryDatabase,
) {
    let mut connections = in_memory_db.connections.lock().await;
    connections.insert(receiver_id.to_string(), sender_id.to_string());
    info!("{:?}", connections);
    info!("Inserted connection: {} -> {}", receiver_id, sender_id);
}

pub async fn handle_send_message(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    Extension(session_db): Extension<Arc<Mutex<HashMap<String, Session>>>>,
    Json(request): Json<Message>,
) -> Result<Response<Body>, StatusCode> {
    info!("validate_token");
    info!("Received token: {:?}", token);
    /* */
    let sender_id = match extract_sender_id(&token, &session_db).await {
        Some(id) => id,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    info!("Sender ID: {:?}", sender_id);
    info!("Sender ID before fetch: {:?}", sender_id);
    let content = request.content.to_lowercase();
    if content == "bye" {
        // Remove the connection between the users
        let mut connections = db.connections.lock().await;
        let receiver_id = connections
            .iter()
            .find_map(|(key, value)| {
                if value == &sender_id {
                    key.split('_').next().map(|id| id.to_string())
                } else {
                    None
                }
            })
            .ok_or(StatusCode::NOT_FOUND)?;

        let key = format!("{}_{}", sender_id, receiver_id);
        let reverse_key = format!("{}_{}", receiver_id, sender_id);
        let removed_key = connections.remove(&key);
        let removed_reverse_key = connections.remove(&reverse_key);
        info!("Connection removed: {} -> {}", key, removed_key.is_some());
        info!(
            "Connection removed: {} -> {}",
            reverse_key,
            removed_reverse_key.is_some()
        );
        return Ok(Response::new(Body::from("Connection ended.")));
    }

    // Proceed with adding the message as usual
    let connections = db.connections.lock().await;
    let receiver_id = connections
        .iter()
        .find_map(|(key, value)| {
            if value == &sender_id {
                key.split('_').next().map(|id| id.to_string())
            } else {
                None
            }
        })
        .ok_or(StatusCode::NOT_FOUND)?;
    db.add_message(&sender_id, &receiver_id, content).await;
    Ok(Response::new(Body::from("Message sent successfully")))
}

pub async fn handle_receiver_message(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<Arc<InMemoryDatabase>>,
    Extension(session_db): Extension<SessionDatabase>,
    Json(_request): Json<Conversation>,
) -> Result<Response<Body>, StatusCode> {
    info!("Connections{:?}", *session_db);
    let receiver_id = match extract_sender_id(&token, &session_db).await {
        Some(id) => id,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    info!("Sender ID: {:?}", receiver_id);
    let messages = db
        .get_messages(&receiver_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    info!("Messages: {:?}", messages);
    let json_response = json!({ "messages": messages });
    Ok(Response::new(Body::from(json_response.to_string())))
}
