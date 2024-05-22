use dotenvy::dotenv;

const WS_URL: &str = "wss://events.saweria.co/stream";

#[tokio::main]

async fn main() {
    dotenv().ok();

    let stream_key = std::env::var("STREAM_KEY").expect("ERROR: STREAM_KEY not found");

    let uri = format!("{WS_URL}?streamKey={stream_key}");

    saweria_axum_listener::run(&uri)
        .await
        .expect("ERROR : main error");
}
