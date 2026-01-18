use std::path::PathBuf;

pub type PackResult<T> = Result<T, PackError>;

#[derive(Debug, thiserror::Error)]
pub enum PackError {
    #[error("Async error: {0}")]
    AsyncError(#[from] tokio::io::Error),

    #[error("Failed to build Xray-core: {0}")]
    BuildFailed(String),

    #[error("Failed to copy from {0} to {1}")]
    CopyFailed(PathBuf, PathBuf),

    #[error("Failed to create file or directory at {0}")]
    CreateFailed(PathBuf),

    #[error("Failed to delete file or directory at {0}")]
    DeleteFailed(PathBuf),

    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("{0} required but not found.")]
    MissingDependency(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Failed to read file at {0}")]
    ReadFailed(PathBuf),

    #[error("Unimplemented")]
    #[allow(dead_code)]
    Unimplemented,

    #[error("Zip error: {0}")]
    ZipError(#[from] zip::result::ZipError),
}
