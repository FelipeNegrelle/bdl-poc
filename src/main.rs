use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    let json_req = r#"
    {
      "track-txs": [
        "8a4666c6d22ce74fa47e1c4fdb09af556a234cc6a606539a75caf66ba44a2d07",
        "941df06064c290b4627e92bdbf3bff7c0e97aab33e273c2a20404f9cfd21b607"
      ]
    }
    "#;

    let req: TxRequest = serde_json::from_str(json_req)?;

    let handle = tokio::spawn(async move {
        if let Err(e) = api::run_websocket(req).await {
            eprintln!("WebSocket error: {:?}", e);
        }
    });

    signal::ctrl_c()
        .await
        .expect("Failed to listen for shutdown signal");
    println!("Shutting down...");

    handle.abort();

    Ok(())
}

pub mod api {
    use crate::{TxRequest, TxResponse};
    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};

    pub const ADDRESS: &str = "wss://mempool.space/api/v1/ws";

    pub async fn run_websocket(
        tx_request: TxRequest,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        let (mut ws_stream, _) = connect_async(ADDRESS).await.expect("Failed to connect");

        ws_stream
            .send(Message::Text(Utf8Bytes::from(
                serde_json::to_string(&tx_request).unwrap(),
            )))
            .await
            .expect("Failed to send message");

        while let Some(message) = ws_stream.next().await {
            match message {
                Ok(Message::Text(text)) => match serde_json::from_str::<TxResponse>(&text) {
                    Ok(response) => {
                        println!("Received update: {:?}", response);
                    }
                    Err(_) => {
                        println!("Received non-JSON message: {}", text);
                    }
                },

                Ok(Message::Close(_)) => {
                    println!("Connection closed by server");
                    break;
                }

                Ok(Message::Ping(payload)) => {
                    ws_stream.send(Message::Pong(payload)).await.ok();
                }

                _ => {
                    eprintln!("Unexpected message type");
                }
            }
        }

        Ok(())
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
