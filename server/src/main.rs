pub mod login;
use axum::Extension;
use axum::{routing::post, Router};
use login::{login, UserDatabase};
use std::sync::Arc;
#[tokio::main]
async fn main() {
    let user_db = Arc::new(UserDatabase::new());
    let app = Router::new()
        .route("/login", post(login))
        .layer(Extension(user_db));
    // Start the server
    let lis = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    axum::serve(lis, app).await.unwrap();
}
