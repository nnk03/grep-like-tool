use thiserror::Error;

#[derive(Debug, Error)]
pub enum DFAError {
    #[error("Invalid transition: {0}")]
    InvalidTransition(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Invalid symbol: {0}")]
    InvalidSymbol(String),
}
