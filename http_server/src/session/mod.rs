use actix_web::{
    HttpResponse,
    Responder,
    web::{
        self,
        ServiceConfig
    }
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.route("/login",web::get().to(login));
}

async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login")
}
