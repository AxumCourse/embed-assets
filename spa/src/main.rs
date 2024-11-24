use axum::{routing::get, Router};
use chrono::Local;
use tokio::net::TcpListener;

mod asset;

#[tokio::main]
async fn main() {
    let tcp_listener = TcpListener::bind("0.0.0.0:9527").await.unwrap();
    let app = Router::new()
        .route("/now", get(now_handler))
        .fallback(asset::static_handler);

    axum::serve(tcp_listener, app).await.unwrap();
}

async fn now_handler() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
