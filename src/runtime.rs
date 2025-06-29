use std::time::Duration;

use actix::prelude::*;

use crate::{Counter, counter::Ping};

pub struct Runtime {
    counter: Addr<Counter>,
}

impl Runtime {
    pub fn new(counter: Addr<Counter>) -> Self {
        Self { counter }
    }

    pub fn ping(&self) -> impl ActorFuture<Self, Output = ()> + 'static {
        self.counter
            .send(Ping(10))
            .into_actor(self)
            .map(|res, _, _| {
                println!("Result: {}", res.unwrap());
            })
    }
}

impl Actor for Runtime {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            ctx.spawn(act.ping());
        });
    }
}
