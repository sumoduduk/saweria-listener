use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::try_join;

#[derive(Debug, Serialize, Deserialize)]
struct Media {
    tag: String,
    src: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DonationData {
    donator: String,
    currency: String,
    amount: u32,
    message: String,
    sound: Option<String>,
    media: Option<Media>,
    ts: Option<String>,
    is_user: bool,
    is_message_flagged: bool,
    is_name_flagged: bool,
    is_replay: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Donation {
    data: Option<Vec<DonationData>>,
    r#type: String,
}

pub async fn run(url: &str) -> eyre::Result<()> {
    let (socket, _) = tokio_tungstenite::connect_async(url).await?;

    println!("success connect");
    let (_, mut read_ws) = socket.split();

    let read_handle = tokio::spawn(async move {
        while let Some(incoming_msg) = read_ws.next().await {
            match incoming_msg {
                Ok(message) => {
                    let bytes_message = message.into_text().expect("ERROR: failed to text");

                    let obj: Donation = serde_json::from_str(&bytes_message)
                        .expect("ERROR: failed to deserialize donation");

                    dbg!(&obj);

                    let str_obj = serde_json::to_string_pretty(&obj).unwrap();

                    println!("{}", str_obj);
                }
                Err(e) => {
                    eprintln!("ERROR: {}", e);
                }
            }
        }
    });

    let _ = try_join!(read_handle);

    Ok(())
}
