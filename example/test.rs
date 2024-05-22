use futures_util::{SinkExt, StreamExt};
use tokio::{
    io::{stdin, AsyncBufReadExt, BufReader},
    try_join,
};
use tokio_tungstenite::tungstenite::Message;

pub async fn run(url: &str) -> eyre::Result<()> {
    let (socket, _) = tokio_tungstenite::connect_async(url).await?;

    println!("success connect");
    let (mut write_ws, mut read_ws) = socket.split();

    let write_handle = tokio::spawn(async move {
        let mut reader = BufReader::new(stdin()).lines();

        while let Some(line) = reader.next_line().await.expect("Failed to get stdin") {
            if !line.trim().is_empty() {
                let msg = Message::text(line);
                write_ws
                    .send(msg)
                    .await
                    .expect("ERROR : failed to send message");
            }
        }
    });

    let read_handle = tokio::spawn(async move {
        while let Some(incoming_msg) = read_ws.next().await {
            match incoming_msg {
                Ok(message) => println!("Received message : {}", message),
                Err(e) => eprintln!("ERROR: {}", e),
            }
        }
    });

    let _ = try_join!(read_handle, write_handle);

    Ok(())
}
