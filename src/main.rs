use broadcaster::BroadcastChannel;
use futures::StreamExt;
use tide::{Request, sse};

#[derive(Clone, Debug)]
struct State {
    channel: BroadcastChannel<String>
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();

    let channel = BroadcastChannel::new();
    let mut app = tide::with_state(State { channel });
    app.at("/emojis").get(sse::endpoint(emojis));
    app.at("/send/:emoji").get(send);
    app.listen("localhost:8080").await?;
    Ok(())
}

async fn emojis(request: Request<State>, sender: sse::Sender) -> Result<(), tide::Error> {
    let mut count: i64 = 0;
    let mut channel = request.state().channel.clone();
    while let Some(emoji) = channel.next().await {
        let id = format!("{}", count);
        sender.send("emoji", emoji, Some(&id)).await?;
        count += 1;
    }
    Ok(())
}

async fn send(request: Request<State>) -> tide::Result {
    let emoji = request.param("emoji")?;
    request.state().channel.send(&emoji.to_string()).await?;
    Ok(emoji.into())
}