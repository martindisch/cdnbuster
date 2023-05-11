// Images
// https://www.galaxus.ch/im/Files/5/4/5/9/8/0/8/9/Master_4_Picture_PNG_schwarz_Solar_L.png?impolicy=ProductTileImage&resizeWidth=648&resizeHeight=486&cropWidth=648&cropHeight=486&resizeType=downsize&quality=high
// https://media.s-bol.com/BpyMDBY9kjLJ/g00KJD/550x335.jpg

use reqwest::{Client, Version};
use std::{
    error::Error,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().http1_only().build()?;
    let mut durations = Vec::new();

    for _ in 0..100 {
        let now = Instant::now();
        client
            .get("https://media.s-bol.com/BpyMDBY9kjLJ/g00KJD/550x335.jpg")
            .version(Version::HTTP_11)
            .send()
            .await?;
        let elapsed = now.elapsed();
        println!("{elapsed:.2?}");
        durations.push(elapsed);
    }

    let average = durations.iter().sum::<Duration>() / durations.len() as u32;
    println!("Average: {average:.2?}");

    Ok(())
}
