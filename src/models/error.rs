use std::fmt;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    CsvError(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::CsvError(e) => write!(f, "Csv serialization error: {}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TransactionValidationError {
    InsufficientFunds,
    AccountLocked,
    TransactionNotFound(u32),
    DuplicateTransaction(u32),
    InvalidTransactionType,
    InvalidAmount,
    ClientMismatch,
    InvalidTransactionState,
    CsvError(String),
    IoError(String),
    DataAccessError(String),
}

impl fmt::Display for TransactionValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionValidationError::InsufficientFunds => write!(f, "Insufficient funds"),
            TransactionValidationError::AccountLocked => write!(f, "Account is locked"),
            TransactionValidationError::TransactionNotFound(id) => {
                write!(f, "Transaction {} not found", id)
            }
            TransactionValidationError::DuplicateTransaction(id) => {
                write!(f, "Transaction {} already exists", id)
            }
            TransactionValidationError::InvalidTransactionType => {
                write!(f, "Invalid transaction type")
            }
            TransactionValidationError::InvalidAmount => write!(f, "Invalid amount"),
            TransactionValidationError::ClientMismatch => write!(f, "Client ID mismatch"),
            TransactionValidationError::InvalidTransactionState => {
                write!(f, "Invalid transaction state")
            }
            TransactionValidationError::CsvError(msg) => write!(f, "CSV error: {}", msg),
            TransactionValidationError::IoError(msg) => write!(f, "IO error: {}", msg),
            TransactionValidationError::DataAccessError(msg) => {
                write!(f, "Data access error: {}", msg)
            }
        }
    }
}
