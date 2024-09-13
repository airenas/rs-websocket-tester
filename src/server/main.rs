use std::time::{Duration, Instant};

use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tokio_util::sync::CancellationToken;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use clap::Parser;
use wslib::shutdown_signal;

#[derive(Parser, Debug)]
#[command(version = env!("CARGO_APP_VERSION"), name = "asr-worker", about, long_about = None)]
struct Args {
    /// Port
    #[arg(long, env, default_value = "8000")]
    port: String,
}

async fn main_int(args: Args) -> anyhow::Result<()> {
    log::info!("Starting websocket server");
    log::info!("Version      : {}", env!("CARGO_APP_VERSION"));
    log::info!("Port         : {}", args.port);
    if let Some(ip) = public_ip::addr().await {
        log::info!("Public IP    : {}", ip);
    } else {
        log::warn!("Public IP    : unknown");
    }

    let token = CancellationToken::new();

    let addr = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(&addr).await?;
    log::info!("Listening on : {}", addr);

    let cl_token = token.clone();
    let server_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                Ok((stream, _)) = listener.accept() => {
                    if let Ok(peer_addr) = stream.peer_addr() {
                        log::info!("Incoming connection from: {}", peer_addr);
                    }
                    let token = cl_token.clone();
                    tokio::spawn(handle_connection(stream, token));
                }
                _ = cl_token.cancelled() => {
                    log::info!("Cancellation signal received, stopping listener...");
                    break;
                }
            }
        }
    });

    shutdown_signal().await;
    token.cancel();
    if let Err(e) = server_task.await {
        log::error!("Server task encountered an error: {:?}", e);
    }

    log::info!("Done");
    Ok(())
}

// Handle a single WebSocket connection
async fn handle_connection(
    stream: tokio::net::TcpStream,
    token: CancellationToken,
) -> anyhow::Result<()> {
    let ws_stream = accept_async(stream).await?;

    let (mut write, mut read) = ws_stream.split();
    let mut last_message_time = Instant::now();

    loop {
        tokio::select! {
            _ = token.cancelled() => {
                log::debug!("canceled");
                return Ok(());
            }
            msg = read.next() => {
                if let Some(Ok(Message::Text(text))) = msg {
                    let now = Instant::now();
                    let delay = now.duration_since(last_message_time);
                    last_message_time = now;
                    if delay < Duration::from_millis(250) {
                        log::info!("Received: {:.2?}. {}", delay, text);
                    } else if delay < Duration::from_millis(450) {
                        log::warn!("Received: {:.2?}. {}", delay, text);
                    } else{
                        log::error!("Received: {:.2?}. {}", delay, text);
                    }

                    // Send the same message back to the client
                    let res = write
                        .send(Message::Text("Response to ".to_owned() + &text))
                        .await;
                    if let Err(e) = res {
                        log::error!("Error sending response: {}", e);
                    }
                } else if msg.is_none() {
                    log::info!("Client disconnected");
                    return Ok(());
                }
            }
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::Layer::default().compact())
        .init();
    let args = Args::parse();
    if let Err(e) = main_int(args).await {
        log::error!("{}", e);
        return Err(e);
    }
    Ok(())
}
