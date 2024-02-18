use actix_web::web;

// Import the config function from book_routes
mod book_routes;
use book_routes::config as book_routes_config;

pub fn config(cfg: &mut web::ServiceConfig) {
    // Set up the /api scope
    cfg.service(
        web::scope("/api")
            // Include book routes under the /api scope
            .configure(book_routes_config) // This adds /api/books/* routes
            // You can add more route configurations here
    );
}
