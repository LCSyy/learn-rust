
use actix_files::Files;
use actix_web::{
    App, HttpServer, Scope
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(Files::new("/", "./data/web/").index_file("index.html"))
        .service(
            Scope::new("/api")
            .configure(http_server::app_config)
        )
    })
    .bind("127.0.0.1:8544")?
    .run()
    .await
}
