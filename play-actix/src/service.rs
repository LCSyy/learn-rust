use actix_web::{
    Scope, Route, Responder, HttpResponse,
    web::ServiceConfig,
};

mod session;
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
            .service(session::service("/session"))
            .service(user::service("/user"))
            .service(blog::service("/blog"))
            .service(tag::service("/tag"))
            .service(trash::service("/trash"))
            .service(res::service("/res"))
        )
    );
}
