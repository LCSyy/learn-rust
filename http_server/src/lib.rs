mod cxfw;

use actix_files::Files;
use actix_web::{
    Scope,
    web::{
        ServiceConfig
    }
};

use cxfw::{ writer };

pub fn app_config(cfg: &mut ServiceConfig) {
    cfg
    .service(Files::new("/", "./data/web/").index_file("index.html"))
    .service(
        Scope::new("/api")
        .service(
            Scope::new("/writer").configure(writer::config)
        )
    );
}
