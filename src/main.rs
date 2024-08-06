use std::sync::Arc;

use config::database::Database;
use sea_orm::DatabaseConnection;

use crate::config::{ parameter, database };
use migration::{ Migrator, MigratorTrait };

mod middleware;
mod config;
mod handler;
mod response;
mod routes;
mod service;
mod state;
mod repository;
mod dto;
mod error;
mod entity;

#[tokio::main]
async fn main() {
    parameter::init();
    let database: Database = database::Database
        ::init().await
        .unwrap_or_else(|e| panic!("Database error: {}", e.to_string()));

    let connection: DatabaseConnection = database.connection.clone();

    Migrator::up(&connection, None).await.unwrap();

    let host = format!("0.0.0.0:{}", parameter::get("PORT"));
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();

    let app = routes::root::routes(Arc::new(database));
    axum::serve(listener, app).await.unwrap_or_else(|e| panic!("Server error: {}", e.to_string()));
}
