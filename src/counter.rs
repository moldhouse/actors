use actix::prelude::*;

/// Public interface for the counter actor.
pub trait CounterApi: Send + Sync + 'static {
    fn read(&self) -> impl Future<Output = usize> + Send;
}

impl CounterApi for Addr<Counter> {
    async fn read(&self) -> usize {
        self.send(Ping(0)).await.unwrap()
    }
}

/// An actor that can be pinged and will increment its count.
pub struct Counter {
    count: usize,
}

impl Counter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Actor for Counter {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Ping(pub usize);

impl Handler<Ping> for Counter {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}
