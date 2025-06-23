use std::{env, process::exit};

use env_logger::{Builder, Target};
use log::{error, info};

use fictional_giggle::{
    InMemoryAccountDao, InMemoryTransactionDao, accounts_to_csv, engine::TransactionEngine,
    serde::load_transactions_from_csv,
};

fn main() {
    Builder::from_default_env()
        .filter_level(log::LevelFilter::Off) // Change when debugging
        .target(Target::Stdout)
        .init();

    let args: Vec<String> = env::args().collect();

    info!("Initialize Account Engine");
    if args.len() < 2 {
        error!("Missing transactions");
        exit(1);
    }

    let filename = args[1].to_owned();

    let transaction_records = match load_transactions_from_csv(filename) {
        Ok(transactions) => transactions,
        Err(e) => {
            error!("error in load_transactions_from_csv {}", e);
            exit(1)
        }
    };

    // Initialize in memory dao implementations of interfaces
    let in_memory_accounts = InMemoryAccountDao::new();
    let in_memory_transactions = InMemoryTransactionDao::new();
    let mut transaction_engine = TransactionEngine::new(in_memory_accounts, in_memory_transactions);

    for transaction in &transaction_records {
        if let Err(e) = transaction_engine.process_transaction(&transaction) {
            error!("Error {} processing transaction: {:?}", e, transaction);
        }
    }

    info!("{:?}", transaction_engine.get_client_outputs());

    let _ = accounts_to_csv(&transaction_engine.get_client_outputs());
}
