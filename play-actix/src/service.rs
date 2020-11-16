use actix_web::{
    Scope, Route, Responder, HttpResponse, 
    web::{self, ServiceConfig},
    http::{Method}
};
use sqlx::postgres::PgPool;
use sqlx::{Executor,Row};
use serde::{Deserialize};

mod user;
mod blog;
mod tag;
mod trash;
mod res;

pub fn service_config(cfg: &mut ServiceConfig) {
    cfg.service(
        Scope::new("/api")
        .service(
            Scope::new("/v1")
            .route("/register", Route::new().method(Method::POST).to(register))
            .route("/login", Route::new().method(Method::GET).to(login))
            .route("/logout", Route::new().method(Method::GET).to(logout))
            .service(user::service("/user"))
            .service(blog::service("/blog"))
            .service(tag::service("/tag"))
            .service(trash::service("/trash"))
            .service(res::service("/res"))
        )
    );
}

#[derive(Debug)]
#[derive(sqlx::FromRow)]
struct User {
    id: i32,
    account: String,
    password: String,
    name: String,
    remark: String,
}

#[derive(Deserialize)]
struct RegisterParam {
    account: String,
    password: String,
    name: String,
    remark: String,
}

async fn register(pool: web::Data<PgPool>, body: web::Json<RegisterParam>) -> impl Responder {
    let mut tx = pool.into_inner().begin().await.unwrap();
    
    if let Ok(row) = tx.fetch_one(sqlx::query(r#"insert into users (account,"password","name",remark) values ($1,$2,$3,$4) returning *;"#)
        .bind(body.account.clone())
        .bind(body.password.clone())
        .bind(body.name.clone())
        .bind(body.remark.clone())).await
    {

        let u = User {
            id: row.get("id"),
            account: row.get("account"),
            password: row.get("password"),
            name: row.get("name"),
            remark: row.get("remark")
        };
        
        tx.commit().await;

        return HttpResponse::Ok().body(format!("{:#?}",u));
    }

    tx.rollback().await;

    HttpResponse::Ok().body("register failed!")
}

#[derive(Deserialize)]
struct LoginParam {
    account: String,
    password: String,
}

async fn login(db: web::Data<PgPool>, body: web::Json<LoginParam>) -> impl Responder {
    let user: User = sqlx::query_as("SELECT * FROM users WHERE account = $1 AND password = $2;")
        .bind(body.account.clone())
        .bind(body.password.clone())
        .fetch_one(&*db.into_inner())
        .await
        .unwrap();

    println!("user account:{}",user.account);

    HttpResponse::Ok().body("Welcome")
}

async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Bye-bye")
}
