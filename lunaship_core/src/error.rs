#[derive(Debug, thiserror::Error)]
pub enum LunashipError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Handlebars Error: {0}")]
    HandlebarsError(#[from] handlebars::RenderError),
    #[error("Toml Deserialize Error: {0}")]
    TomlDeserializeError(#[from] toml::de::Error),
    #[error("Toml Serialize Error: {0}")]
    TomlSerializeError(#[from] toml::ser::Error),
    #[error("Failed to read file name for a path")]
    FileNameError,
}
