use thiserror::Error;

#[derive(Debug, Error)]
pub enum DFAError {
    #[error("Invalid transition: {0}")]
    InvalidTransition(&'static str),

    #[error("Invalid state: {0}")]
    InvalidState(&'static str),

    #[error("Invalid symbol: {0}")]
    InvalidSymbol(&'static str),
}
