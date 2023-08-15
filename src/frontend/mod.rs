use axum::{
    http::{header, StatusCode, Uri},
    response::IntoResponse,
    Router,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "frontend/dist/"]
struct StaticFiles;

mod inertia;
mod vite;

pub use inertia::Inertia;

pub fn serve_assets() -> Router {
    Router::new().fallback(|uri: Uri| async move {
        let path = uri.path().trim_start_matches('/');

        match StaticFiles::get(&format!("assets/{path}")) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();

                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404").into_response(),
        }
    })
}
