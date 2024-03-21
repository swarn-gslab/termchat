pub mod login;
pub mod message;
pub mod implemessage;
use axum::routing::get;
// use axum::routing::get; TODO:

use crate::{
    implemessage::{handle_conversation_request, receive_message, send_message, start_conversation, InMemoryDatabase}, login::{login, online_status, SessionDatabase, UserDatabase}
};
// use axum::Extension;
// use axum::{routing::post, Router};

use std::{collections::HashMap, sync::Arc};
use axum::{extract::Extension, routing::post, Router};
// use crate::message::create_message;
#[tokio::main]
async fn main() {
    if let Err(err) = tracing_subscriber::fmt()
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_level(true)
        .try_init()
    {
        eprintln!("Failed to initialize logger: {}", err);
    }

    let user_db = Arc::new(UserDatabase::new());
    let session_db: SessionDatabase = Arc::new(std::sync::Mutex::new(HashMap::new()));
    // let user_database = Arc::new(Mutex::new(UserDatabase::new()));
    let db = Arc::new(InMemoryDatabase::new());
    let app = Router::new()
        .route("/login", post(login))
        .route("/status", post(online_status))
        .layer(Extension(user_db))
        // .route("/sender", post(handle_sender_request))
        // .route("/receiver/:userid", get(handle_receiver_request))
        
        .route("/start_conversation", post(start_conversation))
        .route("/send_message",post(send_message))

        // .route("/conversation/request", post(handle_conversation_request))
        .route("/receive_message", get(receive_message))
        .layer(Extension(db))
        .layer(Extension(session_db));

    // Start the server
    let lis = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    let port = "3010";
    let address = "0.0.0.0";
    println!("Server Started on {address}:{port}");
    tracing::info!("Ready to connect User");
    axum::serve(lis, app).await.unwrap();
}
