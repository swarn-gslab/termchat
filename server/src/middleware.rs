/* 
#[allow(unused_imports)]
use axum::{
    body::{Body, Bytes},
    extract::{FromRequest, Request},
    http::{Request as HttpRequest, Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
#[allow(unused_imports)]
use anyhow::Error;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::pin::Pin;

pub async fn with_token_validation<S, B>(
     req: Request<Body>,
    service: S,
) -> Result<Response<Body>, StatusCode>
where
    S: Service<Request<Body>, Response = Response<Body>> + Send + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    B: Send,
{
    let is_login_request = is_login_request(&req);
    if !is_login_request {
        validate_token(&req).await?;
    }

    // Call the inner service
    service.call(req).await.map_err(|e| {
        // tracing::error!("Internal Server Error: {:?}", Error::new(e));
        // info!("internal server error: {:?}", Error::new(e));
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn is_login_request(req: &Request<Body>) -> bool {
    req.uri().path() == "/login" && req.method() == axum::http::Method::POST
}

pub async fn validate_token(req: &Request<Body>) -> Result<(), StatusCode> {
    let auth_header = req.headers().get("Authorization");
    match auth_header {
        Some(value) => {
            let value_str = value.to_str().map_err(|_| StatusCode::BAD_REQUEST)?;
            if value_str.starts_with("Bearer ") {
                let token = value_str.trim_start_matches("Bearer ").to_string();
               
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
*/


