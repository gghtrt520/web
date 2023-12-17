use tokio::net::TcpListener;
use tracing::Level;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_ansi(true)
        .init();
    tracing::info!("Listening on{:?}", listener);
    axum::serve(listener, api::routes().into_make_service())
        .await
        .unwrap();
}
