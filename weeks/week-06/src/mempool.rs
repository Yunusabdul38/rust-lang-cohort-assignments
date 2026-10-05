use std::collections::BTreeMap;

use crate::{
    MinerError::{self, DuplicateMempoolTransaction, TransactionNotFound},
    Transaction,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Mempool {
    pub transactions: BTreeMap<String, Transaction>,
}

impl Mempool {
    /// Create an empty mempool.
    pub fn new() -> Self {
        // Steps:
        // 1. Start with an empty `BTreeMap`.
        // 2. Return the mempool.
        let storage = BTreeMap::new();
        Mempool {
            transactions: storage,
        }
    }

    /// Insert a transaction by txid.
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), MinerError> {
        // Steps:
        // 1. Reject duplicate txids with `DuplicateMempoolTransaction(txid)`.
        // 2. Insert the transaction under its txid.
        if self.transactions.contains_key(&transaction.txid) {
            return Err(DuplicateMempoolTransaction(transaction.txid));
        }
        self.transactions
            .insert(transaction.txid.clone(), transaction);
        Ok(())
    }

    /// Remove and return one transaction.
    pub fn remove_transaction(&mut self, txid: &str) -> Result<Transaction, MinerError> {
        // Steps:
        // 1. Remove the transaction with the matching txid.
        // 2. Return it when present.
        // 3. Return `TransactionNotFound(txid)` when missing.
        self.transactions
            .remove(txid)
            .ok_or_else(|| TransactionNotFound(txid.to_string()))
    }

    /// Return transactions in deterministic txid order without removing them.
    pub fn ordered_transactions(&self) -> Vec<Transaction> {
        // Steps:
        // 1. Iterate over the `BTreeMap` values.
        // 2. Clone each transaction into a vector.
        // 3. Return the vector.
        let mut new_storage = Vec::new();
        for (_, x) in &self.transactions {
            new_storage.push(x.clone());
        }
        new_storage
    }

    /// Drain up to `limit` transactions in deterministic txid order.
    pub fn drain_for_candidate(&mut self, limit: usize) -> Vec<Transaction> {
        // Steps:
        // 1. Collect up to `limit` txids in sorted order.
        // 2. Remove those transactions from the map.
        // 3. Return removed transactions in the same order.
        // 4. If `limit` is 0, return an empty vector.
        if limit == 0 {
            return Vec::new();
        }

        let txids: Vec<String> = self.transactions.keys().take(limit).cloned().collect();
        txids
            .iter()
            .filter_map(|txid| self.transactions.remove(txid))
            .collect()
    }

    /// Return total output value across all transactions currently in the mempool.
    pub fn total_output_value(&self) -> u64 {
        // Steps:
        // 1. Iterate over all transactions.
        // 2. Add `transaction.total_output_value()`.
        // 3. Return the total.
        let mut total_output_value = 0;
        for (_, x) in &self.transactions {
            total_output_value += x.total_output_value();
        }
        total_output_value
    }
}
