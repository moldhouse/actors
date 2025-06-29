mod counter;
mod runtime;

pub use counter::Counter;
pub use runtime::Runtime;

use actix::{Actor, Addr};

pub struct App {
    _counter: Addr<Counter>,
    _runtime: Addr<Runtime>,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let counter = Counter::new().start();
        let _runtime = Runtime::new(counter.clone()).start();
        Self {
            _counter: counter,
            _runtime,
        }
    }
}
