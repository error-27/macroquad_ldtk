use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    SerdeJson(serde_json::error::Category),
    LayerTypeNotFound { layer_type: String },
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(..) => std::fmt::Debug::fmt(&self, f),
            Self::SerdeJson(category) => write!(f, "Json Deserialization Error: {:?}", category), // May be better ways of doing this???
            Self::LayerTypeNotFound { layer_type } => write!(
                f, "Invalid layer type: {}. This should not happen unless the leveldata was modified outside LDtk.", layer_type
            )
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        value.downcast::<Self>().unwrap_or_else(Self::Io)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value.classify())
    }
}
