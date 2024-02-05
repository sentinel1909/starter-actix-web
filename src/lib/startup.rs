// src/lib/startup.rs

// dependencies
use crate::routes::health_check;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

// startup function, configures the app, returns a server with all the configured routes
pub fn startup(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .default_service(web::to(HttpResponse::NotFound))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
