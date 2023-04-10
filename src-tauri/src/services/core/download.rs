use std::{cmp::min, fs::File, io::Write, path::PathBuf};

use anyhow::{anyhow, Context};
use futures_util::stream::StreamExt;

async fn download_file<'url, U, F>(
    url: &str,
    path: PathBuf,
    mut callback: Option<F>,
) -> anyhow::Result<()>
where
    F: FnMut(u64),
{
    // Reqwest setup
    let res = reqwest::Client::builder().build()?.get(url).send().await?;
    let total_size = res
        .content_length()
        .ok_or(anyhow!("Failed to get content length from '{url}'"))?;

    let mut file = File::create(path)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.context("Error while downloading file")?;
        file.write_all(&chunk)
            .context("Error while writing to file")?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;

        if let Some(callback) = callback.as_mut() {
            callback(downloaded)
        }
    }

    Ok(())
}
