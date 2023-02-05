use actix_web::{web, HttpServer, App};

mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api").service(handlers::index)
            )
    })
    .workers(16)
    .bind("127.0.0.1:8200")?
    .run()
    .await
}