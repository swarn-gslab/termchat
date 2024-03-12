/*
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::{net::TcpListener, sync::Mutex};
// use futures::{channel::{mpsc::Sender, oneshot::Receiver},:{channel::{mpsc::Sender, oneshot::Receiver}, sink::SinkExt, stream::{SplitSink, StreamExt}};
use futures::{
    sink::SinkExt,
    stream::{SplitSink,  StreamExt},
};


use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Json
    }, response::IntoResponse, routing::get, Router,

};
// use axum::Server;

// struct AppState {
//     user_set: Mutex<HashSet<String>>,
//     tx: broadcast::Sender<String>,
// }
type UserConnections = Arc<Mutex<HashMap<String, SplitSink<WebSocketUpgrade, Message>>>>;


#[tokio::main]
async fn main() {




    // let user_set = Mutex::new(HashSet::new());
    // let (tx, _rx) = broadcast::channel(100);

    // let app_state = Arc::new(AppState { user_set, tx });

    // let app = Router::new().route("/ws", get(websocket_handler)).with_state(app_state);

    /*
    let user_connections = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new().route("/ws", get(websocket_handler)).with_state(user_connections.clone());
    // let user_connections = Arc::new(Mutex::new(HashMap::new()));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3010")
                              .await
                             .unwrap();
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
*/

let user_connections = Arc::new(Mutex::new(HashMap::new()));
let app = Router::new()
    .route("/ws", get(websocket_handler))
    .with_state(user_connections.clone());

let listener = TcpListener::bind("127.0.0.1:3010")
    .await
    .unwrap();
axum::serve(listener, app).await.unwrap();

    // axum::Server::bind(&listener)
            //  .serve(app.into_make_service_with_state(user_connections))
            //  .await
            //  .unwrap();

}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    // State(state): State<Arc<AppState>>,
    user_connections:axum::extract::Extension<UserConnections>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, user_connections))
}

// async fn websocket(stream: WebSocket, state: Arc<AppState>) {
//     let (mut sender, mut receiver) = stream.split();
//     let mut username = String::new();
//     while let Some(Ok(message)) = receiver.next().await {
//              if let Message::Text(name) = message {
//                 check_username(&state, &mut username, &name);

async fn websocket(socket:WebSocket,user_connections:axum::extract::Extension<UserConnections>,){
    let (mut sender, mut receiver) = socket.split();
    if let Some(Ok(Message::Text(credentials)))=receiver.next().await{
        let parts:Vec<&str>=credentials.trim().split(' ').collect();
        if parts.len() == 2{
            let username = parts[0].to_string();
            let password = parts[1].to_string();

            let mut user_connections = UserConnections.lock().unwrap();

            if authenticate_user(&username,&password){
                user_connections.insert(username.clone(),sender.clone());

                while let Some(msg) = receiver.next().await {
                    let msg = match msg {
                        Ok(msg)=>msg,
                        Err(e)=>{
                            println!("Error receiving msg:{:?}",0);
                            break;
                        }
                    };


                    let parts: Vec<&str> = msg.to_str().unwrap().splitn(2, ':').collect();
                    if parts.len()==2{
                        let receipient = parts[0].trim();
                        let content = parts[1].trim();


                        if let Some(recipient_sender) = user_connections.get(receipient){
                            if let Err(_) = recipient_sender.send(Message::Text(content.to_string())).await{
                                println!("failed to send message to user");
                            }
                            else {
                                println!("User '{}' not found",receipient);
                            }
                            }else {
                                println!("Invalid messages format:{}",msg.to_str().unwrap());
                            }
                        }

                        user_connections.remove(&username);
                    } else {
                        if let Err(_) = sender
                                        .send(Message::Text("Authentication failed".to_string()))
                                        .await
                                        {
                                            println!("User Disconnected");
                                        }
                    }
                }
                else {
                    if let Err(_) = sender
                                   .send(Message::Text("Invalid id".to_string())).await{
                                    println!("User Disconnected");
                                   }
                }
            }
        }
    }




//             if !username.is_empty() {
//                 break;
//             } else {
//                 let _ = sender
//                     .send(Message::Text(String::from("client is already in the server")))
//                     .await;
//                 return;
//    }

/*
     if username.is_empty() {
    let _ = sender
        .send(Message::Text(String::from("client is already in the server")))
        .await;
    return;
} else {
    let _ = sender
        .send(Message::Text(format!("Welcome, {}", username)))
        .await;
    break;
}

}
 }

    let mut rx = state.tx.subscribe();

    let msg = format!("client send message:{}",username);
    tracing::debug!("Received Message :{}",msg);
    let _ = state.tx.send(msg);

                 // messages over the websocket to sends to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
                if sender.send(Message::Text(msg)).await.is_err() {
                   break;
            }
        }
    });
    let tx = state.tx.clone();
    let name = username.clone();

                             // Spawn a task that takes messages from the websocket
    #[allow(unused_variables)]
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
                    let _ = tx.send(format!("Received Message: {name}"));
        }
    });


    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
         _=(&mut recv_task) => send_task.abort(),
    };

    let msg = format!("{username} client left.");
    tracing::debug!("{msg}");
    let _ = state.tx.send(msg);

                   state.user_set.lock().unwrap().remove(&username);
}

fn check_username(state: &AppState, string: &mut String, name: &str) {
    let mut user_set = state.user_set.lock().unwrap();

    if !user_set.contains(name) {
        user_set.insert(name.to_owned());

        string.push_str(name);
    }
}

*/

// user authentication
fn authenticate_user(username:&str, password:&str)->bool{
    true
}
*/

/*
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade}, response::IntoResponse, routing::get, Router
};
use futures::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;


type UserConnections = Arc<Mutex<HashMap<String, SplitSink<WebSocketStream<WebSocket>, Message>>>>;

struct WebSocketStream<S>(S);

impl<S> WebSocketStream<S> {
    fn new(socket: S) -> Self {
        WebSocketStream(socket)
    }
}

impl<S> From<WebSocket<S>> for WebSocketStream<S> {
    fn from(socket: WebSocket<S>) -> Self {
        WebSocketStream(socket)
    }
}

#[tokio::main]
async fn main() {
    let user_connections = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(user_connections.clone());

    let listener = TcpListener::bind("127.0.0.1:3010")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    user_connections: axum::extract::State<UserConnections>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, user_connections.clone()))
}

async fn handle_socket(
    socket: WebSocket,
    user_connections: axum::extract::State<UserConnections>,
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
                // Store the user's WebSocket sender
                user_connections
                    .entry(username.clone())
                    .or_insert_with(|| {
                        let (sender, receiver) = socket.split();
                        SplitSink::from(WebSocketStream::new(sender))
                    });

                // Send message to the user indicating successful login
                let _ = sender.send(Message::Text("Successfully logged in!".into())).await;

                // Iterate through all other users and send a message indicating the new user has joined
                for (name, sink) in user_connections.iter_mut() {
                    if name != &username {
                        let _ = sink.send(Message::Text(format!("User {} has joined", username))).await;
                    }
                }

                // Listen for incoming messages
                while let Some(msg) = receiver.next().await {
                    if let Ok(msg) = msg {
                        if let Message::Text(text) = msg {
                            println!("Received message: {}", text);
                        }
                    }
                }
            } else {
                // Send authentication failure message
                let _ = sender.send(Message::Text("Authentication failed".into())).await;
            }
        } else {
            // Send invalid credentials format message
            let _ = sender.send(Message::Text("Invalid credentials format".into())).await;
        }
    }
}
fn authenticate_user(username: &str, password: &str) -> bool {
    // Implement your authentication logic here
    // For demonstration purposes, allow any user with any password
    true
}
*/

// use axum::{
//     extract::{response::IntoResponse, routing::get, ws::{Message, WebSocket, WebSocketUpgrade}},
//     Router,
// };


#[allow(unused_imports)]
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
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// async fn ws_handler(
//     ws: WebSocketUpgrade,
//     user_connections: axum::extract::Extension<UserConnections>,
// ) -> impl IntoResponse {
//     ws.on_upgrade(move |socket| handle_socket(socket, user_connections.clone()))
// }
async fn websocket_handler(
    ws: WebSocketUpgrade,
    // State(state): State<Arc<AppState>>,
    // Extension(user_connections): Extension<Arc<Mutex<HashMap<String, (SplitSink<WebSocketUpgrade, Message>, User)>>>>,
    user_connections: axum::extract::Extension<UserConnections>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, user_connections.clone()))
}

/* 
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

                // Iterate through all other users and send a message indicating the new user has joined
                // for (name, sink) in user_connections.iter_mut() {
                //     if name != &username {
                //         match sink.send(Message::Text(format!("User {} has joined", username))).await {
                //             Ok(_) => {
                //                 // Message sent successfully
                //             }
                //             Err(err) => {
                //                 println!("Failed to send message: {:?}", err);
                //                 // Optionally handle the error here
                //             }
                //         }
                //     }
                // }

                for (name, (sink, _)) in user_connections.iter_mut() {
                    if name != &username {
                        match sink
                            .with(|sink| sink.send(Message::Text(format!("User {} has joined", username))))
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
                    }
                }


                // Listen for incoming messages
                while let Some(msg) = receiver.next().await {
                    if let Ok(msg) = msg {
                        if let Message::Text(text) = msg {
                            println!("Received message: {}", text);
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
*/



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


