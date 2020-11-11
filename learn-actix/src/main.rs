//! actix web
//! 
//! login,logout
//! sync
use actix_web::{self, HttpServer, App};

const SERVER_ADDR: &str = "127.0.0.1:8511";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .service()
    })
    .bind(SERVER_ADDR)?
    .run()
    .await
}

