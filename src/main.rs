use axum::{response::Html, routing::get, Router};
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // let subscriber = tracing_subscriber::fmt()
    //     .with_file(true)
    //     .with_line_number(true)
    //     .with_target(true)
    //     .with_thread_names(true)
    //     .finish();
    // tracing::subscriber::set_global_default(subscriber)?;
    console_subscriber::init();

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
