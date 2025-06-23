use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransactionRecord {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    #[serde(rename = "client")]
    pub client_id: u16,
    #[serde(rename = "tx")]
    pub transaction_id: u32,
    pub amount: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: u32,
    pub client_id: u16,
    pub transaction_type: TransactionType,
    pub amount: Option<f64>,
    pub transaction_state: TransactionState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionState {
    Completed,
    Disputed,
    Resolved,
    ChargedBack,
}

impl Transaction {
    pub fn new(
        id: u32,
        client_id: u16,
        transaction_type: TransactionType,
        amount: Option<f64>,
    ) -> Self {
        Self {
            id,
            client_id,
            transaction_type,
            amount: amount,
            transaction_state: TransactionState::Completed,
        }
    }

    pub fn can_be_disputed(&self) -> bool {
        self.transaction_type == TransactionType::Deposit
            && self.transaction_state == TransactionState::Completed
    }

    pub fn can_be_resolved(&self) -> bool {
        self.transaction_state == TransactionState::Disputed
    }

    pub fn can_be_charged_back(&self) -> bool {
        self.transaction_state == TransactionState::Disputed
    }
}
