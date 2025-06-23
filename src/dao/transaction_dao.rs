use crate::{Transaction, TransactionValidationError};

pub type Result<T> = std::result::Result<T, TransactionValidationError>;

pub trait TransactionDao {
    fn save_transaction(&mut self, transaction: Transaction) -> Result<()>;

    fn get_transaction(&mut self, tx_id: u32) -> Option<&mut Transaction>;

    fn update_transaction(&mut self, transaction: Transaction) -> Result<()>;

    fn transaction_exists(&mut self, tx_id: u32) -> bool;
}
