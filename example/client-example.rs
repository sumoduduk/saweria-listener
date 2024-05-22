use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;

// Define a struct to hold the WebSocket connection
struct AppState {
    socket: Arc<Mutex<Option<WebSocketStream>>>,
}

#[tokio::main]
async fn main() {
    // Create a new WebSocket connection
    let (socket, response) = tokio_tungstenite::connect_async("https://examplewebsocket.com/chat")
        .await
        .unwrap();

    // Check if the connection was successful
    if response.status().is_success() {
        // Create a shared state for the application
        let socket = Arc::new(Mutex::new(Some(socket)));
        let app_state = AppState { socket };

        // Run the Axum application
        let app = Router::new().route("/send", get(send_pong));
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service_with_connect_info::<AppState>(move |_| app_state.clone()))
            .await
            .unwrap();

        // Listen for messages from the WebSocket server
        listen_for_messages(socket.clone()).await;
    } else {
        eprintln!("Failed to connect to WebSocket server");
    }
}

async fn send_pong(State(state): State<AppState>) {
    // Send a "pong" message to the WebSocket server
    if let Some(socket) = state.socket.lock().unwrap() {
        let pong_message = Message::Text(String::from("pong"));
        socket.write_message(pong_message).await.unwrap();
    }
}

async fn listen_for_messages(socket: Arc<Mutex<Option<WebSocketStream>>>) {
    loop {
        let mut socket = match socket.lock().unwrap() {
            Some(socket) => socket,
            None => break,
        };

        match socket.read_message().await {
            Ok(msg) => match msg {
                Message::Close(code, reason) => {
                    println!(
                        "Server closed the connection. Code: {}, Reason: {:?}",
                        code, reason
                    );
                    break;
                }
                Message::Text(text) => {
                    println!("Received text message: {}", text);
                }
                Message::Binary(bin) => {
                    println!("Received binary message of length {}", bin.len());
                }
                Message::Ping(_) => {
                    println!("Received ping");
                }
                Message::Pong(_) => {
                    println!("Received pong");
                }
                Message::Continuation(_) => {
                    println!("Received a continuation frame");
                }
                Message::Error(err) => {
                    println!("Error reading message: {}", err);
                    break;
                }
            },
            Err(e) => {
                println!("Error reading message: {}", e);
                break;
            }
        }
    }
}
