use axum::{response::Html, routing::get, Router};
use opentelemetry::global;
use std::net::{Ipv4Addr, SocketAddr};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("axum_test")
        .install_simple()?;

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(fmt::Layer::default())
        .try_init()?;

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7878));
    axum::Server::bind(&addr)
        .serve(Router::new().route("/", get(root_get)).into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[tracing::instrument]
async fn root_get() -> Html<&'static str> {
    tracing::info!("getting root");
    html().await
}

#[tracing::instrument]
async fn html() -> Html<&'static str> {
    tracing::info!("getting index html");
    Html(include_str!("index.html"))
}
