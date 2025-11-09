use reqwest;
use tokio;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://staging-api.modrinth.com/")
        .await?
        .text()
        .await?;
    println!("body = {body:?}");
    Ok(())
}

mod cli;
