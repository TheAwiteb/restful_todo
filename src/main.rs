use std::path::Path;

use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use utoipa_swagger_ui::SwaggerUi;

mod api_docs;
mod auth;
mod errors;
mod ratelimit;
mod schemas;

#[cfg(test)]
mod tests;

/// Enishalize the database connection pool, return the database connection and if the database is existed.
/// ### Panics
/// * If can't get the database url from the environment
/// * If the database connection pool cannot be created
pub async fn enishalize_poll() -> DatabaseConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    if !Path::new("db.sqlite3").exists() {
        log::info!("Database is not existed, creating a new one");
        std::fs::File::create("db.sqlite3").expect("Can't create the database file");
    }
    Database::connect(database_url)
        .await
        .expect("Failed to create database connection pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_owned());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    let addr = format!("{}:{}", host, port);
    let pool = enishalize_poll().await;
    Migrator::up(&pool, None)
        .await
        .expect("Failed to run migrations");

    log::info!("Listening on http://{}", addr);
    log::info!(
        "OpenAPI document is available at http://{}/docs/openapi.json",
        addr,
    );
    log::info!("Swagger UI is available at http://{}/docs/swagger/", addr);

    let ratelimit_backend = InMemoryBackend::builder().build();

    HttpServer::new(move || {
        let ratelimit_middleware_builder = ratelimit::init_ip(ratelimit_backend.clone());

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(ratelimit_middleware_builder.build())
            .wrap(Logger::default())
            .service(web::scope("/api").configure(auth::init_routes))
            .service(
                // OpenAPI document
                web::scope("/docs").service(api_docs::openapi_json).service(
                    SwaggerUi::new("/swagger/{_:.*}").url("/docs/openapi.json", Default::default()),
                ),
            )
            .default_service(web::route().to(|| async {
                errors::Error::NotFound(
                    "There is no endpoint in this path with this method ):".to_string(),
                )
            }))
    })
    .bind(addr)?
    .run()
    .await
}
