use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    let (mut write, mut read) = ws_stream.split();

    loop {
        tokio::select! {
            // Read from stdin and send to server
            line = stdin.next_line() => {
                match line? {
                    Some(msg) => {
                        if !msg.is_empty() {
                            write.send(Message::text(msg)).await?;
                        }
                    },
                    None => break,
                }
            },
            // Receive from server and print
            msg = read.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if msg.is_close() {
                            println!("Connection closed by server.");
                            break;
                        } else if let Some(text) = msg.as_text() {
                            println!("{}", text);
                        }
                    },
                    Some(Err(e)) => {
                        eprintln!("Error receiving message: {}", e);
                        break;
                    },
                    None => {
                        println!("Connection closed by server.");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}