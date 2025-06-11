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

#[derive(Debug, Error)]
pub enum NFAError {
    #[error("Already Existing Transition {0}")]
    ExistingTransition(&'static str),
}

#[derive(Debug, Error)]
pub enum AutomatonError {
    #[error("DFA Error {0}")]
    DFAError(DFAError),

    #[error("NFA Error {0}")]
    NFAError(NFAError),
}
