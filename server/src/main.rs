pub mod login;
pub mod message;
use axum::middleware::AddExtension;
use axum::routing::get;
// use axum::routing::get; TODO:
use axum::Extension;
use axum::{routing::post, Router};
use login::{login, UserDatabase,online_status};
use message::{get_receiver_msg, send_message};
use std::sync::{Arc, Mutex};

use crate::message::InMemoryDatabase;

// use crate::message::create_message;
#[tokio::main]
async fn main() {
    let user_db = Arc::new(UserDatabase::new());
    // let user_database = Arc::new(Mutex::new(UserDatabase::new()));
    let db = Arc::new(InMemoryDatabase::new());
    let app = Router::new()
        .route("/login", post(login))
        .route("/status", post(online_status))
        .layer(Extension(user_db))
       
        // .layer(Extension(user_database)) 
        // here we add more routers
        .route("/send_message", post(send_message))
        .route("/get_receiver/:sender", get(get_receiver_msg))
        .layer(Extension(db));

    // .route("/", get(health_check))
    // TODO:
    // .route("/message", get(create_message)) // TODO:

    // Start the server
    let lis = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    let port = "3010";
    let address = "0.0.0.0";
    println!("Server Started on {address}:{port}");
    axum::serve(lis, app).await.unwrap();
}
