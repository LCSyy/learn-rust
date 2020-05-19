use actix_web::{ HttpRequest, HttpResponse, Responder };

pub async fn index(_req: HttpRequest) -> impl Responder {
    println!("default index handler");
    HttpResponse::Ok().body("{\"msg\":Hello world!}")
}
