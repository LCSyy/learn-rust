use actix_web::{
    Scope, Responder, HttpResponse,
    web
};

pub fn service(scope: &str) -> Scope {
    Scope::new(scope)
    .route("", web::get().to(user_info))
}

async fn user_info() -> impl Responder {
    HttpResponse::Ok().body("User information")
}
