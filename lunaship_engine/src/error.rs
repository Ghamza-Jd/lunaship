#[derive(Debug, thiserror::Error)]
pub enum LunashipError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Toml Deserialize Error: {0}")]
    TomlDeserializeError(#[from] toml::de::Error),
    #[error("Toml Serialize Error: {0}")]
    TomlSerializeError(#[from] toml::ser::Error),
    #[error("Template Render Error: {0}")]
    TemplateRenderError(#[from] askama::Error),
}
