use actix_web::{
    Scope, Responder, HttpResponse,
    web
};

pub fn service(scope: &str) -> Scope {
    Scope::new(scope)
    .route("", web::get().to(get_tags))
}

async fn get_tags() -> impl Responder {
    HttpResponse::Ok().body("This is all tags.")
}
