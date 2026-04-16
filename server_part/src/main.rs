use tower_http::cors::CorsLayer;
use axum::{
    routing::{get, post},
    Router,
};
use tokio::fs::{OpenOptions, read_to_string};
use tokio::io::AsyncWriteExt;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::net::TcpListener;
struct AppState {
    tx: broadcast::Sender<String>,
}
#[tokio::main]
async fn main() {
let app = Router::new()
    .route("/send-text", post(accept_text))
    .route("/get-state", get(get_state))
    .layer(CorsLayer::permissive());
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("server is started");
    axum::serve(listener, app).await.unwrap();

}
async fn accept_text(body : String){
    if body == "ON"{
        let mut file = OpenOptions::new()
        .write(true)
            .create(true)
        .truncate(true)
            .open("state.txt").await.unwrap();
        file.write_all(b"ON").await.unwrap();

    }else if body == "OFF"{
        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("state.txt").await.unwrap();
        file.write_all(b"OFF").await.unwrap();
    
    }
}
async fn get_state() -> String {
    match read_to_string("state.txt").await {
        Ok(s) => s,
        Err(_) => "ERROR".to_string(),
    }
}



