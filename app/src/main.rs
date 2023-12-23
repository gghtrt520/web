use tokio::net::TcpListener;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_ansi(true)
        .init();

    dotenv::dotenv().ok();
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    let store = store::Store::new().await.unwrap();

    tracing::info!("Listening on{:?}", listener);
    axum::serve(listener, api::routes(store).into_make_service())
        .await
        .unwrap();
}
