
use axum::{
    extract::{ws::Message, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};

use futures::{
    sink::SinkExt,
    stream::{SplitSink, StreamExt},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
// use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;

// Data structure to store user credentials
#[derive(Debug, Clone)]
struct User {
    username: String,
    password: String,
}

// Global state to store user connections
type UserConnections = Arc<Mutex<HashMap<String, (SplitSink<WebSocketUpgrade, Message>, User)>>>;

#[tokio::main]
async fn main() {
    let user_connections = Arc::new(Mutex::new(HashMap::<
        String,
        (SplitSink<WebSocketUpgrade, Message>, User),
    >::new()));
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(user_connections)
        .clone();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3010")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    user_connections: axum::extract::Extension<UserConnections>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, user_connections.clone()))
}


async fn handle_socket(
    mut socket: WebSocket,
    user_connections: axum::extract::Extension<UserConnections>,
) {
    let (sender, mut receiver) = socket.split();

    // Receive login credentials from the client
    if let Some(Ok(Message::Text(credentials))) = receiver.next().await {
        let parts: Vec<&str> = credentials.trim().split(' ').collect();
        if parts.len() == 2 {
            let username = parts[0].to_string();
            let password = parts[1].to_string();

            let mut user_connections = user_connections.lock().unwrap();
            if authenticate_user(&username, &password) {
                // Placeholder user or default user
                let user = User {
                    username: username.clone(),
                    password: password.clone(),
                };

                // Create a WebSocketStream instance
                let websocket_stream = WebSocketStream::from_raw_socket(
                    socket,
                    tokio_tungstenite::tungstenite::protocol::Role::Server,
                    None,
                );

                // Store the user's WebSocket stream and the user itself
                user_connections
                    .entry(username.clone())
                    .or_insert_with(|| (websocket_stream, user.clone()));

                // Send message to the user indicating successful login
                let _ = sender
                    .send(Message::Text("Successfully logged in!".into()))
                    .await;

                // Listen for incoming messages
                while let Some(msg) = receiver.next().await {
                    if let Ok(msg) = msg {
                        if let Message::Text(text) = msg {
                            println!("Received message: {}", text);

                            // TODO: Restructure data so that we don't have to split
                            
                            // Parse the message to get recipient username and content
                            let parts: Vec<&str> = text.trim().splitn(2, ':').collect();
                            if parts.len() == 2 {
                                let recipient = parts[0].trim();
                                let content = parts[1].trim();

                                // Check if recipient exists and send the message to them
                                let recipient = recipient.to_string();
                                if let Some((recipient_sink, _)) = user_connections.get(&recipient) {
                                    let recipient_sink = recipient_sink.
                                    match recipient_sink
                                        .send(Message::Text(format!("{}: {}", username, content)))
                                        .await
                                    {
                                        Ok(_) => {
                                            // Message sent successfully
                                        }
                                        Err(err) => {
                                            println!("Failed to send message: {:?}", err);
                                            // Optionally handle the error here
                                        }
                                    }
                                } else {
                                    println!("Recipient {} not found", recipient);
                                    // Optionally handle the case when the recipient is not found
                                }
                            } else {
                                println!("Invalid message format: {}", text);
                                // Optionally handle the case when the message format is invalid
                            }
                        }
                    }
                }
            } else {
                // Send authentication failure message
                let _ = sender
                    .send(Message::Text("Authentication failed".into()))
                    .await;
            }
        } else {
            // Send invalid credentials format message
            let _ = sender
                .send(Message::Text("Invalid credentials format".into()))
                .await;
        }
    }
}

fn authenticate_user(username: &str, password: &str) -> bool {
    // Implement your authentication logic here
    // For demonstration purposes, allow any user with any password
    true
}


