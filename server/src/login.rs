
use axum::extract::Json;
use axum::http:: StatusCode;
use axum::Extension;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    message: String,
}

#[axum_macros::debug_handler]
pub async fn login(
    Extension(database): Extension<Arc<UserDatabase>>,
    Json(request_user): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    if let Some(user) = database.get(&request_user.username) {
        if user.password == request_user.password {
            // Authentication successful
            let response = LoginResponse {
                message: "Login successful".to_string(),
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

#[derive(Clone)]
pub struct User {
    username: String,
    password: String,
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
                username: "user1".to_string(),
                password: "password1".to_string(),
            },
        );
        users.insert(
            "user2".to_string(),
            User {
                username: "user2".to_string(),
                password: "password2".to_string(),
            },
        );
        users.insert(
            "user3".to_string(),
            User {
                username: "user3".to_string(),
                password: "password3".to_string(),
            },
        );
        Self { users }
    }

    pub fn get(&self, username: &str) -> Option<&User> {
        self.users.get(username)
    }
}
