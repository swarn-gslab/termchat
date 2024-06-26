extern crate uuid;
use axum::extract::Json;
use axum::http::StatusCode;
use axum::Extension;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
// use tokio::sync::Mutex;
use std::fmt;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    userid: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
}

pub type SessionDatabase = Arc<Mutex<HashMap<String, Session>>>;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Session {
    pub token: String,
    pub userid: String,
}
impl Session {
    pub fn new(token: String, userid: String) -> Self {
        Session { token, userid }
    }
}

pub async fn get_session(session_db: &SessionDatabase, token: &str) -> Option<Session> {
    let sessions = session_db.lock().await;
    sessions.get(token).cloned()
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Assuming `id` is a field you want to display
        write!(f, "Session ID: {}", self.userid)
    }
}
#[axum_macros::debug_handler]
pub async fn login(
    Extension(database): Extension<Arc<UserDatabase>>,
    Extension(sessions): Extension<SessionDatabase>,
    Json(request_user): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    if let Some(user) = database.get(&request_user.userid) {
        if user.password == request_user.password {
            // Authentication successful
            info!("Client is authenticated");
            let token = Uuid::new_v4().to_string();
            let session = Session {
                token: token.clone(),
                userid: request_user.userid.clone(),
            };
            sessions.lock().await.insert(token.clone(), session);
            log::info!(
                "Session created - Token: {}, UserID: {}",
                token,
                request_user.userid
            );

            let response = LoginResponse { token: token };
            Ok(Json(response))
        } else {
            // Incorrect password
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        // User not found
        Err(StatusCode::UNAUTHORIZED)
    }
}

// here we check user online and offline status
#[derive(Debug, Deserialize, Serialize)]
pub struct CheckOnlineStatusByUserIdRequest {
    userid: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CheckOnlineStatusByUserIdResponse {
    online: bool,
}
pub async fn online_status(
    Extension(database): Extension<Arc<UserDatabase>>,
    Json(request): Json<CheckOnlineStatusByUserIdRequest>,
) -> Result<Json<CheckOnlineStatusByUserIdResponse>, StatusCode> {
    // let database = database.lock().unwrap();
    info!(
        "Received online status request for UserId {}",
        request.userid
    );
    match database.is_online(&request.userid) {
        Some(online) => {
            info!(
                "user{} is {}",
                request.userid,
                if online { "online" } else { "offline" }
            );
            Ok(Json(CheckOnlineStatusByUserIdResponse { online }))
        }
        None => {
            error!("user {} is not online", request.userid);
            Err(StatusCode::NOT_FOUND)
        }
    }
}
// handle the get request

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    userid: String,
    password: String,
    online: bool,
}

#[derive(Clone)]
pub struct UserDatabase {
    users: std::collections::HashMap<String, User>,
}

impl UserDatabase {
    pub fn new() -> Self {
        let mut users = std::collections::HashMap::new();
        users.insert(
            "Swarnjit".to_string(),
            User {
                userid: "Swarnjit".to_string(),
                password: "GS1".to_string(),
                online: false,
            },
        );
        users.insert(
            "Sanjeev".to_string(),
            User {
                userid: "Sanjeev".to_string(),
                password: "GS2".to_string(),
                online: false,
            },
        );
        users.insert(
            "Kamlesh".to_string(),
            User {
                userid: "Kamlesh".to_string(),
                password: "GS3".to_string(),
                online: false,
            },
        );
        Self { users }
    }

    pub fn get(&self, userid: &str) -> Option<&User> {
        self.users.get(userid)
    }
    // here we check user is online or not
    pub fn is_online(&self, userid: &str) -> Option<bool> {
        self.users
            .values()
            .find(|user| user.userid == userid)
            .map(|user| user.online)
    }
}
