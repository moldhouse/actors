mod counter;
mod runtime;

use actix::Actor;
use tokio::signal;

use crate::{counter::Counter, runtime::Runtime};

#[actix::main]
async fn main() {
    let addr = Counter::new().start();
    let runtime = Runtime::new(addr);

    runtime.start();
    signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
}
