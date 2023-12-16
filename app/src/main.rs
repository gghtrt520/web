use tokio::net::TcpListener;



#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, api::routes().into_make_service()).await.unwrap();
}