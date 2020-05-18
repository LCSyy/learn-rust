use actix_web::{ HttpRequest, HttpResponse, Responder };

pub async fn index(req: HttpRequest) -> impl Responder {
    println!("{}",req.path());
    HttpResponse::Ok().body("Hello world!")
}