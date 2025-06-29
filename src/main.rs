mod counter;
use actix::{Actor, System};

use crate::counter::{Counter, Ping};

#[actix::main]
async fn main() {
    let addr = Counter::new().start();
    let res = addr.send(Ping(10)).await;

    println!("Result: {}", res.unwrap());

    System::current().stop();
}
