use actix_web::{web, HttpServer, App, middleware};

mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api").service(handlers::index)
            )
    })
    .workers(16)
    .bind("0.0.0.0:8200")?
    .run()
    .await
}