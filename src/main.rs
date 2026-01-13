use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::signal;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    use api;

    let json_req = r#"
    {
      "track-txs": [
        "8a4666c6d22ce74fa47e1c4fdb09af556a234cc6a606539a75caf66ba44a2d07",
        "941df06064c290b4627e92bdbf3bff7c0e97aab33e273c2a20404f9cfd21b607"
      ]
    }
    "#;

    let req: TxRequest = serde_json::from_str(json_req)?;

    tokio::select! {
        _ = api::track_transactions(req) => {
            println!("Tracking task completed.");
        }
        _ = signal::ctrl_c() => {
            println!("Received shutdown signal. Exiting...");
        }
    }

    Ok(())
}

pub mod api {
    use crate::TxRequest;
    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};

    pub const ADDRESS: &str = "wss://mempool.space/api/v1/ws";

    pub async fn track_transactions(tx_request: TxRequest) {
        let (mut ws_stream, _) = match connect_async(ADDRESS).await {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to connect: {:?}", e);
                return;
            }
        };

        let message = match serde_json::to_string(&tx_request) {
            Ok(msg) => Message::Text(Utf8Bytes::from(msg)),
            Err(e) => {
                eprintln!("Failed to serialize request: {:?}", e);
                return;
            }
        };

        if let Err(e) = ws_stream.send(message).await {
            eprintln!("Failed to send message: {:?}", e);
            return;
        }

        println!("Connected and sent tracking request. Listening for updates...");

        while let Some(message) = ws_stream.next().await {
            match message {
                Ok(Message::Text(text)) => match serde_json::from_str::<crate::TxResponse>(&text) {
                    Ok(response) => {
                        println!("Received transaction update: {:?}", response);
                    }
                    Err(_) => {
                        println!("Received non-JSON message: {}", text);
                    }
                },
                Ok(Message::Close(frame)) => {
                    println!("Connection closed by server: {:?}", frame);
                    break;
                }
                Ok(Message::Ping(payload)) => {
                    if let Err(e) = ws_stream.send(Message::Pong(payload)).await {
                        eprintln!("Failed to send pong: {:?}", e);
                    }
                }
                Ok(Message::Pong(_)) => {}
                Ok(Message::Binary(data)) => {
                    println!("Received binary data: {:?}", data);
                }
                Ok(_) => {
                    println!("Received unhandled message type");
                }
                Err(e) => {
                    eprintln!("Error receiving message: {:?}", e);
                    break;
                }
            }
        }

        println!("WebSocket connection ended.");
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxRequest {
    #[serde(rename = "track-txs")]
    pub track_txs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxResponse {
    #[serde(rename = "tracked-txs")]
    pub tracked_txs: HashMap<String, TxInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxInfo {
    pub position: Position,
    pub cpfp: Option<Cpfp>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub block: u64,
    pub vsize: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpfpEntry {
    pub txid: String,
    pub fee: i64,
    pub weight: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cpfp {
    pub ancestors: Vec<CpfpEntry>,
    #[serde(rename = "bestDescendant")]
    pub best_descendant: Option<CpfpEntry>,
    pub descendants: Vec<CpfpEntry>,
    #[serde(rename = "effectiveFeePerVsize")]
    pub effective_fee_per_vsize: f64,
    pub sigops: u64,
    #[serde(rename = "adjustedVsize")]
    pub adjusted_vsize: u64,
}
