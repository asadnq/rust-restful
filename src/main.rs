use actix_web::{web, App, HttpServer};

use actix_diesel_crud::handlers::posts;

fn init_server() {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
            .service(posts::get_all)
            .service(posts::find)
            .service(posts::add)
            .service(posts::delete)
            .service(posts::update)
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
