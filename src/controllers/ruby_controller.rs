// ruby controller

use actix_web::{web, HttpResponse, Responder};
use handlebars::Handlebars;
use macro_ruby::{ruby_code_to, ruby_file_to};
use serde::Deserialize;
use std::process::{Command, Output};

// alternative to macro_ruby
fn execute_ruby_code(ruby_code: &str) -> Result<Output, std::io::Error> {
    Command::new("ruby")
        .arg("-e")
        .arg(ruby_code)
        .output()
}

#[derive(Deserialize)]
pub struct QueryParams {
    fib_number: Option<i32>,
}

pub async fn ruby_run(tmpl: web::Data<Handlebars<'_>>, params: web::Query<QueryParams>) -> impl Responder {

    // ruby_code_to! execute ruby code and parse the result
    let val_a: i32 = ruby_code_to!(i32 "print 20+22");

    // more complexe
    let val_b: i32 = ruby_code_to!(i32 r#"
        def fibonacci(n)
            return n if n <= 1
            fibonacci(n - 1) + fibonacci(n - 2)
        end
        print fibonacci(20);
        "#);

    // ALTERNATIVE

    let fib_number = params.fib_number.unwrap_or(20);

    let ruby_code = format!(r#"
        def fibonacci(n)
            return n if n <= 1
            fibonacci(n - 1) + fibonacci(n - 2)
        end
        print fibonacci({});
    "#, fib_number);

    let output = execute_ruby_code(&ruby_code).unwrap();
    let fib_from_query = String::from_utf8_lossy(&output.stdout).trim().parse::<i32>().unwrap_or_default();

    // FROM SCRIPT FILE
    let fib_from_file: i32 = ruby_file_to!(i32 "src/controllers/ruby_scripts/fib.rb");

    let data = serde_json::json!({ "value_a": val_a, "value_b": val_b, "value_c": fib_from_query, "from_file": fib_from_file });
    let body = tmpl.render("ruby/run", &data).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}
