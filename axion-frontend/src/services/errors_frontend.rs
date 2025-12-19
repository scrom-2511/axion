use thiserror::{self, Error};

#[derive(Error, Debug)]
pub enum AxionFrontendError {
    #[error("Cannot get block headers")]
    BlockHeadersNotFound
}