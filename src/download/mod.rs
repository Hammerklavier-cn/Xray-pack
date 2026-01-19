use std::path::Path;

use futures::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio::runtime::Runtime;

use crate::errors::{PackError, PackResult};

pub mod geodat;
pub mod wintun;

/// Asynchronous version of download function
pub async fn download_file_async(url: impl AsRef<str>, dest: impl AsRef<Path>) -> PackResult<()> {
    let response = reqwest::get(url.as_ref()).await?;

    if response.status().is_success() {
        log::debug!("Successfully connected to {}", url.as_ref());
        let mut file = tokio::fs::File::create(dest.as_ref())
            .await
            .map_err(|_| PackError::CreateFailed(dest.as_ref().to_path_buf()))?;
        let mut stream = response.bytes_stream();

        // Todo: optimize the implementation so that download and write are done concurrently.
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
        }

        file.flush().await?;
        Ok(())
    } else {
        Err(crate::errors::PackError::NetworkError(
            response.error_for_status().unwrap_err(),
        ))
    }
}

/// Download a file from the given URL to the specified destination.
pub fn download_file(url: impl AsRef<str>, dest: impl AsRef<Path>) -> PackResult<()> {
    // Create a new runtime for executing async code
    let rt = Runtime::new()?;

    // Block on the async download function
    rt.block_on(download_file_async(url, dest))
}

/// Download a file and get its content as a String.
///
/// Async version of `download_file_content`.
#[allow(dead_code)]
pub async fn download_file_content_async(url: impl AsRef<str>) -> PackResult<String> {
    let response = reqwest::get(url.as_ref()).await?;

    if response.status().is_success() {
        log::debug!("Successfully connected to {}", url.as_ref());
        let content = response.text().await?;
        Ok(content)
    } else {
        Err(crate::errors::PackError::NetworkError(
            response.error_for_status().unwrap_err(),
        ))
    }
}

/// Download a file and get its content as a String.
#[allow(dead_code)]
pub fn download_file_content(url: impl AsRef<str>) -> PackResult<String> {
    // Create a new runtime for executing async code
    let rt = Runtime::new()?;

    // Block on the async download function
    rt.block_on(download_file_content_async(url))
}
