use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, OriginalUri},
    http::{request::Parts, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    Extension, Json,
};
use indoc::formatdoc;
use sha256::digest as sha256;
use std::sync::Arc;

use crate::frontend::vite::{self, Vite};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Page {
    pub url: String,
    pub component: &'static str,
    pub props: serde_json::Value,
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Inertia {
    vite: Arc<Vite>,
    version: Option<String>,
    request: Option<Request>,
}

impl Inertia {
    pub fn new() -> Result<Self, vite::Error> {
        let vite = Vite::new()?;

        let version = if let Vite::Production { manifest } = &vite {
            Some(sha256(serde_json::to_string(&manifest).unwrap()).to_string())
        } else {
            None
        };

        Ok(Inertia {
            version,
            request: None,
            vite: Arc::new(vite),
        })
    }

    pub fn extension(self) -> Extension<Self> {
        Extension(self)
    }

    pub fn redirect(&self, url: String) -> impl IntoResponse {
        (StatusCode::CONFLICT, [("X-Inertia-Location", url)])
    }

    pub fn render<T: serde::Serialize>(&self, component: &'static str, props: T) -> Response {
        let request = self.request.clone().unwrap();

        let page = Page {
            component,
            url: request.path.clone(),
            version: self.version.clone(),
            props: serde_json::to_value(props).unwrap(),
        };

        Response {
            page,
            request,
            vite: self.vite.clone(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Inertia
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let method = Method::from_request_parts(parts, state)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR).into_response())?;

        let Extension(mut inertia) = Extension::<Self>::from_request_parts(parts, state)
            .await
            .unwrap();

        let request = Request::from_request_parts(parts, state)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR).into_response())?;

        inertia.request = Some(request.clone());

        if matches!(method, Method::GET) && inertia.version != request.version {
            return Err(inertia.redirect(request.path).into_response());
        }

        Ok(inertia)
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    path: String,
    is_xhr: bool,
    version: Option<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for Request
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state).await.unwrap();
        let OriginalUri(path) = OriginalUri::from_request_parts(parts, state).await.unwrap();

        Ok(Self {
            path: path.to_string(),
            version: headers
                .get("X-Inertia-Version")
                .map(|header| header.to_str().map(|s| s.to_string()).unwrap_or_default()),
            is_xhr: headers
                .get("X-Inertia")
                .map(|header| header == "true")
                .unwrap_or(false),
        })
    }
}

pub struct Response {
    page: Page,
    vite: Arc<Vite>,
    request: Request,
}

impl Response {
    fn html_page(&self) -> String {
        formatdoc! {r#"
            <!doctype html>
            <html lang="en">
                <head>
                    <title>PingCRM</title>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    {}{}
                </head>
                <body>
                    <div id="app" data-page='{}'></div>
                </body>
            </html>
        "#, self.vite.dev_scripts().unwrap_or_default(), self.vite.asset("src/index.tsx").unwrap(), serde_json::to_string(&self.page).unwrap()}
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        if self.request.is_xhr {
            Json(self.page).into_response()
        } else {
            Html(Self::html_page(&self)).into_response()
        }
    }
}
