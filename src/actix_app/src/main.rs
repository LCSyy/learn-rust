use actix_web::{ web, App,  HttpServer };

mod universe_api;
mod app_api;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(
            web::scope("/app")
            .route("/register", web::post().to(app_api::app_reigster))
            .route("/start", web::post().to(app_api::app_start))
        )
        .default_service(web::route().to(universe_api::index))
    })
    .bind("127.0.0.1:9001")?
    .run()
    .await
}