use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut rx = bcast_tx.subscribe();
    let (mut write, mut read) = ws_stream.split();

    loop {
        tokio::select! {
            // Receive from client and broadcast
            msg = read.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            // Print message from client to server console
                            println!("From client ({}): {}", addr, text);
                            // Optionally include addr in the message
                            let _ = bcast_tx.send(format!("{}: {}", addr, text));
                        } else if msg.is_close() {
                            break;
                        }
                    },
                    Some(Err(e)) => {
                        eprintln!("Error receiving from client {}: {}", addr, e);
                        break;
                    },
                    None => break,
                }
            },
            // Receive from broadcast and send to client
            result = rx.recv() => {
                match result {
                    Ok(msg) => {
                        // Optionally, skip sending to sender (optional part)
                        // if !msg.starts_with(&addr.to_string()) {
                        let _ = write.send(Message::text(msg)).await;
                        // }
                    },
                    Err(_) => break,
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}