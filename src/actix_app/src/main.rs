mod api;

use actix_web::{ web, App,  HttpServer };
use api::{ universe, app };

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(
            web::scope("/app")
            .route("/register", web::post().to(app::app_reigster))
            .route("/start", web::post().to(app::app_start))
        )
        .default_service(web::route().to(universe::index))
    })
    .bind("127.0.0.1:9001")?
    .run()
    .await
}
