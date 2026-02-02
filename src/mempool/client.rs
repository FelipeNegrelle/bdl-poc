use crate::mempool::types::{
    AddrRequest, AddrResponse, AddressInfo, BlockInfo, FeeResponse, Transaction, TxRequest,
    TxResponse,
};
use crate::rest::rest_client::rest::RestClient;
use crate::ws::websocket_client::websocket::WebsocketClient;
use std::str::FromStr;

pub struct MempoolClient {
    inner_ws: Option<WebsocketClient>,
    inner_rest: Option<RestClient>,
}

impl MempoolClient {
    /// Connect to mempool.space APIs. Both WebSocket and REST addresses are optional, allowing for flexible usage.
    pub async fn connect(
        ws_address: Option<&str>,
        rest_address: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let ws = if let Some(addr) = ws_address {
            Some(WebsocketClient::connect(addr).await?)
        } else {
            None
        };

        let rest = if let Some(addr) = rest_address {
            Some(RestClient::new(addr)?)
        } else {
            None
        };

        Ok(Self {
            inner_ws: ws,
            inner_rest: rest,
        })
    }

    /// Track multiple transactions by their txids
    ///
    /// # Example
    /// ```no_run
    /// # use your_crate::mempool::client::MempoolClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = MempoolClient::connect(Some("wss://mempool.space/api/v1/ws"), Some("https://mempool.space/api/")).await?;
    /// let txids = vec!["txid1".to_string(), "txid2".to_string()];
    /// client.track_transactions(txids).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn track_transactions(
        &mut self,
        txids: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ws) = &mut self.inner_ws {
            let request = TxRequest { track_txs: txids };
            ws.send(&request).await
        } else {
            Err("WebSocket client is not available".into())
        }
    }

    /// Track a Bitcoin address to receive updates about its transactions
    ///
    /// # Example
    /// ```no_run
    /// # use your_crate::mempool::client::MempoolClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = MempoolClient::connect(Some("wss://mempool.space/api/v1/ws"), Some("https://mempool.space/api/")).await?;
    /// client.track_address("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn track_address(
        &mut self,
        address: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ws) = &mut self.inner_ws {
            let request = AddrRequest {
                track_address: address.into(),
            };
            ws.send(&request).await
        } else {
            Err("WebSocket client is not available".into())
        }
    }

    /// Receive and parse transaction tracking response
    pub async fn receive_tx_response(
        &mut self,
    ) -> Result<Option<TxResponse>, Box<dyn std::error::Error>> {
        if let Some(ws) = &mut self.inner_ws {
            match ws.receive().await? {
                Some(text) => {
                    let response: TxResponse = serde_json::from_str(&text)?;
                    Ok(Some(response))
                }
                None => Ok(None),
            }
        } else {
            Err("WebSocket client is not available".into())
        }
    }

    /// Receive and parse address tracking response
    pub async fn receive_address_response(
        &mut self,
    ) -> Result<Option<AddrResponse>, Box<dyn std::error::Error>> {
        if let Some(ws) = &mut self.inner_ws {
            match ws.receive().await? {
                Some(text) => {
                    let response: AddrResponse = serde_json::from_str(&text)?;
                    Ok(Some(response))
                }
                None => Ok(None),
            }
        } else {
            Err("WebSocket client is not available".into())
        }
    }

    /// Receive raw message as string (for debugging or custom parsing)
    pub async fn receive_raw(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        if let Some(ws) = &mut self.inner_ws {
            ws.receive().await
        } else {
            Err("WebSocket client is not available".into())
        }
    }

    /// Send a custom JSON request
    pub async fn send_custom<T: serde::Serialize>(
        &mut self,
        req: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ws) = &mut self.inner_ws {
            ws.send(req).await
        } else {
            Err("WebSocket client is not available".into())
        }
    }

    /// Close the WebSocket connection gracefully
    pub async fn close(self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ws) = self.inner_ws {
            ws.close().await
        } else {
            Ok(())
        }
    }

    /// Get the actual fees from the mempool now
    pub async fn get_fees(&mut self) -> Result<FeeResponse, Box<dyn std::error::Error>> {
        let endpoint = "v1/fees/recommended";

        let fees: FeeResponse = self
            .inner_rest
            .as_ref()
            .ok_or("REST client is not available")?
            .get(endpoint)
            .await?;

        Ok(fees)
    }

    /// Get all transaction IDs currently in the mempool
    pub async fn get_tx_ids(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let endpoint = "mempool/txids";

        let txids: Vec<String> = self
            .inner_rest
            .as_ref()
            .ok_or("REST client is not available")?
            .get(endpoint)
            .await?;

        Ok(txids)
    }

    pub async fn get_block_by_hash(
        &mut self,
        block_hash: &str,
    ) -> Result<BlockInfo, Box<dyn std::error::Error>> {
        let endpoint = format!("block/{}", block_hash);

        let block: BlockInfo = self
            .inner_rest
            .as_ref()
            .ok_or("REST client is not available")?
            .get(&endpoint)
            .await?;

        Ok(block)
    }

    pub async fn get_block_by_height(
        &mut self,
        height: u32,
    ) -> Result<BlockInfo, Box<dyn std::error::Error>> {
        let heigth_endpoint = format!("block-height/{}", height);

        let block_hash: String = self
            .inner_rest
            .as_ref()
            .ok_or("REST client is not available")?
            .get_text(&heigth_endpoint)
            .await?;

        if block_hash.is_empty() {
            Err(format!("Block not found at height {}", height).into())
        } else {
            self.get_block_by_hash(block_hash.as_str()).await
        }
    }

    pub async fn get_address_info(
        &mut self,
        address: &str,
    ) -> Result<AddressInfo, Box<dyn std::error::Error>> {
        let endpoint = format!("address/{}", address);

        let address_info: AddressInfo = self
            .inner_rest
            .as_ref()
            .ok_or("REST client is not available")?
            .get(&endpoint)
            .await?;

        Ok(address_info)
    }

    pub async fn get_address_transactions(
        &mut self,
        address: &str,
        after_txid: Option<&str>,
        is_rest_v1: bool,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
        let mut endpoint = if is_rest_v1 {
            String::from_str("v1/address")?
        } else {
            String::from_str("address")?
        };

        endpoint = if let Some(txid) = after_txid {
            format!("{endpoint}/{}/txs?after_txid={}", address, txid)
        } else {
            format!("{endpoint}/{}/txs", address)
        };

        let transactions: Vec<Transaction> = self
            .inner_rest
            .as_ref()
            .ok_or("REST client is not available")?
            .get(&endpoint)
            .await?;

        Ok(transactions)
    }
}
