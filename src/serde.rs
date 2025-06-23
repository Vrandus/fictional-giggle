use std::{fs::File, io, path::Path};

use csv::Reader;

use crate::{AccountOutput, RuntimeError, TransactionRecord};

pub fn load_transactions_from_csv(
    file_path: String,
) -> Result<Vec<TransactionRecord>, RuntimeError> {
    if !Path::new(&file_path).exists() {
        return Err(RuntimeError::CsvError(format!(
            "Path not found {}",
            file_path
        )));
    }

    let file = match File::open(&file_path) {
        Ok(f) => f,
        Err(_) => return Err(RuntimeError::CsvError(file_path.to_string())),
    };

    let mut csv_reader = Reader::from_reader(file);
    let mut transactions = Vec::new();

    for result in csv_reader.deserialize() {
        let record: TransactionRecord =
            result.map_err(|e| RuntimeError::CsvError(e.to_string()))?;
        transactions.push(record);
    }

    Ok(transactions)
}

pub fn accounts_to_csv(account_output: &Vec<AccountOutput>) -> Result<(), RuntimeError> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    for account in account_output {
        wtr.serialize(account)
            .map_err(|e| RuntimeError::CsvError(format!("{:?}", e)))?;
    }
    wtr.flush()
        .map_err(|e| RuntimeError::CsvError(format!("{:?}", e)))?;
    Ok(())
}
