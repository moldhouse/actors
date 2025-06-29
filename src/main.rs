use tokio::signal;

use actors::App;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
    };

    tokio::select! {
        _ = ctrl_c => {},
    }
}

#[actix::main]
async fn main() {
    let app = App::new();
    app.wait_for_shutdown(shutdown_signal()).await;
}
