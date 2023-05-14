use clap::Parser;
use reqwest::{Client, Version};
use std::{
    error::Error,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse();
    let (client, http_version) = build_client(args.http)?;
    let mut durations = Vec::new();

    for _ in 0..101 {
        let now = Instant::now();
        client
            .get("https://media.s-bol.com/BpyMDBY9kjLJ/g00KJD/550x335.jpg")
            .version(http_version)
            .send()
            .await?;
        let elapsed = now.elapsed();
        println!("{elapsed:.2?}");
        durations.push(elapsed);
    }

    let average = durations[1..].iter().sum::<Duration>() / durations.len() as u32;
    println!("Average (excluding first request): {average:.2?}");

    Ok(())
}

fn build_client(http_version: u32) -> Result<(Client, Version), reqwest::Error> {
    let builder = Client::builder().use_rustls_tls();
    Ok(match http_version {
        3 => (builder.http3_prior_knowledge().build()?, Version::HTTP_3),
        2 => (builder.http2_prior_knowledge().build()?, Version::HTTP_2),
        _ => (builder.http1_only().build()?, Version::HTTP_11),
    })
}

#[derive(Parser)]
#[command(author, version, about)]
struct Arguments {
    #[arg(long)]
    http: u32,
}
