// books controller

use actix_web::{web, HttpResponse, Error as ActixError, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use handlebars::Handlebars;
use crate::dal::DataAccessLayer;
use crate::models::books::{Book, NewBook, UpdateBook};

type DbPool = Pool<ConnectionManager<PgConnection>>;

//// Front end

pub async fn books_handler(tmpl: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>) -> impl Responder {
    let dal: DataAccessLayer = DataAccessLayer::new(pool.get_ref().clone());

    match dal.list::<Book>() {
        Ok(books) => {
            let data = serde_json::json!({ "books": books });
            let body = tmpl.render("books/list", &data).unwrap();
            HttpResponse::Ok().content_type("text/html").body(body)        
        },
        Err(e) => {
            eprintln!("Error listing books: {}", e);
            let body = tmpl.render("errors/error", &{}).unwrap();
            HttpResponse::Ok().content_type("text/html").body(body)        
        }
    }
}


//// API
/// // MOVE IN ANOThER FILE ?
// CREATE
pub async fn create_book(pool: web::Data<DbPool>, new_book: web::Json<NewBook>) -> Result<HttpResponse, ActixError> {
    let dal: DataAccessLayer = DataAccessLayer::new(pool.get_ref().clone());

    match dal.create::<Book, NewBook>(new_book.into_inner()) {
        Ok(book) => Ok(HttpResponse::Ok().json(book)),
        Err(e) => {
            eprintln!("Error: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

// LIST
pub async fn list_books(pool: web::Data<DbPool>) -> Result<HttpResponse, ActixError> {
    let dal = DataAccessLayer::new(pool.get_ref().clone());

    match dal.list::<Book>() {
        Ok(books) => Ok(HttpResponse::Ok().json(books)),
        Err(e) => {
            eprintln!("Error listing books: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

// UPDATE
pub async fn update_book(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
    updated_book: web::Json<UpdateBook>,
) -> Result<HttpResponse, ActixError> {
    let dal = DataAccessLayer::new(pool.get_ref().clone());
    let id = book_id.into_inner();

    match dal.update::<Book, UpdateBook>(id, updated_book.into_inner()) {
        Ok(book) => Ok(HttpResponse::Ok().json(book)),
        Err(e) => {
            eprintln!("Error updating book: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

// DELETE 
pub async fn delete_book(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, ActixError> {
    let dal = DataAccessLayer::new(pool.get_ref().clone());
    let id = book_id.into_inner();

    match dal.delete::<Book>(id) {
        Ok(_) => Ok(HttpResponse::Ok().json("Book deleted successfully")),
        Err(e) => {
            eprintln!("Error deleting book: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

// GET
pub async fn get_book(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, ActixError> {
    let dal = DataAccessLayer::new(pool.get_ref().clone());
    let id = book_id.into_inner();

    match dal.retrieve::<Book>(id) {
        Ok(book) => Ok(HttpResponse::Ok().json(book)),
        Err(e) => {
            eprintln!("Error retrieving book: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}


// TESTS