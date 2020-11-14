mod conf;
mod service;

use conf::{Config};
use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("conf.toml");
    
    HttpServer::new(||{
        App::new().configure(service::service_config)
    })
    .bind(config.server.addr)?
    .run()
    .await
}
