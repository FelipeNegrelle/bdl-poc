// Calls all available REST helpers in MempoolClient against mempool.space.
// Usage examples:
//   cargo run --example rest_all -- --height 840000 --address bc1...
//   cargo run --example rest_all -- --block-hash <hash>
// Flags:
//   --height <u32>         Query block by height
//   --block-hash <hash>    Query block by hash
//   --address <addr>       Query address info and transactions
//   --after-txid <txid>    Pass after_txid to address transactions
//   --rest-v1              Use v1/address/... endpoint for address txs
// If no flags are provided, only fees and mempool txids are fetched.
use std::env;

use bdl_poc::mempool::client::MempoolClient;

const REST_ENDPOINT: &str = "https://mempool.space/api/";

#[derive(Default)]
struct Args {
    height: Option<u32>,
    block_hash: Option<String>,
    address: Option<String>,
    after_txid: Option<String>,
    rest_v1: bool,
}

fn parse_args() -> Args {
    let mut args = env::args().skip(1);
    let mut cfg = Args::default();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--height" => {
                if let Some(h) = args.next() {
                    cfg.height = h.parse().ok();
                }
            }
            "--block-hash" => {
                if let Some(h) = args.next() {
                    cfg.block_hash = Some(h);
                }
            }
            "--address" => {
                if let Some(a) = args.next() {
                    cfg.address = Some(a);
                }
            }
            "--after-txid" => {
                if let Some(t) = args.next() {
                    cfg.after_txid = Some(t);
                }
            }
            "--rest-v1" => cfg.rest_v1 = true,
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => {
                eprintln!("Unknown flag: {other}");
                print_help();
                std::process::exit(1);
            }
        }
    }
    cfg
}

fn print_help() {
    println!("See top-of-file comments for usage.");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = parse_args();

    let mut client = MempoolClient::connect(None, Some(REST_ENDPOINT)).await?;

    let fees = client.get_fees().await?;
    println!("fees: {fees:?}");

    let txids = client.get_tx_ids().await?;
    println!(
        "mempool txids: count={} sample={:?}",
        txids.len(),
        txids.get(0)
    );

    if let Some(height) = cfg.height {
        match client.get_block_by_height(height).await {
            Ok(block) => println!(
                "block height {height}: hash={} txs={}",
                block.id, block.tx_count
            ),
            Err(e) => eprintln!("block height {height} error: {e}"),
        }
    }

    if let Some(hash) = cfg.block_hash {
        match client.get_block_by_hash(&hash).await {
            Ok(block) => println!(
                "block {}: height={} txs={}",
                block.id, block.height, block.tx_count
            ),
            Err(e) => eprintln!("block {hash} error: {e}"),
        }
    }

    if let Some(address) = cfg.address {
        match client.get_address_info(&address).await {
            Ok(info) => println!(
                "address info {}: txs={} funded={} spent={}",
                address,
                info.chain_stats.tx_count,
                info.chain_stats.funded_txo_sum,
                info.chain_stats.spent_txo_sum
            ),
            Err(e) => eprintln!("address info error: {e}"),
        }

        match client
            .get_address_transactions(&address, cfg.after_txid.as_deref(), cfg.rest_v1)
            .await
        {
            Ok(txs) => println!(
                "address txs {}: count={} first={:?}",
                address,
                txs.len(),
                txs.get(0).map(|t| &t.txid)
            ),
            Err(e) => eprintln!("address txs error: {e}"),
        }
    }

    client.close().await?;
    Ok(())
}
