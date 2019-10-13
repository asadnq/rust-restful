use actix_web::{delete, get, post, put};
#[allow(unused_imports)]
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};

use serde::Serialize;
use serde_json;

use crate::diesel::prelude::*;
use crate::establish_connection;
use crate::models::post::{NewPost, Post};

pub fn respond_with_json<T: Serialize>(body: T) -> impl Responder {
    let response = serde_json::to_string(&body).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

#[get("/get_all")]
pub fn get_all() -> impl Responder {
    use crate::schema::posts::dsl;
    let conn = establish_connection();

    let posts = dsl::posts
        .limit(5)
        .load::<Post>(&conn)
        .expect("Error loading posts");
    // let mut result = ListOfPost::new();
    println!("posts: {:?}", posts);

    respond_with_json(posts)
}

#[get("/find/{id}")]
pub fn find(path: web::Path<(i32,)>) -> impl Responder {
    use crate::schema::posts::dsl::*;
    let conn = establish_connection();

    let post = posts
        .filter(id.eq(path.0))
        .first::<Post>(&conn)
        .expect("Error loading post");

    respond_with_json(post)
}

#[post("/add")]
pub fn add(post: String) -> impl Responder {
    use crate::schema::posts;

    let conn = establish_connection();

    let mut payload: NewPost = serde_json::from_str(&post).unwrap();

    if payload.created_at == None {
        payload = payload.with_time();
    }

    let new_post: Post = diesel::insert_into(posts::table)
        .values(&payload)
        .get_result(&conn)
        .expect("Error when creating post");

    respond_with_json(new_post)
}
