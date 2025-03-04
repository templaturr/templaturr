use thiserror::Error;

#[derive(Debug, Error)]
pub enum TemplaturrError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Missing variables: {0:?}")]
    MissingVariables(Vec<String>),

    #[error("Invalid template path: {0}")]
    InvalidPath(String),
    
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("Walkdir error: {0}")]
    Walkdir(#[from] walkdir::Error),

    #[error("Strip prefix error: {0}")]
    StripPrefix(#[from] std::path::StripPrefixError),
}