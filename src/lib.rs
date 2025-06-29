use crate::{counter::Counter, runtime::Runtime};

mod counter;
mod runtime;

pub struct App {
    counter: Counter,
    runtime: Runtime,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let counter = Counter::new();
        let runtime = Runtime::new(counter.api());
        Self { counter, runtime }
    }

    pub async fn wait_for_shutdown(
        self,
        shutdown_signal: impl Future<Output = ()> + Send + 'static,
    ) {
        shutdown_signal.await;

        // Shutdown actors in reverse order of creation
        self.runtime.wait_for_shutdown().await;
        self.counter.wait_for_shutdown().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn app_listens_for_shutdown_signal() {
        let app = App::new();
        app.wait_for_shutdown(async {}).await;
    }
}
