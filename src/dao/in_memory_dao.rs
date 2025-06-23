use std::collections::HashMap;

use crate::{
    Account, AccountOutput, Transaction, TransactionValidationError, account_dao::AccountDao,
    transaction_dao::TransactionDao,
};

pub type Result<T> = std::result::Result<T, TransactionValidationError>;

#[derive(Debug, Default)]
pub struct InMemoryAccountDao {
    accounts: HashMap<u16, Account>,
}

impl InMemoryAccountDao {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }
}

impl AccountDao for InMemoryAccountDao {
    fn get_or_create_account(&mut self, client_id: u16) -> Result<&mut Account> {
        Ok(self
            .accounts
            .entry(client_id)
            .or_insert_with(|| Account::new(client_id)))
    }

    fn update_account(&mut self, account: Account) -> Result<()> {
        self.accounts.insert(account.client_id, account);
        Ok(())
    }

    fn get_all_client_outputs(&self) -> Vec<AccountOutput> {
        self.accounts
            .values()
            .map(|client| client.to_output())
            .collect()
    }
}

#[derive(Debug, Default)]
pub struct InMemoryTransactionDao {
    transactions: HashMap<u32, Transaction>,
}

impl InMemoryTransactionDao {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
        }
    }
}

impl TransactionDao for InMemoryTransactionDao {
    fn save_transaction(&mut self, transaction: Transaction) -> Result<()> {
        if self.transactions.contains_key(&transaction.id) {
            return Err(TransactionValidationError::DuplicateTransaction(
                transaction.id,
            ));
        }
        self.transactions.insert(transaction.id, transaction);
        Ok(())
    }

    fn get_transaction(&mut self, tx_id: u32) -> Option<&mut Transaction> {
        self.transactions.get_mut(&tx_id)
    }

    fn update_transaction(&mut self, transaction: Transaction) -> Result<()> {
        let tx_id = transaction.id;
        if self.transactions.contains_key(&tx_id) {
            self.transactions.insert(tx_id, transaction);
            Ok(())
        } else {
            Err(TransactionValidationError::TransactionNotFound(tx_id))
        }
    }

    fn transaction_exists(&mut self, tx_id: u32) -> bool {
        self.transactions.contains_key(&tx_id)
    }
}
