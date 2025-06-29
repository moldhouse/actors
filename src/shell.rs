use axum::{Json, Router, extract::State, routing::get};
use std::net::SocketAddr;
use tokio::{net::TcpListener, task::JoinHandle};

use crate::counter::CounterApi;

pub struct Shell {
    handle: JoinHandle<()>,
}

#[derive(Clone)]
pub struct AppState<T> {
    counter: T,
}

impl<T> AppState<T> {
    pub fn new(counter: T) -> Self {
        Self { counter }
    }
}

impl Shell {
    pub async fn new<T>(
        addr: impl Into<SocketAddr>,
        app_state: AppState<T>,
        shutdown_signal: impl Future<Output = ()> + Send + 'static,
    ) -> Self
    where
        T: Clone + Send + Sync + 'static,
        T: CounterApi,
    {
        let addr = addr.into();
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Listening on {addr}");

        let handle = tokio::spawn(async move {
            let _ = axum::serve(listener, http(app_state))
                .with_graceful_shutdown(shutdown_signal)
                .await;
        });
        Self { handle }
    }

    pub async fn wait_for_shutdown(self) {
        self.handle.await.unwrap();
    }
}

fn http<T>(app_state: AppState<T>) -> Router
where
    T: Clone + Send + Sync + 'static,
    T: CounterApi,
{
    Router::new()
        .route("/", get(read_counter))
        .with_state(app_state)
        .route("/health", get(async || "ok"))
}

async fn read_counter<T>(State(AppState { counter }): State<AppState<T>>) -> Json<usize>
where
    T: CounterApi,
{
    let value = counter.read().await;
    Json(value)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::util::ServiceExt;

    use crate::{
        counter::CounterApi,
        shell::{AppState, http},
    };

    #[tokio::test]
    async fn counter_is_read() {
        // Given a counter api that always returns 42
        #[derive(Clone)]
        struct CounterApiMock;

        impl CounterApi for CounterApiMock {
            async fn ping(&self, _increment: usize) -> usize {
                unimplemented!()
            }

            async fn read(&self) -> usize {
                42
            }
        }
        let app_state = AppState::new(CounterApiMock);
        let http = http(app_state);

        // When doing a GET request to read the counter
        let req = Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::empty())
            .unwrap();
        let resp = http.oneshot(req).await.unwrap();

        // Then the counter is read
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(String::from_utf8(body.to_vec()).unwrap(), "42".to_owned());
    }
}
