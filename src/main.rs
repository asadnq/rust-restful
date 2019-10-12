use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::{delete, get, post, put};

use actix_diesel_crud::handlers::posts;

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::Ok().body("it works")
}

fn init_server() {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
            .service(index)
            .service(posts::get_all)
            .service(posts::find)
        )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap()
}

#[allow(unused_imports)]
#[allow(dead_code)]
fn main() {
    init_server()
}
