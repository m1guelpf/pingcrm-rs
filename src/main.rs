use std::{env, net::SocketAddr};

use anyhow::Result;
use axum::{response::IntoResponse, routing::get, Router};
use frontend::Inertia;
use tracing::info;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};

mod frontend;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "pingcrm=info".into()),
        ))
        .init();

    let inertia = Inertia::new()?;

    let app = Router::new()
        .route("/", get(hello_world))
        .nest("/assets", frontend::serve_assets())
        .layer(inertia.extension());

    let address = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("PORT").map_or(8000, |p| p.parse().unwrap()),
    ));
    info!("⚡ PingCRM started on http://{address}");

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn hello_world(inertia: Inertia) -> impl IntoResponse {
    inertia.render("Auth/Login", ())
}
