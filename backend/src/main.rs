use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::{error, info};
use log4rs;
use std::env;
use std::sync::Arc;

pub mod db;
pub mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");

    // Initialize logger
    match log4rs::init_file("log4rs.yaml", Default::default()) {
        Ok(_) => info!("Logger initialized successfully"),
        Err(e) => error!("Failed to initialize logger: {}", e),
    }

    info!("Starting application");

    // Initialize SurrealDB connection
    let db = match db::init_surrealdb().await {
        Ok(db) => {
            info!("Successfully connected to SurrealDB");
            Arc::new(db)
        }
        Err(e) => {
            error!("Failed to connect to SurrealDB: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to initialize database",
            ));
        }
    };

    // Start the server
    info!("Server listening on 127.0.0.1:3000");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
    .map_err(|e| {
        error!("Server error: {:?}", e);
        e
    })
}
