use {
    axum::{
        extract::ws::{WebSocket, WebSocketUpgrade},
        response::IntoResponse,
        response::Json,
        routing::get,
        Router,
    },
    serde::{Deserialize, Serialize},
    std::net::SocketAddr,
};

pub fn app() -> Router {
    Router::new().route("/ws", get(handler))
}
pub async fn run() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening?");
    hyper::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    UniversalInput { input: String },
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    UniversalInput { results: Vec<Result> },
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub key: usize,
    pub message: String,
}

async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    println!("handler?");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("handler socket..");
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            println!("got msg");
            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}
