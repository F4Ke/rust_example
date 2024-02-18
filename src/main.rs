mod schema;
mod config;
mod models;
mod controllers;
mod routes;
mod db;
mod dal;
mod traits;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
use actix_files as fs;

use handlebars::Handlebars;
use crate::models::Status;
use dotenv::dotenv;
use tracing::{info, Level};
use tracing_subscriber;


#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "ok".to_string()
    })
}

fn init_tracing_and_logs() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        // Add Datadog-specific configuration here if available
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // load the .env file
    // dotenv::from_filename(".env.test").ok(); if we want test specific?

    let config = crate::config::AppConfig::from_env().unwrap();
    init_tracing_and_logs();

    let host = config.server.host;
    let port = config.server.port;
    let pool = db::init_pool(&config.database.url);
    let pool_data = web::Data::new(pool); // Wrap the pool with Data::new()


    let mut handlebars = Handlebars::new();
    // Register your templates
    let _ = handlebars.register_templates_directory(".html", "./templates/").unwrap();

    let handlebars_ref = web::Data::new(handlebars);

    info!("Server started listening at http://{}:{}/", host, port);

    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing()) // Serve static files
            .app_data(handlebars_ref.clone()) // front
            .app_data(pool_data.clone()) // Assuming you have set up a database pool
            .configure(crate::routes::config) // API routes
            .service(status)
            .service(
                web::resource("/books")
                    .route(web::get().to(controllers::books_handler))
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await

}

// TESTS

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn test_status() {
        // Initialize the test service with the handler.
        let app = test::init_service(App::new().service(status)).await;

        // Create a test request for the health checker endpoint.
        let req = test::TestRequest::get()
            .uri("/status")
            .to_request();

        // Send the request to the handler and get the response.
        let resp = test::call_service(&app, req).await;

        // Assert that the response status code is OK (200).
        assert_eq!(resp.status(), StatusCode::OK);

        // Read and parse the response body.
        let body = test::read_body(resp).await;
        let expected_json =
            serde_json::json!({"status": "ok"});

        // Assert that the response body matches the expected JSON.
        assert_eq!(body, serde_json::to_string(&expected_json).unwrap());
    }
}
