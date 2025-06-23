use crate::{Account, AccountOutput, TransactionValidationError};

pub type Result<T> = std::result::Result<T, TransactionValidationError>;

pub trait AccountDao {
    fn get_or_create_account(&mut self, client_id: u16) -> Result<&mut Account>;

    fn update_account(&mut self, account: Account) -> Result<()>;

    fn get_all_client_outputs(&self) -> Vec<AccountOutput>;
}
