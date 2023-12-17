use axum::routing::get;
use socketioxide::{extract::SocketRef, SocketIo};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World" }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    info!("Starting server");

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
