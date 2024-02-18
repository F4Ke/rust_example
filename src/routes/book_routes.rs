// books routes
use actix_web::web;
use crate::controllers::book_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/books")
            .route("/", web::post().to(book_controller::create_book))
            .route("/", web::get().to(book_controller::list_books))
            .route("/{id}", web::get().to(book_controller::get_book))
            .route("/{id}", web::put().to(book_controller::update_book))
            .route("/{id}", web::delete().to(book_controller::delete_book))
    );
}
