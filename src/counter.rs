use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

/// Public interface for the counter actor.
pub trait CounterApi: Send + Sync + 'static {
    fn ping(&self, increment: usize) -> impl Future<Output = usize> + Send;
}

impl CounterApi for mpsc::Sender<CounterMsg> {
    async fn ping(&self, increment: usize) -> usize {
        let (send, recv) = oneshot::channel();
        self.send(CounterMsg::Ping { increment, send })
            .await
            .unwrap();
        recv.await.unwrap()
    }
}

/// Handle to the counter actor. Spin this up in order to use the counter API.
pub struct Counter {
    send: mpsc::Sender<CounterMsg>,
    handle: JoinHandle<()>,
}

impl Counter {
    pub fn new() -> Self {
        let (send, recv) = mpsc::channel(1);
        let actor = CounterActor::new(recv);
        let handle = tokio::spawn(async move {
            actor.run().await;
        });

        Self { send, handle }
    }

    pub fn api(&self) -> impl CounterApi {
        self.send.clone()
    }

    pub async fn wait_for_shutdown(self) {
        drop(self.send);
        self.handle.await.unwrap();
    }
}

/// An actor that can be pinged and will increment its count.
struct CounterActor {
    recv: mpsc::Receiver<CounterMsg>,
    count: usize,
}

impl CounterActor {
    pub fn new(recv: mpsc::Receiver<CounterMsg>) -> Self {
        Self { recv, count: 0 }
    }

    pub async fn run(mut self) {
        while let Some(msg) = self.recv.recv().await {
            self.act(msg);
        }
    }

    pub fn act(&mut self, msg: CounterMsg) {
        match msg {
            CounterMsg::Ping { increment, send } => {
                self.count += increment;
                send.send(self.count).unwrap();
            }
        }
    }
}

pub enum CounterMsg {
    Ping {
        increment: usize,
        send: oneshot::Sender<usize>,
    },
}
