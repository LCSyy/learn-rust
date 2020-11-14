use actix_web::{
    Scope, Responder, HttpResponse,
    web
};

pub fn service(scope: &str) -> Scope {
    Scope::new(scope)
    .route("", web::post().to(new_blog))
    .route("/{uuid}", web::get().to(del_blog))
    .route("", web::get().to(get_blogs))
    .route("/{uuid}", web::get().to(get_blog))
    .route("", web::put().to(update_blog))
}

async fn new_blog() -> impl Responder {
    HttpResponse::Ok().body("New blog")
}

async fn del_blog(uuid: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Del blog: {}", uuid.into_inner()))
}

async fn get_blogs() -> impl Responder {
    HttpResponse::Ok().body("This is my post list.")
}

async fn get_blog(uuid: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Get post: {}", uuid.into_inner()))
}

async fn update_blog() -> impl Responder {
    HttpResponse::Ok().body("Update blog")
}
