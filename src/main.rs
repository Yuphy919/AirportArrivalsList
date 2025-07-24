mod models;
mod services;
mod handlers;

use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};
use handlers::{flights_api, health_check};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("🚀 Starting Airport Arrivals List Server...");
    println!("📡 Server will be available at: http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(flights_api)
            .service(health_check)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
