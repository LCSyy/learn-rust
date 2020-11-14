use actix_web::{
    Scope, Route, Responder, HttpResponse, 
    web::{ServiceConfig},
    http::{Method}
};

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

async fn login() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Bye-bye")
}
