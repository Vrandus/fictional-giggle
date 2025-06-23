use crate::Result;
use crate::{
    AccountDao, AccountOutput, Transaction, TransactionDao, TransactionRecord, TransactionType,
    TransactionValidationError,
};

pub struct TransactionEngine<A, T>
where
    A: AccountDao,
    T: TransactionDao,
{
    accounts: A,
    transactions: T,
}

impl<A, T> TransactionEngine<A, T>
where
    A: AccountDao,
    T: TransactionDao,
{
    pub fn new(account_dao: A, transaction_dao: T) -> Self {
        Self {
            accounts: account_dao,
            transactions: transaction_dao,
        }
    }

    pub fn process_transaction(&mut self, record: &TransactionRecord) -> Result<()> {
        match record.transaction_type {
            TransactionType::Deposit => self.process_deposit(record),
            TransactionType::Withdrawal => self.process_withdrawal(record),
            TransactionType::Dispute => self.process_dispute(record),
            TransactionType::Resolve => self.process_resolve(record),
            TransactionType::Chargeback => self.process_chargeback(record),
        }
    }

    pub fn get_client_outputs(&self) -> Vec<AccountOutput> {
        self.accounts.get_all_client_outputs()
    }

    fn process_deposit(&mut self, record: &TransactionRecord) -> Result<()> {
        let amount = record.amount;

        if self.transactions.transaction_exists(record.transaction_id) {
            return Err(TransactionValidationError::DuplicateTransaction(
                record.transaction_id,
            ));
        }

        let transaction = Transaction::new(
            record.transaction_id,
            record.client_id,
            record.transaction_type.clone(),
            amount,
        );

        let client = self.accounts.get_or_create_account(record.client_id)?;
        client.deposit(amount.unwrap())?;

        self.transactions.save_transaction(transaction)?;
        Ok(())
    }

    fn process_withdrawal(&mut self, record: &TransactionRecord) -> Result<()> {
        let amount = record.amount;

        if self.transactions.transaction_exists(record.transaction_id) {
            return Err(TransactionValidationError::DuplicateTransaction(
                record.transaction_id,
            ));
        }

        let client = self.accounts.get_or_create_account(record.client_id)?;
        client.withdraw(amount.unwrap())?;

        let transaction = Transaction::new(
            record.transaction_id,
            record.client_id,
            record.transaction_type.clone(),
            amount,
        );
        self.transactions.save_transaction(transaction)?;
        Ok(())
    }

    fn process_dispute(&mut self, record: &TransactionRecord) -> Result<()> {
        let original_tx = self
            .transactions
            .get_transaction(record.transaction_id)
            .ok_or(TransactionValidationError::TransactionNotFound(
                record.transaction_id,
            ))?;

        if !original_tx.can_be_disputed() {
            return Err(TransactionValidationError::InvalidTransactionState);
        }

        if original_tx.client_id != record.client_id {
            return Err(TransactionValidationError::ClientMismatch);
        }

        let amount = original_tx.amount;
        let client = self.accounts.get_or_create_account(record.client_id)?;
        client.hold_funds(amount.unwrap())?;

        original_tx.transaction_state = crate::models::TransactionState::Disputed;
        Ok(())
    }

    fn process_resolve(&mut self, record: &TransactionRecord) -> Result<()> {
        let original_tx = self
            .transactions
            .get_transaction(record.transaction_id)
            .ok_or(TransactionValidationError::TransactionNotFound(
                record.transaction_id,
            ))?;

        if !original_tx.can_be_resolved() {
            return Err(TransactionValidationError::InvalidTransactionState);
        }

        if original_tx.client_id != record.client_id {
            return Err(TransactionValidationError::ClientMismatch);
        }

        let amount = original_tx.amount;
        let client = self.accounts.get_or_create_account(record.client_id)?;
        client.release_held_funds(amount.unwrap())?;

        original_tx.transaction_state = crate::models::TransactionState::Resolved;
        Ok(())
    }

    fn process_chargeback(&mut self, record: &TransactionRecord) -> Result<()> {
        let original_tx = self
            .transactions
            .get_transaction(record.transaction_id)
            .ok_or(TransactionValidationError::TransactionNotFound(
                record.transaction_id,
            ))?;

        if !original_tx.can_be_charged_back() {
            return Err(TransactionValidationError::InvalidTransactionState);
        }

        if original_tx.client_id != record.client_id {
            return Err(TransactionValidationError::ClientMismatch);
        }

        let amount = original_tx.amount;
        let client = self.accounts.get_or_create_account(record.client_id)?;
        client.chargeback(amount.unwrap())?;

        original_tx.transaction_state = crate::models::TransactionState::ChargedBack;
        Ok(())
    }
}
