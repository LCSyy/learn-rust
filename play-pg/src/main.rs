// Rust访问pg数据库
// 增删查改

use std::env;
use postgres::{
    Client,
    NoTls,
};


fn main() {
    let mut client = Client::connect("postgresql://postgres:pg2020@localhost:5432/postgres", NoTls).expect("Connect error");

    let args = env::args();
    for arg in args {
        match arg.as_str() {
            "add" => new_record(&mut client),
            _ => {}
        }
    }

    let mut accounts = vec![];
    for row in client.query("SELECT * FROM accounts;",&[]).expect("No rows") {
        accounts.push(model::Accounts {
            id:              row.get(0),
            account:         row.get(1),
            passwd:          row.get(2),
            description:     row.get(3),
            create_datetime: row.get(4),
        });
    }

    println!("count: {}", accounts.len());
}

mod model {
    use std::time::SystemTime;
    use postgres::types::Timestamp;
    
    pub struct Accounts {
        pub id: i64,
        pub account: String,
        pub passwd: String,
        pub description: String,
        pub create_datetime: Timestamp<SystemTime>,
    }
}

fn new_record(db: &mut Client) {
    db.execute("INSERT INTO accounts (account,passwd,description) VALUES ('lcs',md5('123456'),'This is my account.')", &[]).expect("Insert error");
}

