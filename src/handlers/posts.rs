use actix_web::{delete, get, post, put};
#[allow(unused_imports)]
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};

use serde::Serialize;
use serde_json;

use crate::diesel::prelude::*;
use crate::establish_connection;
use crate::models::post::{DeletePost, NewPost, Post};

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

    let posts = dsl::posts.order_by(dsl::id.asc()).load::<Post>(&conn).expect("Error loading posts");

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

#[delete("/delete")]
pub fn delete(post_target: web::Json<DeletePost>) -> impl Responder {
    use crate::schema::posts::dsl::*;

    let conn = establish_connection();

    let deleted_post: Post = diesel::delete(posts.filter(id.eq(post_target.id)))
        .get_result(&conn)
        .expect("Error deleting post");

    respond_with_json(deleted_post)
}

#[put("/update")]
pub fn update(post_json: String) -> impl Responder {
    use crate::schema::posts;

    let conn = establish_connection();

    let new_post: Post = serde_json::from_str(&post_json).unwrap();

    let updated_post: Post = diesel::update(posts::table)
        .set(&new_post)
        .filter(posts::id.eq(new_post.id))
        .get_result(&conn)
        .expect("Error updating post");
    
    respond_with_json(updated_post)
}
