#![allow(unused_variables)]

use std::{fmt::format, vec};

use hex::decode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub const COINBASE_PREVIOUS_OUTPUT: &str = "-";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TxStatus {
    Spent,
    Unspent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxInput {
    pub previous_txid: String,
    pub previous_vout: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxOutput {
    pub value_sats: u64,
    pub unique_id: Uuid,
    pub recipient: String,
    pub status: TxStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub txid: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub block_hash: String,
    pub previous_block_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub nonce: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub height: u64,
    pub network: Network,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AmountSummary {
    pub output_count: usize,
    pub total_sats: u64,
    pub spent_sats: u64,
    pub unspent_sats: u64,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum BtcLibError {
    #[error("malformed transaction data")]
    MalformedData,
    #[error("missing transaction")]
    MissingTransaction,
    #[error("empty transaction id")]
    EmptyTxId,
    #[error("missing transaction inputs")]
    MissingInputs,
    #[error("missing transaction outputs")]
    MissingOutputs,
    #[error("zero value output")]
    ZeroValueOutput,
    #[error("empty block")]
    EmptyBlock,
    #[error("duplicate transaction id")]
    DuplicateTxId,
    #[error("invalid hash hex")]
    InvalidHash,
    #[error("io error: {0}")]
    Io(String),
}

pub trait Hashable {
    /// Return stable string material that should be fed into `hash_hex`.
    fn hash_material(&self) -> String;

    /// Compute a SHA-256 hex digest for this value.
    ///
    /// This default implementation is complete; students do not need to edit it.
    fn hash_hex(&self) -> String {
        sha256::digest(self.hash_material())
    }
}

pub trait Validate {
    /// Validate a value and return a specific `BtcLibError` for bad data.
    fn validate(&self) -> Result<(), BtcLibError>;
}

impl From<std::io::Error> for BtcLibError {
    /// Convert an IO error into `BtcLibError::Io` while preserving its message.
    fn from(error: std::io::Error) -> Self {
        // Steps:
        // 1. Convert the IO error to a string.
        // 2. Store that string inside `BtcLibError::Io`.
        let m = BtcLibError::Io(error.to_string());
        m
    }
}

impl TxInput {
    /// Build a transaction input by copying the previous txid and storing vout.
    pub fn new(previous_txid: &str, previous_vout: u32) -> Self {
        // Steps:
        // 1. Convert `previous_txid` into an owned `String`.
        // 2. Store `previous_vout` unchanged.
        // 3. Return a `TxInput`.
        Self {
            previous_txid: previous_txid.to_string(),
            previous_vout,
        }
    }
}

impl TxOutput {
    /// Build a transaction output with a fresh UUID.
    pub fn new(value_sats: u64, recipient: &str, status: TxStatus) -> Self {
        // Steps:
        // 1. Store `value_sats` unchanged.
        // 2. Generate `unique_id` using `Uuid::new_v4()`.
        // 3. Convert `recipient` into an owned `String`.
        // 4. Store `status` unchanged.
        // 5. Return a `TxOutput`.
        Self {
            value_sats: value_sats,
            unique_id: Uuid::new_v4(),
            recipient: recipient.to_string(),
            status: status,
        }
    }

    /// Return true when this output status is `TxStatus::Unspent`.
    pub fn is_unspent(&self) -> bool {
        // Steps:
        // 1. Compare `self.status` with `TxStatus::Unspent`.
        // 2. Return the boolean result.
        if self.status == TxStatus::Unspent {
            true
        } else {
            false
        }
    }
}

impl Validate for TxOutput {
    /// Reject zero-value outputs.
    fn validate(&self) -> Result<(), BtcLibError> {
        // Steps:
        // 1. If `value_sats` is 0, return `Err(BtcLibError::ZeroValueOutput)`.
        // 2. Otherwise return `Ok(())`.
        if self.value_sats == 0 {
            Err(BtcLibError::ZeroValueOutput)
        } else {
            Ok(())
        }
    }
}

impl Transaction {
    /// Build a transaction by copying the txid and storing inputs/outputs.
    pub fn new(txid: &str, inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        // Steps:
        // 1. Convert `txid` into an owned `String`.
        // 2. Move `inputs` and `outputs` into the transaction.
        // 3. Return a `Transaction`.
        Self {
            txid: txid.to_string(),
            inputs: inputs,
            outputs,
        }
    }

    /// Return true for the simplified coinbase rule used in this assignment.
    ///
    /// A coinbase transaction has txid `"coinbase"` and no inputs.
    pub fn is_coinbase(&self) -> bool {
        // Steps:
        // 1. Check that `self.txid == "coinbase"`.
        // 2. Check that `self.inputs` is empty.
        // 3. Return true only when both checks pass.
        if self.txid == "coinbase" && self.inputs.is_empty() {
            true
        } else {
            false
        }
    }

    /// Sum the satoshi value of every output in this transaction.
    pub fn total_output_value(&self) -> u64 {
        // Steps:
        // 1. Start a total at 0.
        // 2. Add every output's `value_sats`.
        // 3. Return the total.
        let mut total = 0;
        for x in &self.outputs {
            total += x.value_sats;
        }
        total
    }

    /// Count outputs whose status is `TxStatus::Unspent`.
    pub fn unspent_output_count(&self) -> usize {
        // Steps:
        // 1. Walk through `self.outputs`.
        // 2. Count outputs whose status is `TxStatus::Unspent`.
        // 3. Return the count.
        let mut total = 0;
        for x in &self.outputs {
            if x.status == TxStatus::Unspent {
                total += 1;
            }
        }
        total as usize
    }

    /// Count outputs whose status is `TxStatus::Spent`.
    pub fn spent_output_count(&self) -> usize {
        // Steps:
        // 1. Walk through `self.outputs`.
        // 2. Count outputs whose status is `TxStatus::Spent`.
        // 3. Return the count.
        let mut total = 0;
        for x in &self.outputs {
            if x.status == TxStatus::Spent {
                total = x.value_sats;
            }
        }
        total as usize
    }
}

impl Hashable for Transaction {
    /// Return deterministic transaction hash material.
    ///
    /// Use exactly this format:
    /// `tx:<txid>|inputs:<prev_txid>:<vout>;...|outputs:<value>:<recipient>:<status>;...`
    ///
    /// Status text must be lowercase: `spent` or `unspent`.
    fn hash_material(&self) -> String {
        // Steps:
        // 1. Start with `tx:<txid>|inputs:`.
        // 2. Append each input as `<previous_txid>:<previous_vout>;`.
        // 3. Append `|outputs:`.
        // 4. Append each output as `<value_sats>:<recipient>:<status>;`.
        // 5. Return the final string.
        let mut hs = format!("tx:{}|inputs:", self.txid);
        for x in &self.inputs {
            let fmt = format!("{}:{};", x.previous_txid, x.previous_vout);
            hs.push_str(fmt.as_str());
        }
        hs.push_str("|outputs:");
        for x in &self.outputs {
            let mut status = "";
            if x.status == TxStatus::Spent {
                status = "spent"
            } else {
                status = "unspent"
            }
            let fmt = format!("{}:{}:{};", x.value_sats, x.recipient, status);
            hs.push_str(fmt.as_str());
        }
        hs
    }
}

impl Validate for Transaction {
    /// Validate a transaction using the Week 3 model plus Week 4 errors.
    fn validate(&self) -> Result<(), BtcLibError> {
        // Steps:
        // 1. If `txid` is empty, return `Err(BtcLibError::EmptyTxId)`.
        // 2. If the transaction is not coinbase and has no inputs, return
        //    `Err(BtcLibError::MissingInputs)`.
        // 3. If there are no outputs, return `Err(BtcLibError::MissingOutputs)`.
        // 4. Validate every output and return the first output error.
        // 5. Otherwise return `Ok(())`.
        for x in &self.outputs {
            match x.validate() {
                Ok(_) => {}
                Err(err) => {
                    return Err(err);
                }
            }
        }
        if self.txid.is_empty() {
            Err(BtcLibError::EmptyTxId)
        } else if !self.is_coinbase() && self.inputs.is_empty() {
            Err(BtcLibError::MissingInputs)
        } else if self.outputs.is_empty() {
            Err(BtcLibError::MissingOutputs)
        } else {
            Ok(())
        }
    }
}

impl BlockHeader {
    /// Build a block header by copying string fields and storing numbers.
    pub fn new(
        block_hash: &str,
        previous_block_hash: &str,
        merkle_root: &str,
        timestamp: u64,
        nonce: u64,
    ) -> Self {
        // Steps:
        // 1. Convert the three string fields into owned `String`s.
        // 2. Store `timestamp` and `nonce` unchanged.
        // 3. Return a `BlockHeader`.
        Self {
            block_hash: block_hash.to_string(),
            previous_block_hash: previous_block_hash.to_string(),
            merkle_root: merkle_root.to_string(),
            timestamp,
            nonce,
        }
    }
}

impl Block {
    /// Build a block from the provided header, transactions, height, and network.
    pub fn new(
        header: BlockHeader,
        transactions: Vec<Transaction>,
        height: u64,
        network: Network,
    ) -> Self {
        // Steps:
        // 1. Move `header` and `transactions` into the block.
        // 2. Store `height` and `network` unchanged.
        // 3. Return a `Block`.
        Self {
            header,
            transactions,
            height,
            network,
        }
    }

    /// Return how many transactions are in this block.
    pub fn transaction_count(&self) -> usize {
        // Steps:
        // 1. Return `self.transactions.len()`.
        self.transactions.len()
    }

    /// Sum the total output value of every transaction in the block.
    pub fn total_output_value(&self) -> u64 {
        // Steps:
        // 1. Start a total at 0.
        // 2. Add `transaction.total_output_value()` for each transaction.
        // 3. Return the total.
        // let total:u64 = self.transactions.iter().map(|x| x.outputs as u64).sum();
        // total
        let mut total = 0;
        for x in &self.transactions {
            let p: u64 = x.outputs.iter().map(|o| o.value_sats).sum();
            total = total + p
        }
        total
    }

    /// Return the first transaction with the matching txid, if one exists.
    pub fn find_transaction(&self, txid: &str) -> Option<&Transaction> {
        // Steps:
        // 1. Walk through transactions in order.
        // 2. Return `Some(transaction)` for the first exact txid match.
        // 3. Return `None` when no match exists.
        let tx = self.transactions.iter().find(|x| x.txid == txid);
        tx
    }
}

impl Hashable for Block {
    /// Return deterministic block hash material.
    ///
    /// Use exactly this format:
    /// `block:<block_hash>|prev:<previous_block_hash>|height:<height>|txs:<txid>;...`
    fn hash_material(&self) -> String {
        // Steps:
        // 1. Start with block hash, previous hash, and height in the format above.
        // 2. Append each transaction id followed by `;`.
        // 3. Return the final string.
        let block_hash = self.header.block_hash.clone();
        let prev_hash = self.header.previous_block_hash.clone();
        let hieght = self.height;
        let mut hs = format!(
            "block:{}|prev:{}|height:{}|txs:",
            block_hash, prev_hash, hieght
        );
        for x in &self.transactions {
            let fmt = format!("{};", x.txid);
            hs.push_str(fmt.as_str());
        }
        hs
    }
}

impl Validate for Block {
    /// Validate a block and its transactions.
    fn validate(&self) -> Result<(), BtcLibError> {
        // Steps:
        // 1. If there are no transactions, return `Err(BtcLibError::EmptyBlock)`.
        // 2. Check for duplicate transaction ids. Return `DuplicateTxId` on repeat.
        // 3. Validate every transaction and return the first validation error.
        // 4. Otherwise return `Ok(())`.
        let mut duplicate = Vec::new();
        let arr = self.transactions.clone();
        for x in arr {
            let m = self.find_transaction(&x.txid);
            match m {
                Some(x) => {
                    if duplicate.contains(x) {
                        return Err(BtcLibError::DuplicateTxId);
                    }
                    duplicate.push(x.clone());
                }
                _ => {}
            }
        }
        if self.transactions.is_empty() {
            Err(BtcLibError::EmptyBlock)
        } else {
            Ok(())
        }
    }
}

/// Parse `spent` or `unspent` into a `TxStatus`.
///
/// Trim whitespace, ignore ASCII case, and reject unknown values.
pub fn parse_status(input: &str) -> Result<TxStatus, BtcLibError> {
    // Steps:
    // 1. Trim whitespace from `input`.
    // 2. Compare using lowercase text.
    // 3. Return `Ok(TxStatus::Spent)` for "spent".
    // 4. Return `Ok(TxStatus::Unspent)` for "unspent".
    // 5. Return `Err(BtcLibError::MalformedData)` for anything else.
    let tx_input = input.trim().to_lowercase();
    if tx_input == "spent" {
        Ok(TxStatus::Spent)
    } else if tx_input == "unspent" {
        Ok(TxStatus::Unspent)
    } else {
        Err(BtcLibError::MalformedData)
    }
}

/// Parse a previous output reference.
///
/// The coinbase marker is `-`. Normal outpoints use `previous_txid:vout`.
pub fn parse_outpoint(input: &str) -> Result<Option<TxInput>, BtcLibError> {
    // Steps:
    // 1. Trim `input`.
    // 2. If it is exactly `COINBASE_PREVIOUS_OUTPUT`, return `Ok(None)`.
    // 3. Otherwise split once on `:`.
    // 4. Reject missing txid, missing vout, or non-numeric vout.
    // 5. Return `Ok(Some(TxInput::new(previous_txid, vout)))`.
    let data = input.trim();
    if input == "-" {
        return Ok(None);
    } else {
        let split: Vec<&str> = input.split(":").collect();
        if split.len() < 2 {
            return Err(BtcLibError::MalformedData);
        } else {
            let previous_txid = split[0];
            let vout = split[1].parse::<u32>();
            match vout {
                Ok(value) => return Ok(Some(TxInput::new(previous_txid, value))),
                Err(_) => {
                    return Err(BtcLibError::MalformedData);
                }
            }
        }
    }
}

/// Parse a row into the Week 3 transaction model.
///
/// Row format: `txid,previous_txid:vout,recipient,amount_sats,status`.
/// For coinbase, use `coinbase,-,recipient,amount_sats,status`.
pub fn parse_transaction(input: &str) -> Result<Transaction, BtcLibError> {
    // Steps:
    // 1. Split the row by commas.
    // 2. Require exactly five fields.
    // 3. Trim every field and reject empty txid, recipient, amount, or status.
    // 4. Parse the previous output field with `parse_outpoint`.
    // 5. If the previous output is `-`, require `txid == "coinbase"`.
    // 6. Parse amount as `u64` and reject zero.
    // 7. Parse status with `parse_status`.
    // 8. Build one output and a transaction with zero or one input.
    // 9. Do not use `unwrap()` or `expect()` in this parser.
    let split: Vec<&str> = input.split(",").collect();
    if split.len() < 5 || split[0] == "" {
        return Err(BtcLibError::MalformedData);
    }
    let split: Vec<&str> = split.iter().map(|x| x.trim()).collect();
    let previous_txid = parse_outpoint(split[1]);
    let status = parse_status(split[4])?;
    let amount = split[3].parse::<u64>();
    let mut amount_sat = 0;
    match amount {
        Ok(t) => {
            if t == 0 {
                return Err(BtcLibError::MalformedData);
            }
            amount_sat = t;
        }
        Err(_) => {
            return Err(BtcLibError::MalformedData);
        }
    }
    match previous_txid {
        Ok(x) => match x {
            Some(m) => {
                let mut txid = m.previous_txid;
                if txid == "-" {
                    txid = "coinbase".to_string()
                }
                let tx_input = vec![TxInput {
                    previous_txid: txid,
                    previous_vout: m.previous_vout,
                }];
                let tx_id = split[0].to_string();

                let outputs = vec![TxOutput {
                    recipient: split[2].to_string(),
                    status: status,
                    unique_id: Uuid::new_v4(),
                    value_sats: amount_sat,
                }];
                let transaction = Transaction {
                    txid: tx_id,
                    inputs: tx_input,
                    outputs,
                };
                return Ok(transaction);
            }
            None => {
                let m: Vec<&str> = split[1].split(":").collect();
                if m.len() < 2 && split[0] != "coinbase" {
                    return Err(BtcLibError::MalformedData);
                }
                // if split[1] == "coinbabe"{
                let tx_input: Vec<TxInput> = Vec::new();
                let outputs = vec![TxOutput {
                    recipient: split[2].to_string(),
                    status: status,
                    unique_id: Uuid::new_v4(),
                    value_sats: amount_sat,
                }];
                let transaction = Transaction {
                    txid: "coinbase".to_string(),
                    inputs: tx_input,
                    outputs,
                };
                return Ok(transaction);
                // }else {
                //     return Err(BtcLibError::MalformedData);
                // }
            }
        },
        Err(err) => {
            return Err(err);
        }
    }
}

/// Parse every row into a transaction.
///
/// Stop and return the first error if any row is malformed.
pub fn parse_transactions(lines: &[&str]) -> Result<Vec<Transaction>, BtcLibError> {
    // Steps:
    // 1. Create an empty `Vec<Transaction>`.
    // 2. Parse rows from first to last with `parse_transaction`.
    // 3. Push valid transactions into the vector.
    // 4. If a row returns an error, return that error immediately.
    // 5. Return `Ok(vec)` when all rows parse successfully.
    let mut transactions = Vec::new();
    for x in lines {
        let m = parse_transaction(x);
        match m {
            Ok(tx) => {
                transactions.push(tx);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok(transactions)
}

/// Parse all valid rows and skip malformed rows.
pub fn valid_transactions_only(lines: &[&str]) -> Vec<Transaction> {
    // Steps:
    // 1. Create an empty `Vec<Transaction>`.
    // 2. Try to parse every row.
    // 3. Push only successfully parsed transactions.
    // 4. Silently skip malformed rows.
    let mut transactions = Vec::new();
    for x in lines {
        let m = parse_transaction(x);
        match m {
            Ok(tx) => {
                transactions.push(tx);
            }
            Err(err) => {}
        }
    }
    transactions
}

/// Build and validate a block from parsed transaction rows.
pub fn build_block_from_rows(
    header: BlockHeader,
    rows: &[&str],
    height: u64,
    network: Network,
) -> Result<Block, BtcLibError> {
    // Steps:
    // 1. Parse all rows with `parse_transactions`.
    // 2. Build a `Block` from the parsed transactions.
    // 3. Validate the block.
    // 4. Return the block only when parsing and validation succeed.
    let tx = parse_transactions(rows);
    let mut transactions = Vec::new();
    match tx {
        Ok(x) => {
            transactions = x;
        }
        Err(err) => {
            return Err(err);
        }
    }
    let block = Block {
        header: header,
        height: height,
        network: network,
        transactions: transactions,
    };
    match block.validate() {
        Ok(_) => {}
        Err(err) => {
            return Err(err);
        }
    }
    Ok(block)
}

/// Validate every item in order.
///
/// Stop and return the first validation error, otherwise return `Ok(())`.
pub fn validate_all<T: Validate>(items: &[T]) -> Result<(), BtcLibError> {
    // Steps:
    // 1. Walk through `items` in order.
    // 2. Call `validate()` on each item.
    // 3. Return the first error immediately.
    // 4. Return `Ok(())` if every item is valid.
    for x in items {
        x.validate().unwrap();
    }
    Ok(())
}

/// Return the SHA-256 hex hash for every hashable item, preserving input order.
pub fn hash_all<T: Hashable>(items: &[T]) -> Vec<String> {
    // Steps:
    // 1. Create a new `Vec<String>`.
    // 2. For each item, call `hash_hex()`.
    // 3. Push the hash into the output vector.
    // 4. Preserve the original order.
    let mut hashs = Vec::new();
    for x in items {
        hashs.push(x.hash_hex().clone());
    }
    hashs
}

/// Decode a 64-character SHA-256 hex string into 32 bytes.
pub fn decode_hash_hex(input: &str) -> Result<[u8; 32], BtcLibError> {
    // Steps:
    // 1. Trim whitespace from `input`.
    // 2. Decode the string with the `hex` crate.
    // 3. Reject invalid hex or decoded values that are not exactly 32 bytes.
    // 4. Convert the decoded bytes into `[u8; 32]`.
    // 5. Return `Err(BtcLibError::InvalidHash)` for invalid input.
    let m = decode(input.trim());
    match m {
        Ok(x) => {
            if x.len() != 32 {
                Err(BtcLibError::InvalidHash)
            } else {
                let k: [u8; 32] = x.as_slice().try_into().unwrap();
                Ok(k)
            }
        }
        Err(_) => Err(BtcLibError::InvalidHash),
    }
}

/// Sum unspent output amounts across all transactions.
pub fn total_unspent(transactions: &[Transaction]) -> u64 {
    // Steps:
    // 1. Walk through every transaction and every output.
    // 2. Add `value_sats` only when the output is unspent.
    // 3. Return the total.
    let mut total = 0;
    for x in transactions {
        for m in &x.outputs {
            if m.is_unspent() {
                total = total + m.value_sats
            }
        }
    }
    total
}

/// Return a borrowed transaction with the matching txid, if one exists.
pub fn find_by_txid<'a>(transactions: &'a [Transaction], txid: &str) -> Option<&'a Transaction> {
    // Steps:
    // 1. Walk through the slice from first to last.
    // 2. Compare each transaction's txid with `txid`.
    // 3. Return `Some(transaction)` for the first exact match.
    // 4. Return `None` if no match exists.
    let m = transactions.iter().find(|x| x.txid == txid);
    match m {
        Some(x) => Some(x),
        None => None,
    }
}

/// Return the matching transaction or `BtcLibError::MissingTransaction`.
pub fn require_transaction<'a>(
    transactions: &'a [Transaction],
    txid: &str,
) -> Result<&'a Transaction, BtcLibError> {
    // Steps:
    // 1. Reuse `find_by_txid` or perform the same lookup.
    // 2. Return `Ok(transaction)` when found.
    // 3. Return `Err(BtcLibError::MissingTransaction)` when missing.
    let m = find_by_txid(transactions, txid);
    match m {
        Some(x) => Ok(x),
        None => Err(BtcLibError::MissingTransaction),
    }
}

/// Build an amount summary from all transaction outputs.
pub fn summarize_amounts(transactions: &[Transaction]) -> AmountSummary {
    let sat = 100_000_000;
    // Steps:
    // 1. Count every output across every transaction...
    // 2. Sum every output amount into `total_sats`....
    // 3. Sum spent output amounts into `spent_sats`.
    // 4. Sum unspent output amounts into `unspent_sats`.
    // 5. Return an `AmountSummary` with all four fields filled.
    let output_count: usize = transactions.iter().map(|x| x.outputs.len()).sum();
    let total_sats: u64 = transactions.iter().map(|x| x.total_output_value()).sum();

    let mut spent_sats = 0;
    for x in transactions {
        let p: u64 = x
            .outputs
            .iter()
            .filter(|m| !m.is_unspent())
            .map(|o| o.value_sats)
            .sum();
        spent_sats = spent_sats + p
    }

    let mut unspent_sats = 0;
    for x in transactions {
        let p: u64 = x
            .outputs
            .iter()
            .filter(|m| m.is_unspent())
            .map(|o| o.value_sats)
            .sum();
        unspent_sats = unspent_sats + p
    }
    AmountSummary {
        output_count,
        total_sats,
        spent_sats,
        unspent_sats,
    }
}
