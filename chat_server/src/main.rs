#![allow(unused_imports)]
#![allow(dead_code)]
#![forbid(unsafe_code)]

use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use axum::response::Html;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener};
use bytes;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let app = Router::new()
        .route("/hello", get(|| async { Html("Hello World!") }));

    axum::serve(listener, app).await.unwrap();
}
