// src/bin/main.rs

// dependencies
use starter_actix_web_lib::configuration::Config;
use starter_actix_web_lib::startup;
use std::net::TcpListener;

// main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = Config::default();
    let address = TcpListener::bind(format!("{}:{}", app_config.address, app_config.port))?;
    startup(address)?.await
}
