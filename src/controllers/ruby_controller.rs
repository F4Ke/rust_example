// ruby controller

use actix_web::{web, HttpResponse, Error as ActixError, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use handlebars::Handlebars;

//// Front end

pub async fn ruby_handler(tmpl: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>) -> impl Responder {
    let data = serde_json::json!({ "ruby": ruby });
    let body = tmpl.render("ruby/list", &data).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}
