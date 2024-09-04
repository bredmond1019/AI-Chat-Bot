use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod models;
mod routes;
mod services;

use services::chat_server::ChatServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let chat_server = web::Data::new(ChatServer::new().start());

    println!("Server running at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(chat_server.clone())
            .configure(routes::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
