use reqwest::{Client, Version};
use std::{
    error::Error,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::builder()
        .use_rustls_tls()
        .http3_prior_knowledge()
        .build()?;
    let mut durations = Vec::new();

    for _ in 0..101 {
        let now = Instant::now();
        client
            .get("https://media.s-bol.com/BpyMDBY9kjLJ/g00KJD/550x335.jpg")
            .version(Version::HTTP_3)
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
