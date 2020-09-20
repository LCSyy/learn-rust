use actix_web::{
    Scope,
    Responder,
    HttpResponse,
    web::{
        self,
        ServiceConfig
    }
};

use rusqlite::{ params, Connection };
use super::model;

fn db() -> Connection {
    Connection::open("data/writer.db").unwrap()
}

pub fn config(cfg: &mut ServiceConfig) {
    db().execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT UNIQUE NOT NULL, data TEXT NULL)", params![]).unwrap();

    cfg
    .service(
        Scope::new("/users")
        .route("", web::post().to(add))
        .route("/{id}", web::get().to(get))
        .route("/{id}", web::delete().to(del))
        .route("/{id}", web::put().to(update))
    );
}

async fn add(data: web::Json<model::Users>) -> impl Responder {
    let row_count = match db().execute("INSERT INTO users (name,data) VALUES (?,?)",params![data.name_ref(), data.data_ref()]) {
        Ok(c) => c,
        Err(e) => {
            println!("add user error: {}", e);
            0
        }
    };

    HttpResponse::Ok().body(row_count.to_string())
}

async fn del(id: web::Path<i64>) -> impl Responder {
    let mut conn = db();
    let tx = conn.transaction().unwrap();
    tx.execute("DELETE FROM users WHERE id = ?",params![id.to_string()]).unwrap();
    tx.commit().unwrap();
    HttpResponse::Ok().body(format!("Removed row: {}",id))
}

async fn get(id: web::Path<i64>) -> impl Responder {
    let conn = db();
    let mut stmt = conn.prepare("SELECT * FROM users WHERE id = ?").unwrap();
    let mut users_iter = stmt.query_map(params![id.to_string()],|row|{
        Ok(model::Users::from(row))
    }).unwrap();

    let d = match users_iter.nth(0) {
        Some(row) => {
            match row {
                Ok(u) => u,
                Err(_) => model::Users::new()
            }
        },
        None => model::Users::new()
    };

    HttpResponse::Ok().json(d)
}

async fn update(id: web::Path<i64>, data: web::Json<model::Users>) -> impl Responder {
    let user_id = id.into_inner();

    let mut conn = db();
    let tx = conn.transaction().unwrap();
    tx.execute("UPDATE users SET name = ?, data = ? WHERE id = ?", params![data.name_ref(), data.data_ref(),user_id]).unwrap();
    let ret = tx.query_row("SELECT * FROM users WHERE id = ?",params![user_id],|row|{
        Ok(model::Users::from(row))
    });
    tx.commit().unwrap();

    HttpResponse::Ok().json(ret.unwrap_or(model::Users::new()))
}
