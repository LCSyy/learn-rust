mod conf;
mod service;

use conf::{Config};
use actix_web::{HttpServer, App};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("conf.toml");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:pg2020@localhost:5432/postgres").await.unwrap();

    HttpServer::new(move||{
        App::new().data(pool.clone()).configure(service::service_config)
    })
    .bind(config.server.addr)?
    .run()
    .await
}
