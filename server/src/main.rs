mod handlers;
// mod models;
// mod utils;
// mod config;

use actix_files as fs;
use actix_web::{App, HttpServer, web};
use crate::handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/pkg", "./wasm_chat_client/pkg").show_files_listing())
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .route("/", web::get().to(handlers::greet))
            .route("/ws/", web::get().to(handlers::chat_route))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
