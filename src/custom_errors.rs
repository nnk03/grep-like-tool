use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum DFAError {
    #[error("Invalid transition: {0}")]
    InvalidTransition(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Invalid symbol: {0}")]
    InvalidSymbol(String),
}

#[derive(Debug, Error)]
pub enum NFAError {
    #[error("Already Existing Transition {0}")]
    ExistingTransition(String),
}

#[derive(Debug, Error)]
pub enum AutomatonError {
    #[error("DFA Error {0}")]
    DFAError(DFAError),

    #[error("NFA Error {0}")]
    NFAError(NFAError),
}
