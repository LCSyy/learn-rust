use actix_web::{
    Scope, Responder, HttpResponse,
    web
};

pub fn service(scope: &str) -> Scope {
    Scope::new(scope)
    .route("", web::get().to(get_all_resource))
    .route("/{resname}", web::get().to(get_resource))
}

async fn get_resource(res: web::Path<String>) -> impl Responder {
    if res.len() == 0 {
        return HttpResponse::Ok().body("All trash.");
    }

    HttpResponse::Ok().body(format!("Get trash {}", res))
}

async fn get_all_resource() -> impl Responder {
    HttpResponse::Ok().body("All trash.")
}
