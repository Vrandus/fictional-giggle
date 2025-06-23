mod helpers {
    use fictional_giggle::{
        InMemoryAccountDao, InMemoryTransactionDao, TransactionRecord, TransactionType,
        engine::TransactionEngine,
    };

    pub fn create_engine() -> TransactionEngine<InMemoryAccountDao, InMemoryTransactionDao> {
        let account_dao = InMemoryAccountDao::new();
        let transaction_dao = InMemoryTransactionDao::new();
        TransactionEngine::new(account_dao, transaction_dao)
    }

    pub fn create_deposit_record(
        client_id: u16,
        transaction_id: u32,
        amount: f64,
    ) -> TransactionRecord {
        TransactionRecord {
            transaction_type: TransactionType::Deposit,
            client_id,
            transaction_id,
            amount: Some(amount),
        }
    }

    pub fn create_withdrawal_record(
        client_id: u16,
        transaction_id: u32,
        amount: f64,
    ) -> TransactionRecord {
        TransactionRecord {
            transaction_type: TransactionType::Withdrawal,
            client_id,
            transaction_id,
            amount: Some(amount),
        }
    }

    pub fn create_dispute_record(client_id: u16, transaction_id: u32) -> TransactionRecord {
        TransactionRecord {
            transaction_type: TransactionType::Dispute,
            client_id,
            transaction_id,
            amount: None,
        }
    }

    pub fn create_resolve_record(client_id: u16, transaction_id: u32) -> TransactionRecord {
        TransactionRecord {
            transaction_type: TransactionType::Resolve,
            client_id,
            transaction_id,
            amount: None,
        }
    }

    pub fn create_chargeback_record(client_id: u16, transaction_id: u32) -> TransactionRecord {
        TransactionRecord {
            transaction_type: TransactionType::Chargeback,
            client_id,
            transaction_id,
            amount: None,
        }
    }
}

#[test]
fn test_simple_deposit() {
    let mut engine = helpers::create_engine();
    let deposit = helpers::create_deposit_record(1, 1, 100.0);

    let result = engine.process_transaction(&deposit);
    assert!(result.is_ok());

    let outputs = engine.get_client_outputs();
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].client_id, 1);
    assert_eq!(outputs[0].available, 100.0);
    assert_eq!(outputs[0].held, 0.0);
    assert_eq!(outputs[0].total, 100.0);
    assert!(!outputs[0].locked);
}

#[test]
fn test_dispute_workflow() {
    let mut engine = helpers::create_engine();

    // Deposit money
    engine
        .process_transaction(&helpers::create_deposit_record(1, 1, 100.0))
        .unwrap();

    // Dispute the deposit
    let dispute = helpers::create_dispute_record(1, 1);
    let result = engine.process_transaction(&dispute);
    assert!(result.is_ok());

    let outputs = engine.get_client_outputs();
    assert_eq!(outputs[0].available, 0.0);
    assert_eq!(outputs[0].held, 100.0);
    assert_eq!(outputs[0].total, 100.0);
}

#[test]
fn test_dispute_resolved_workflow() {
    let mut engine = helpers::create_engine();

    // Deposit money
    engine
        .process_transaction(&helpers::create_deposit_record(1, 1, 100.0))
        .unwrap();

    // Dispute the deposit
    let dispute = helpers::create_dispute_record(1, 1);
    let result = engine.process_transaction(&dispute);
    assert!(result.is_ok());

    let mut outputs = engine.get_client_outputs();
    assert_eq!(outputs[0].available, 0.0);
    assert_eq!(outputs[0].held, 100.0);
    assert_eq!(outputs[0].total, 100.0);

    // Resolve previous Dispute
    let resolution = helpers::create_resolve_record(1, 1);

    let resolution_result = engine.process_transaction(&resolution);

    outputs = engine.get_client_outputs();
    assert!(resolution_result.is_ok());
    assert_eq!(outputs[0].available, 100.0);
    assert_eq!(outputs[0].held, 0.0);
    assert_eq!(outputs[0].total, 100.0);
}

#[test]
fn test_dispute_chargeback_workflow() {
    let mut engine = helpers::create_engine();

    // Deposit money
    engine
        .process_transaction(&helpers::create_deposit_record(1, 1, 100.0))
        .unwrap();

    // Dispute the deposit
    let dispute = helpers::create_dispute_record(1, 1);
    let result = engine.process_transaction(&dispute);
    assert!(result.is_ok());

    let mut outputs = engine.get_client_outputs();
    assert_eq!(outputs[0].available, 0.0);
    assert_eq!(outputs[0].held, 100.0);
    assert_eq!(outputs[0].total, 100.0);

    //
    let chargeback = helpers::create_chargeback_record(1, 1);

    let resolution_result = engine.process_transaction(&chargeback);

    outputs = engine.get_client_outputs();
    assert!(resolution_result.is_ok());
    assert_eq!(outputs[0].available, 0.0);
    assert_eq!(outputs[0].held, 0.0);
    assert_eq!(outputs[0].total, 0.0);
}

#[test]
fn test_multiple_client_ids() {
    let mut engine = helpers::create_engine();

    engine
        .process_transaction(&helpers::create_deposit_record(1, 1, 100.0))
        .unwrap();
    engine
        .process_transaction(&helpers::create_deposit_record(2, 2, 200.0))
        .unwrap();
    engine
        .process_transaction(&helpers::create_withdrawal_record(1, 3, 25.0))
        .unwrap();

    let outputs = engine.get_client_outputs();
    assert_eq!(outputs.len(), 2);

    let client_id1 = outputs.iter().find(|c| c.client_id == 1).unwrap();
    let client_id2 = outputs.iter().find(|c| c.client_id == 2).unwrap();

    assert_eq!(client_id1.available, 75.0);
    assert_eq!(client_id1.total, 75.0);
    assert_eq!(client_id2.available, 200.0);
    assert_eq!(client_id2.total, 200.0);
}
