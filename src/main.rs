use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

const HELP_MSG: &str = include_str!("help.txt");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = TcpListener::bind("127.0.0.1:9876").await?;
    loop {
        let (mut tcp, _) = server.accept().await?;
        let (reader, writer) = tcp.split();
        let mut stream = FramedRead::new(reader, LinesCodec::new());
        let mut sink = FramedWrite::new(writer, LinesCodec::new());

        sink.send(HELP_MSG).await?;

        while let Some(Ok(mut msg)) = stream.next().await {
            // handle new /help command
            if msg.starts_with("/help") {
                sink.send(HELP_MSG).await?;
            // handle new /quit command
            } else if msg.starts_with("/quit") {
                break;
            // handle regular message
            } else {
                msg.push_str(" ❤️");
                sink.send(msg).await?;
            }
        }
    }
}
