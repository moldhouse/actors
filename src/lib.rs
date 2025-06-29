mod counter;
mod runtime;

pub use counter::Counter;
pub use runtime::Runtime;

use actix::{Actor, Addr};

pub struct App {
    counter: Addr<Counter>,
    runtime: Addr<Runtime>,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let counter = Counter::new().start();
        let runtime = Runtime::new(counter.clone()).start();
        Self { counter, runtime }
    }

    pub async fn wait_for_shutdown(
        self,
        shutdown_signal: impl Future<Output = ()> + Send + 'static,
    ) {
        shutdown_signal.await;
    }
}
