use warp::Filter;
use warp::ws::{Ws, WebSocket, Message};
use futures_util::{StreamExt, SinkExt};
use std::error::Error;
use std::time::{UNIX_EPOCH, SystemTime};

#[tokio::main]
async fn main() {
    println!("Starting server...");

    let routes = warp::path("api").and(warp::ws()).map(|ws: Ws| {
        ws.on_upgrade(answer)
    });

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn answer(socket: WebSocket) {
    let (mut tx, mut rx) = socket.split();
    let inner = || async move {
        while let Some(message) = rx.next().await {
            let message = message?;
            let message = message.to_str().map_err(|()| "Message is not a string")?;
            if message != "time" {
                continue;
            }
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
            let res = Message::text(now.as_millis().to_string());
            tx.send(res).await?;
        }

        Ok(()) as Result<(), Box<dyn Error + Send + Sync + 'static>>
    };

    if let Err(err) = inner().await {
        eprintln!("Error: {:?}", err);
    }
}
