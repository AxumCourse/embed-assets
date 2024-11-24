use axum::{
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
};
use rust_embed::Embed;

pub const INDEX_HTML: &'static str = "index.html";

#[derive(Embed)]
#[folder = "dist/"]
pub struct Assets;

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches("/");

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }
            index_html().await
        }
    }
}

pub async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

pub async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}
