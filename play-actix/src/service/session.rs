use std::error::Error;
use actix_web::{
    Scope, Route, Responder, HttpResponse, 
    web,
    http::{Method}
};
use sqlx::postgres::PgPool;
use sqlx::{Executor,Row,FromRow};
use serde::{Serialize,Deserialize};

pub fn service(scope: &str) -> Scope {
    Scope::new(scope)
    .route("/register", Route::new().method(Method::POST).to(register))
    .route("/login", Route::new().method(Method::GET).to(login))
    .route("/logout", Route::new().method(Method::GET).to(logout))
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct User {
    id: i32,
    account: String,
    password: String,
    name: String,
    remark: Option<String>,
}

#[derive(Deserialize)]
struct RegisterParam {
    account: String,
    password: String,
    name: String,
}

async fn register(pool: web::Data<PgPool>, body: web::Json<RegisterParam>) -> impl Responder {
    let mut tx = pool.into_inner().begin().await.unwrap();
    
    match tx.fetch_one(sqlx::query(r#"insert into users (account,"password","name") values ($1,$2,$3) returning *;"#)
        .bind(body.account.clone()).bind(body.password.clone()).bind(body.name.clone())).await
    {
        Ok(row) => {
            let u = User::from_row(&row).unwrap();
            tx.commit().await;
    
            return HttpResponse::Ok().json(u);
        },
        Err(e) => {
            tx.rollback().await;
            return HttpResponse::Ok().body(format!("[ERROR] {}",e));
        }
    }
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
