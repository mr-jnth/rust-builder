use tweet::TweetBuilder;
use crate::custom::*;

mod tweet;
mod custom;

fn main() -> Result<()> {
    let mut tweet_builder = TweetBuilder::new();
    let tweet_base = tweet_builder
    .user("Jayanth")
    .subject("Not printed")
    .body("Them Bones has the sickest riff ever");

    let tweet_base2 = tweet_base
    .subject("AIC");

    let tweet = tweet_base.build()?;

    println!("{:#?}", tweet);

    Ok(())
}
