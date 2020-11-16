use actix_web::{
    Scope, Responder, HttpResponse,
    web
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

pub fn service(scope: &str) -> Scope {
    Scope::new(scope)
    .route("", web::get().to(user_info))
    .route("", web::post().to(new_user))
}

async fn user_info(db: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok().body("User information")
}

#[derive(Deserialize)]
struct NewUserParam {
    name: String,
    age: i32,
}

#[derive(Serialize)]
struct UserInfo {
    id: i32,
    name: String,
    age: i32,
}

async fn new_user(db: web::Data<PgPool>, body: web::Json<NewUserParam>) -> impl Responder {
    let user: (i32,String,i32) = sqlx::query_as("INSERT INTO user_info (name, age) VALUES ($1,$2) RETURNING *;")
        .bind(body.name.clone())
        .bind(body.age)
        .fetch_one(&*db.into_inner())
        .await.unwrap();
    
    HttpResponse::Ok().body(format!("New user information: {}, {}, {}", user.0, user.1, user.2))
}
