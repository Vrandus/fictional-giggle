use serde::Serialize;

use crate::TransactionValidationError;

pub type Result<T> = std::result::Result<T, TransactionValidationError>;

#[derive(Debug, Clone)]
pub struct Account {
    pub client_id: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

#[derive(Debug, Serialize)]
pub struct AccountOutput {
    #[serde(rename = "client")]
    pub client_id: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

impl Account {
    pub fn new(client_id: u16) -> Self {
        Self {
            client_id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }

    pub fn deposit(&mut self, amount: f64) -> Result<()> {
        if self.locked {
            return Err(TransactionValidationError::AccountLocked);
        }

        self.available += amount;
        self.total += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<()> {
        if self.locked {
            return Err(TransactionValidationError::AccountLocked);
        }

        if self.available < amount {
            return Err(TransactionValidationError::InsufficientFunds);
        }

        self.available -= amount;
        self.total -= amount;
        Ok(())
    }

    pub fn hold_funds(&mut self, amount: f64) -> Result<()> {
        if self.available < amount {
            return Err(TransactionValidationError::InsufficientFunds);
        }

        self.available -= amount;
        self.held += amount;
        Ok(())
    }

    pub fn release_held_funds(&mut self, amount: f64) -> Result<()> {
        if self.held < amount {
            return Err(TransactionValidationError::InsufficientFunds);
        }

        self.held -= amount;
        self.available += amount;
        Ok(())
    }

    pub fn chargeback(&mut self, amount: f64) -> Result<()> {
        if self.held < amount {
            return Err(TransactionValidationError::InsufficientFunds);
        }

        self.held -= amount;
        self.total -= amount;
        self.locked = true;
        Ok(())
    }

    pub fn to_output(&self) -> AccountOutput {
        AccountOutput {
            client_id: self.client_id,
            available: rounded(self.available),
            held: rounded(self.held),
            total: rounded(self.total),
            locked: self.locked,
        }
    }
}

fn rounded(val: f64) -> f64 {
    (val * 10000.0).round() / 10000.0
}
