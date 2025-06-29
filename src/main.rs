use std::future::pending;

use actors::App;

fn main() {
    let fut = async move {
        App::new();
        pending::<()>().await;
    };
    actix::System::new().block_on(fut);
}
