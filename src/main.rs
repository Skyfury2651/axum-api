use crate::config::database::DatabaseTrait;
use crate::config::{database, parameter};
use std::sync::Arc;

mod config;
mod handler;
mod model;
mod response;
mod routes;
mod service;
mod state;
// TODO:: we currently removing CORS

#[tokio::main]
async fn main() {
    parameter::init();
    let connection = database::Database::init()
        .await
        .unwrap_or_else(|e| panic!("Database error: {}", e.to_string()));

    tracing_subscriber::fmt::init();
    let host = format!("0.0.0.0:{}", parameter::get("PORT"));
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();

    axum::serve(listener, routes::root::routes(Arc::new(connection)))
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e.to_string()));
}
