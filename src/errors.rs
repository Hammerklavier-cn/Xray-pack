pub type PackResult<T> = Result<T, PackError>;

#[derive(Debug, thiserror::Error)]
pub enum PackError {
    #[error("Failed to build Xray-core: {0}")]
    BuildFailed(String),
    #[error("{0} required but not found.")]
    MissingDependency(String),
    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Async error: {0}")]
    AsyncError(#[from] tokio::io::Error),
    #[error("Zip error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    #[error("Unimplemented")]
    #[allow(dead_code)]
    Unimplemented,
}
