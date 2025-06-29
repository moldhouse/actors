use std::net::SocketAddr;

use crate::{
    counter::Counter,
    runtime::Runtime,
    shell::{AppState, Shell},
};

mod counter;
mod runtime;
mod shell;

pub struct App {
    counter: Counter,
    runtime: Runtime,
    shell: Shell,
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
    pub async fn new(
        config: AppConfig,
        shutdown_signal: impl Future<Output = ()> + Send + 'static,
    ) -> Self {
        let counter = Counter::new();
        let runtime = Runtime::new(counter.api());
        let app_state = AppState::new(counter.api());
        let shell = Shell::new(config.addr, app_state, shutdown_signal).await;
        Self {
            counter,
            runtime,
            shell,
        }
    }

    pub async fn wait_for_shutdown(self) {
        // Shutdown actors in reverse order of creation
        self.shell.wait_for_shutdown().await;
        self.runtime.wait_for_shutdown().await;
        self.counter.wait_for_shutdown().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn app_listens_for_shutdown_signal() {
        // Given an app with a shutdown signal that instantly resolves
        let config = AppConfig::default();
        let app = App::new(config, async {}).await;

        // Then the app should shutdown
        app.wait_for_shutdown().await;
    }
}
