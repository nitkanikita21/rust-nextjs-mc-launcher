use std::{
    cmp::min,
    fs::File,
    io::{Write},
    path::PathBuf,
};

use futures_util::stream::StreamExt;

async fn download_file<F>(
    url: String,
    path: PathBuf,
    mut callback: Option<F>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    F: FnMut(u64),
{
    // Reqwest setup
    let res = reqwest::Client::builder()
        .build()?
        .get(&url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    let mut file = File::create(path).or(Err("Failed to create file "))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file".to_string()))?;
        file.write_all(&chunk)
            .or(Err("Error while writing to file".to_string()))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;

        if callback.is_some() {
            callback.as_mut().unwrap()(downloaded)
        }
    }

    Ok(())
}
