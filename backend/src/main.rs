// File: backend/src/main.rs

use actix_web::{App, HttpServer};

mod db;
mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: Initialize database connection
    // TODO: Set up Actix web server
    // TODO: Configure WebSocket
    // TODO: Set up routes

    println!("Server running at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
        // TODO: Add service configurations and routes
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
