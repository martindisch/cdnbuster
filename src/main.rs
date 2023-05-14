use clap::Parser;
use eyre::{eyre, Result};
use reqwest::{Client, Version};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();
    let (client, http_version) = build_client(args.http)?;
    let mut durations = Vec::new();

    for _ in 0..101 {
        let now = Instant::now();
        let response = client.get(&args.url).version(http_version).send().await?;
        let elapsed = now.elapsed();

        if !response.status().is_success() {
            Err(eyre!(
                "Server responded with error code {}",
                response.status()
            ))?
        }

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
    /// The HTTP version to use.
    #[arg(long)]
    http: u32,

    /// The resource to request.
    url: String,
}
