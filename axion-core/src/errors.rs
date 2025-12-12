use thiserror::Error;

#[derive(Debug, Error)]
pub enum AxionError {
    #[error("Invalid Key")]
    InvalidKey,

    #[error("Unauthorized signer")]
    UnauthorizedSigner,

    #[error("Verification failed")]
    VerificationFailed
}