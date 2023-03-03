use std::net::{Ipv4Addr, SocketAddr};

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7878));
    axum::Server::bind(&addr)
        .serve(Router::new().route("/", get(root_get)).into_make_service())
        .await
        .unwrap()
}

async fn root_get() -> Html<&'static str> {
    Html(include_str!("index.html"))
}
