use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrontmatterError {
    #[error("Failed to read file '{path}': {source}")]
    IoError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("No frontmatter found in '{0}'")]
    NoFrontmatter(PathBuf),

    #[error("Invalid frontmatter in '{path}': {message}")]
    InvalidFrontmatter { path: PathBuf, message: String },

    #[error("YAML parsing error in '{path}': {source}")]
    YamlError {
        path: PathBuf,
        source: serde_yaml::Error,
    },

    #[error("Directory error: {0}")]
    WalkDirError(#[from] walkdir::Error),
}

pub type Result<T> = std::result::Result<T, FrontmatterError>;
