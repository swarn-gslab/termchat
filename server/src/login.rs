use axum::extract::Json;
use axum::http::StatusCode;
use axum::Extension;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    userid: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    message: String,
    token: String,
}

#[axum_macros::debug_handler]
pub async fn login(
    Extension(database): Extension<Arc<UserDatabase>>,
    Json(request_user): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    if let Some(user) = database.get(&request_user.userid) {
        if user.password == request_user.password {
            // Authentication successful
            info!("Client is authenticated");
            let response = LoginResponse {
                message: "Login successful".to_string(),
                token: user.token.clone(),
            };
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
    token: String,
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
            "user1".to_string(),
            User {
                userid: "user1".to_string(),
                password: "password1".to_string(),
                token: "n2739271027012hjasvda".to_string(),
                online: false,
            },
        );
        users.insert(
            "user2".to_string(),
            User {
                userid: "user2".to_string(),
                password: "password2".to_string(),
                token: "vdha28736bz2321hsad63g".to_string(),
                online: false,
            },
        );
        users.insert(
            "user3".to_string(),
            User {
                userid: "user3".to_string(),
                password: "password3".to_string(),
                token: "12jassan736bas7ajas".to_string(),
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
