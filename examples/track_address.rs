// Connects to mempool.space WS, subscribes to an address, and prints address events.
// Usage: cargo run --example track_address -- <bech32-address>
// If no address is provided, a default demo address is used. Press Ctrl+C to stop.
use std::env;

use bdl_poc::mempool::client::MempoolClient;
use tokio::signal;

const WS_ENDPOINT: &str = "wss://mempool.space/api/v1/ws";
const DEFAULT_ADDRESS: &str = "bc1qe6xgeeu2jjazv9x406cwt8y47nw5hl06f507ds";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_ADDRESS.to_string());

    let mut client = MempoolClient::connect(Some(WS_ENDPOINT), None).await?;
    println!("Connected to {WS_ENDPOINT}");

    client.track_address(&address).await?;
    println!("Subscribed to address {address}");
    println!("Waiting for events... (Ctrl+C to exit)");

    tokio::select! {
        res = pump_events(&mut client) => res?,
        _ = signal::ctrl_c() => {
            println!("\nReceived Ctrl+C, shutting down...");
        }
    }

    client.close().await?;
    Ok(())
}

async fn pump_events(client: &mut MempoolClient) -> Result<(), Box<dyn std::error::Error>> {
    while let Some(response) = client.receive_address_response().await? {
        let txs = response.transactions();
        println!("event={} txs={}", response.kind(), txs.len());
        for tx in txs {
            println!("  txid={} fee={} vsize={:?}", tx.txid, tx.fee, tx.vsize);
        }
    }

    println!("Socket closed by server");
    Ok(())
}
