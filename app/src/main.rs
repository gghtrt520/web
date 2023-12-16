use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tracing_subscriber::fmt::init();
    tracing::info!("Listening on{:?}", listener);
    axum::serve(listener, api::routes().into_make_service())
        .await
        .unwrap();
}
