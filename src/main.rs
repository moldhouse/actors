use actors::{App, AppConfig};

fn main() {
    let fut = async move {
        let config = AppConfig::default();
        let _app = App::new(config).await;
    };
    actix::System::new().block_on(fut);
}
