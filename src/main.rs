use axum::{response::Html, routing::get, Router};
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7878));
    tracing_subscriber::fmt::init();
    axum::Server::bind(&addr)
        .serve(Router::new().route("/", get(root_get)).into_make_service())
        .await
        .unwrap()
}

async fn root_get() -> Html<&'static str> {
    tracing::info!("getting index");
    Html(include_str!("index.html"))
}
