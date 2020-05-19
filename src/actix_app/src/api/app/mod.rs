use rand;
use serde_json;
use serde::{ Serialize, Deserialize };
use actix_web::{ HttpRequest, HttpResponse, Responder };

const MSG: [&str;4] = [
    "Everything is ok!",
    "Often I Am Permitted to Return to a Meadow",
    "I Ask Grandmother If We an Make Lahmajun",
    "I'm doing homework."
];

const BODY: [&str;4] = [
    "What we want is money! So don't tell me about your dream, just tell me what you have, and what i can get.",
    "As if it were a scene made-up by the mind,that is not mine, but is a made palce, this is mine, it is so near to the heart, an eternal pasture folded in all thought so that there is a hall therein.",
    "Sure, she says, why not, we buy the ground lamb from the market, we by parsley, fresh tomatoes, garlic, we cut, press, dice, mix.",
    "The Ending..."
];

#[derive(Serialize, Deserialize)]
struct Paragraph {
    brief: String,
    text: String
}

fn msg<'a>() -> &'a str {
    let rnum = rand::random::<usize>();
    MSG[rnum % MSG.len()]
}

fn msg_body<'a>() -> &'a str {
    let rnum = rand::random::<usize>();
    BODY[rnum % BODY.len()]
}

pub async fn app_reigster(_req: HttpRequest) -> impl Responder {
    println!("[app_register]");

    let c = Paragraph {
        brief: msg().to_owned(),
        text: msg_body().to_owned()
    };

    let m = match serde_json::to_string(&c) {
        Ok(val) => val,
        Err(_) => String::from(r#"{"brief":"error","text":"Maybe convert error!"}"#)
    };

    HttpResponse::Ok().body(m)
}

pub async fn app_start() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}
