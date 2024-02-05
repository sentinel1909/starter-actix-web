// src/lib/startup.rs

// dependencies
use crate::configuration::Config;
use crate::routes::health_check;
use actix_web::{web, App, HttpResponse, HttpServer};

// startup function, configures the app, returns a server with all the configured routes
pub async fn startup() -> std::io::Result<()> {
    let app_config = Config::default();
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .default_service(web::to(HttpResponse::NotFound))
    })
    .bind((app_config.address, app_config.port))?
    .run()
    .await
}
