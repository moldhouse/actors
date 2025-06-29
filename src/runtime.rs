use std::time::Duration;

use tokio::{select, task::JoinHandle};

use crate::counter::CounterApi;

pub struct Runtime {
    handle: JoinHandle<()>,
    // Channel to communicate a shutdown signal to the runtime actor
    shutdown: tokio::sync::watch::Sender<bool>,
}

impl Runtime {
    pub fn new(counter: impl CounterApi) -> Self {
        let (shutdown, receiver) = tokio::sync::watch::channel(false);
        let mut actor = RuntimeActor::new(counter, receiver);
        let handle = tokio::spawn(async move {
            actor.run().await;
        });
        Self { handle, shutdown }
    }

    pub async fn wait_for_shutdown(self) {
        self.shutdown.send(true).unwrap();
        self.handle.await.unwrap();
    }
}

pub struct RuntimeActor<C: CounterApi> {
    counter: C,
    shutdown: tokio::sync::watch::Receiver<bool>,
}

impl<C: CounterApi> RuntimeActor<C> {
    pub fn new(counter: C, shutdown: tokio::sync::watch::Receiver<bool>) -> Self {
        Self { counter, shutdown }
    }

    pub async fn run(&mut self) {
        loop {
            select! {
                _ = self.shutdown.changed() => break,
                () = tokio::time::sleep(Duration::from_secs(5)) => (),
            };
            let count = self.counter.ping(10).await;
            println!("Count: {count}");
        }
    }
}
