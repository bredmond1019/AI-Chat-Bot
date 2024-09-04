use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;
use log::{error, info};
use std::env;

mod models;
mod routes;
mod services;

use services::chat_server::ChatServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "debug"); // Changed to debug for more verbose logging
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    info!("Initializing ChatServer");
    let chat_server = ChatServer::new().start();
    info!("ChatServer initialized and started");

    info!("Starting server at http://localhost:8080");
    HttpServer::new(move || {
        info!("Configuring new worker");
        App::new()
            .app_data(Data::new(chat_server.clone()))
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .configure(routes::init_routes)
    })
    .bind(("127.0.0.1", 8080))
    .map_err(|e| {
        error!("Failed to bind to address: {:?}", e);
        e
    })?
    .run()
    .await
    .map_err(|e| {
        error!("Server error: {:?}", e);
        e
    })
}
