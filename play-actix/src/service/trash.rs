use actix_web::{
    Scope, Responder, HttpResponse,
    web
};

pub fn service(scope: &str) -> Scope {
    Scope::new(scope)
    .route("", web::get().to(get_trash))
}

async fn get_trash() -> impl Responder {
    HttpResponse::Ok().body("All trash.")
}
