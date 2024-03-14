pub mod login;
pub mod message;
// use axum::routing::get; TODO:
use axum::Extension;
use axum::{routing::post, Router};
use login::{login, UserDatabase};
use std::sync::Arc;
#[tokio::main]
async fn main() {
    let user_db = Arc::new(UserDatabase::new());
    let app = Router::new()
        .route("/login", post(login))
        // .route("/", get(health_check)) 
        // TODO:
        .layer(Extension(user_db));
    // Start the server
    let lis = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    let port = "3010";
    let address = "0.0.0.0";
    println!("Listening {address}:{port}");
    axum::serve(lis, app).await.unwrap();
}
