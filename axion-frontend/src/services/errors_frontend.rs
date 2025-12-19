use thiserror::{self, Error};

#[derive(Error, Debug)]
pub enum AxionFrontendError {
    #[error("Cannot get home path of your system. Make sure you are using a correct OS.")]
    HomeDirNotFound,

    #[error("Cannot get block headers")]
    BlockHeadersNotFound,

    #[error("Json Error:")]
    JsonError(#[from] serde_json::Error),

    #[error("Unable to write the latest block details to your device. Try again")]
    UnableToWriteLatestBlockDetails
}