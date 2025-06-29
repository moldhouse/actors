mod counter;
mod runtime;
mod shell;

use std::net::SocketAddr;

pub use counter::Counter;
pub use runtime::Runtime;

use actix::{Actor, Addr};

use crate::shell::{AppState, Shell};

pub struct App {
    _counter: Addr<Counter>,
    _runtime: Addr<Runtime>,
    _shell: Shell,
}

pub struct AppConfig {
    addr: SocketAddr,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:8080".parse().unwrap(),
        }
    }
}

impl App {
    #[allow(clippy::new_without_default)]
    pub async fn new(config: AppConfig) -> Self {
        let counter = Counter::new().start();
        let _runtime = Runtime::new(counter.clone()).start();
        let app_state = AppState::new(counter.clone());
        let _shell = Shell::new(config.addr, app_state).await;
        Self {
            _counter: counter,
            _runtime,
            _shell,
        }
    }
}
