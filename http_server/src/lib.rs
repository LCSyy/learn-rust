mod cxfw;
mod session;

use actix_web::{
    Scope,
    web::{
        self,
        ServiceConfig
    }
};

use cxfw::{ writer };

pub fn app_config(cfg: &mut ServiceConfig) {
    cfg.service(Scope::new("/writer").configure(writer::config))
    .service(web::scope("/session").configure(session::config));
}
