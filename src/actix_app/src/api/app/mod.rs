use rand;
use actix_web::{ HttpRequest, HttpResponse, Responder };

const MSG: [&str;5] = [
    "Everything is ok!",
    "Should I do something?",
    "That's great!","Hello!",
    "I'm doing homework."
];

fn msg<'a>() -> &'a str {
    let rnum = rand::random::<usize>();
    MSG[rnum % MSG.len()]
}

pub async fn app_reigster(req: HttpRequest) -> impl Responder {
    println!("{}",req.path());
    HttpResponse::Ok().body(msg())
}

pub async fn app_start() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}
