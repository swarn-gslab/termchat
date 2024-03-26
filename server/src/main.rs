pub mod login;
pub mod message;
use crate::{
    login::{login, online_status, SessionDatabase, UserDatabase},
    message::{handle_receiver_message, handle_send_message, start_conversation, InMemoryDatabase},
};
use axum::{extract::Extension, routing::post, Router};
use std::io::Write;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter(Some(env!("CARGO_BIN_NAME")), log::LevelFilter::Debug)
        .format(|buf, record| {
            let file = record.file().unwrap_or("unknown");
            let line = record.line().unwrap_or(0);
            let level = record.level();
            let args = record.args();
            match file.starts_with("") {
                true => writeln!(buf, "{file}:{line} {level} : {args}"),
                false => Ok(()),
            }
        })
        .init();

    let user_db = Arc::new(UserDatabase::new());
    let session_db: SessionDatabase = Arc::new(Mutex::new(HashMap::new()));
    // let user_database = Arc::new(Mutex::new(UserDatabase::new()));
    let db = Arc::new(InMemoryDatabase::new());
    let app = Router::new()
        .route("/login", post(login))
        .route("/status", post(online_status))
        .layer(Extension(user_db))
        // .route("/sender", post(handle_sender_request))
        // .route("/receiver/:userid", get(handle_receiver_request))
        .route("/start_conversation", post(start_conversation))
        .route("/send_message", post(handle_send_message))
        .route("/receive_message", post(handle_receiver_message))
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
