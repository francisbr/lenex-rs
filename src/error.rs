use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    FileExtension(FileExtensionError),

    #[error("{0}")]
    Custom(String),
}

#[derive(Error, Debug)]
pub enum FileExtensionError {
    #[error("Unknown file extension.")]
    UnknownExtension,

    #[error("Unsupported file extension.")]
    UnsupportedExtension(String),
}

impl From<FileExtensionError> for Error {
    fn from(value: FileExtensionError) -> Self {
        Self::FileExtension(value)
    }
}
