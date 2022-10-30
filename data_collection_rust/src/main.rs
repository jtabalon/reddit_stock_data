use roux::Subreddit;
#[cfg(feature = "async")]
use tokio;

// #[cfg_attr(feature = "async", tokio::main)]
#[tokio::main]
#[maybe_async::maybe_async]

async fn main() {
    let subreddit_name: &str = "teslainvestorsclub";
    let subreddit = Subreddit::new(subreddit_name);

    let moderators = subreddit.moderators().await;

    let hot = subreddit.hot(25, None).await;

    let rising = subreddit.rising(25, None).await;

    println!("{:?}", hot)

}
