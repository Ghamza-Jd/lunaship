#[derive(Debug, thiserror::Error)]
pub enum LunashipError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Handlebars Error: {0}")]
    HandlebarsError(#[from] handlebars::RenderError),
    #[error("Failed to read file name for a path")]
    FileNameError,
}
