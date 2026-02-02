pub mod websocket {
    use crate::ws::backoff::Backoff;
    use futures_util::{SinkExt, StreamExt, stream::SplitSink, stream::SplitStream};
    use std::{io, time::Duration};
    use tokio::sync::{mpsc, watch};
    use tokio::{net::TcpStream, task::JoinHandle, time::sleep};
    use tokio_tungstenite::{
        MaybeTlsStream, WebSocketStream, connect_async,
        tungstenite::{Error as WsError, Message, Utf8Bytes},
    };

    type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
    type WsSink = SplitSink<WsStream, Message>;
    type WsSource = SplitStream<WsStream>;

    pub struct WebsocketClient {
        outbound: mpsc::Sender<Message>,
        inbound: mpsc::Receiver<String>,
        shutdown: watch::Sender<bool>,
        send_handle: JoinHandle<()>,
        recv_handle: JoinHandle<()>,
    }

    impl WebsocketClient {
        pub async fn connect(address: &str) -> Result<Self, Box<dyn std::error::Error>> {
            let mut backoff =
                Backoff::new(Duration::from_millis(500), Duration::from_secs(10), 1.8);

            let mut last_err: Option<WsError> = None;

            for attempt in 1..=5 {
                match connect_async(address).await {
                    Ok((ws_stream, _)) => {
                        let (sink, stream) = ws_stream.split();

                        let (out_tx, out_rx) = mpsc::channel::<Message>(64);
                        let (in_tx, in_rx) = mpsc::channel::<String>(64);
                        let (shutdown_tx, shutdown_rx) = watch::channel(false);

                        let send_handle =
                            tokio::spawn(run_send_loop(sink, out_rx, shutdown_rx.clone()));
                        let recv_handle =
                            tokio::spawn(run_recv_loop(stream, in_tx, out_tx.clone(), shutdown_rx));

                        return Ok(Self {
                            outbound: out_tx,
                            inbound: in_rx,
                            shutdown: shutdown_tx,
                            send_handle,
                            recv_handle,
                        });
                    }
                    Err(err) => {
                        last_err = Some(err);
                        if attempt == 5 {
                            break;
                        }
                        let delay = backoff.next_delay();
                        eprintln!(
                            "connect attempt {attempt} failed, retrying in {:?}...",
                            delay
                        );
                        sleep(delay).await;
                    }
                }
            }

            Err(last_err
                .map(|e| Box::new(e) as Box<dyn std::error::Error>)
                .unwrap_or_else(|| {
                    Box::new(io::Error::new(io::ErrorKind::Other, "connection failed"))
                }))
        }

        pub async fn send<T: serde::Serialize>(
            &mut self,
            request: &T,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let json = serde_json::to_string(request)?;
            let message = Message::Text(Utf8Bytes::from(json));
            self.outbound
                .send(message)
                .await
                .map_err(|e| Box::new(io::Error::new(io::ErrorKind::BrokenPipe, e)) as _)
        }

        pub async fn receive(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>> {
            Ok(self.inbound.recv().await)
        }

        pub async fn close(self) -> Result<(), Box<dyn std::error::Error>> {
            let _ = self.outbound.send(Message::Close(None)).await;
            let _ = self.shutdown.send(true);
            let _ = self.send_handle.await;
            let _ = self.recv_handle.await;
            Ok(())
        }
    }

    async fn run_send_loop(
        mut sink: WsSink,
        mut outbound: mpsc::Receiver<Message>,
        mut shutdown: watch::Receiver<bool>,
    ) {
        loop {
            tokio::select! {
                maybe_msg = outbound.recv() => {
                    match maybe_msg {
                        Some(msg) => {
                            if let Err(err) = sink.send(msg).await {
                                eprintln!("ws send error: {err}");
                                break;
                            }
                        }
                        None => break, // sender dropped
                    }
                }
                _ = shutdown.changed() => {
                    if *shutdown.borrow() { break; }
                }
            }
        }
        let _ = sink.close().await;
    }

    async fn run_recv_loop(
        mut stream: WsSource,
        inbound: mpsc::Sender<String>,
        outbound: mpsc::Sender<Message>,
        mut shutdown: watch::Receiver<bool>,
    ) {
        loop {
            tokio::select! {
                _ = shutdown.changed() => {
                    if *shutdown.borrow() { break; }
                }
                incoming = stream.next() => match incoming {
                    Some(Ok(Message::Text(text))) => {
                        let _ = inbound.send(Utf8Bytes::to_string(&text)).await;
                    }
                    Some(Ok(Message::Ping(payload))) => {
                        let _ = outbound.try_send(Message::Pong(payload));
                    }
                    Some(Ok(Message::Pong(_))) => {}
                    Some(Ok(Message::Binary(data))) => {
                        eprintln!("Received binary data: {} bytes", data.len());
                    }
                    Some(Ok(Message::Close(frame))) => {
                        eprintln!("Connection closed by server: {:?}", frame);
                        break;
                    }
                    Some(Ok(_)) => {
                        eprintln!("Received unhandled message type");
                    }
                    Some(Err(e)) => {
                        eprintln!("ws receive error: {e}");
                        break;
                    }
                    None => break,
                }
            }
        }
    }
}
