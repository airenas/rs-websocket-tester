use std::time::{Duration, Instant};

use anyhow::Context;
use futures::{SinkExt, StreamExt};
use tokio::time;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_util::sync::CancellationToken;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use clap::Parser;
use url::Url;
use wslib::shutdown_signal;

#[derive(Parser, Debug)]
#[command(version = env!("CARGO_APP_VERSION"), name = "asr-worker", about, long_about = None)]
struct Args {
    /// Url
    #[arg(long, env)]
    url: String,
}

async fn main_int(args: Args) -> anyhow::Result<()> {
    log::info!("Starting websocket client");
    log::info!("Version      : {}", env!("CARGO_APP_VERSION"));
    log::info!("URL          : {}", args.url);

    let url = Url::parse(&args.url)?;
    let (ws_stream, _) = connect_async(url).await.context("Failed to connect")?;
    log::info!("WebSocket connection established");
    let (mut write, mut read) = ws_stream.split();

    let token = CancellationToken::new();

    let mut interval = time::interval(Duration::from_millis(200));
    let mut message_id = 0;

    let cl_token = token.clone();

    let write_task: tokio::task::JoinHandle<Result<(), anyhow::Error>> = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cl_token.cancelled() => {
                    log::debug!("canceled");
                    break;
                }
                _ = interval.tick() => {}
            }

            let message = format!("Message ID: {}", message_id);
            message_id += 1;

            write.send(Message::Text(message)).await?;
            log::info!("Sent: {}", message_id - 1);
        }
        Ok(())
    });
    let cl_token = token.clone();
    let read_task = tokio::spawn(async move {
        let mut last_message_time = Instant::now();

        loop {
            tokio::select! {
                msg = read.next() => {
                    if let Some(Ok(msg)) = msg {
                        let now = Instant::now();
                        let delay = now.duration_since(last_message_time);
                        last_message_time = now;
                        if delay < Duration::from_millis(250) {
                            log::info!("Received: {:.2?}. {}", delay, msg);
                    } else if delay < Duration::from_millis(450) {
                        log::warn!("Received: {:.2?}. {}", delay, msg);
                    } else{
                        log::error!("Received: {:.2?}. {}", delay, msg);
                    }
                    } else {
                        log::info!("Connection closed or error occurred");
                        cl_token.cancel();
                    }
                }
                _ = cl_token.cancelled() => {
                    log::info!("Cancellation signal received, stopping listener...");
                    break;
                }
            }
        }
    });

    tokio::select! {
        _ = shutdown_signal() => {
            token.cancel();
        }
        _ = token.cancelled() => {}
    }

    if let Err(e) = read_task.await {
        log::error!("Client read encountered an error: {:?}", e);
    }
    if let Err(e) = write_task.await {
        log::error!("Client write encountered an error: {:?}", e);
    }

    log::info!("Done");
    Ok(())
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
