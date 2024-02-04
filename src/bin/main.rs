// src/bin/main.rs

// dependencies
use actix_web_template_lib::startup;

// main function
#[actix_web::main]
async fn main() {
    startup()
        .await
        .expect("Unable to start the server on port 8000")
}
