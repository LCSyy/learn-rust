use actix_web::{
    App, HttpServer
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .configure(http_server::app_config)
    })
    .bind("127.0.0.1:8544")?
    .run()
    .await
}
