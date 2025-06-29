use std::future::pending;

use actors::App;

fn main() {
    actix::System::new().block_on(async move {
        {
            App::new();
            pending::<()>().await;
        }
    })
}
