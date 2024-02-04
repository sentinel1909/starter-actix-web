// src/lib/startup.rs

// dependencies
use crate::routes::health_check;
use actix_web::{web, App, HttpResponse, HttpServer};

// startup function, configures the app, returns a server with all the configured routes
pub async fn startup() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .default_service(web::to(HttpResponse::NotFound))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
