mod api;
mod game;

#[tokio::main]
async fn main() {
    let words = api::fetch_words(50).await.expect("Failed to fetch words");
    game::start_typing_session(words).await;
}
