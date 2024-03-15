use axum::extract::Json;
use axum::http::StatusCode;
use axum::Extension;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
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
    if let Some(user) = database.get(&request_user.username) {
        if user.password == request_user.password {
            // Authentication successful
            println!("Client is authenticated");
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

// handle the get request

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    username: String,
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
                username: "user1".to_string(),
                password: "password1".to_string(),
                token: "n2739271027012hjasvda".to_string(),
                online: false,
            },
        );
        users.insert(
            "user2".to_string(),
            User {
                username: "user2".to_string(),
                password: "password2".to_string(),
                token: "vdha28736bz2321hsad63g".to_string(),
                online: false,
            },
        );
        users.insert(
            "user3".to_string(),
            User {
                username: "user3".to_string(),
                password: "password3".to_string(),
                token: "12jassan736bas7ajas".to_string(),
                online: false,
            },
        );
        Self { users }
    }
    // het by user is username
    pub fn get(&self, username: &str) -> Option<&User> {
        self.users
            .get(username)
            .or_else(|| self.users.get(&username.to_string()))
    }
    // here we check client is online or not
    pub fn set_online_status(&mut self, username: &str, online: bool) -> bool {
        println!("{}", username);
        if let Some(user) = self.users.get_mut(username) {
            user.online = online;
            true
        } else {
            false
        }
    }
    // here we get status of the client

    pub fn is_online(&self, username: &str) -> Option<bool> {
        println!("{}", username);
        if let Some(user) = self.users.get(username) {
            Some(user.online)
        } else {
            None
        }
    }
}
