use askama::Template;
use axum::{response::Html, routing::get, Router};
use chrono::Local;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "templates/"]
struct Asset;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    now: String,
}

#[tokio::main]
async fn main() {
    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:9527").await.unwrap();
    let app = Router::new().route("/", get(index_handler));
    axum::serve(tcp_listener, app).await.unwrap();
}

async fn index_handler() -> Html<String> {
    let tmp = IndexTemplate {
        now: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    Html(tmp.render().unwrap())
}
