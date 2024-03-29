pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed converting [u8] to String: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Failed serializing to json: {0}")]
    SerdeYaml(#[from] serde_yaml::Error),
    #[error("Error building regex: {0}")]
    Regex(#[from] regex::Error),

    // Application errors
    #[error("Config not found: tried config.(json|yaml|yml)")]
    ConfigNotFound,
    #[error("Stdin is none.")]
    NoneStdin,
    #[error("Empty result from dmenu.")]
    EmptyResult,
}
