#![allow(unused_variables)]

use std::process::Output;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
pub enum ValidationError {
    EmptyTxId,
    MissingInputs,
    MissingOutputs,
    ZeroValueOutput,
    EmptyBlock,
    DuplicateTxId,
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

pub trait Identifiable {
    /// Return the stable identifier for this value.
    fn id(&self) -> &str;
}

impl TxInput {
    /// Build a transaction input by copying the previous txid and storing vout.
    pub fn new(previous_txid: &str, previous_vout: u32) -> Self {
        // Steps:
        // 1. Convert `previous_txid` into an owned `String`.
        // 2. Store `previous_vout` unchanged.
        // 3. Return a `TxInput` with both fields filled.
        let tx_input = TxInput {
            previous_txid: previous_txid.to_string(),
            previous_vout,
        };
        tx_input
    }
}

impl TxOutput {
    /// Build a transaction output by copying the recipient and storing value/status.
    pub fn new(value_sats: u64, recipient: &str, status: TxStatus) -> Self {
        // Steps:
        // 1. Store `value_sats` unchanged.
        // 2. Generate a fresh `Uuid` with `Uuid::new_v4()` for `unique_id`.
        // 3. Convert `recipient` into an owned `String`.
        // 4. Store `status` unchanged.
        // 5. Return a `TxOutput`.
        let id = Uuid::new_v4();
        let tx_output = TxOutput {
            recipient: recipient.to_string(),
            status,
            unique_id: id,
            value_sats,
        };
        tx_output
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

impl Transaction {
    /// Build a transaction by copying the txid and storing the provided inputs
    /// and outputs.
    pub fn new(txid: &str, inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        // Steps:
        // 1. Convert `txid` into an owned `String`.
        // 2. Move `inputs` and `outputs` into the transaction.
        // 3. Return a `Transaction`.
        let tx = Transaction {
            inputs,
            outputs,
            txid: txid.to_string(),
        };
        tx
    }

    /// Return true for the simplified coinbase rule used in this assignment:
    /// txid is `"coinbase"` and there are no inputs.
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
        // 2. Add each output's `value_sats`.
        // 3. Return the total.
        let mut total = 0;
        for x in &self.outputs {
            total = total + x.value_sats
        }
        total
    }

    /// Count outputs whose status is `TxStatus::Unspent`.
    pub fn unspent_output_count(&self) -> usize {
        // Steps:
        // 1. Walk through `self.outputs`.
        // 2. Count outputs where `status == TxStatus::Unspent`.
        // 3. Return the count.
        let mut total = 0;
        for x in &self.outputs {
            if x.status == TxStatus::Unspent {
                total += 1
            }
        }
        total
    }

    /// Count outputs whose status is `TxStatus::Spent`.
    pub fn spent_output_count(&self) -> usize {
        // Steps:
        // 1. Walk through `self.outputs`.
        // 2. Count outputs where `status == TxStatus::Spent`.
        // 3. Return the count.
        let mut total = 0;
        for x in &self.outputs {
            if x.status == TxStatus::Spent {
                total += 1
            }
        }
        total
    }

    /// Validate this transaction using the rules in the README.
    ///
    /// Return the first matching `ValidationError`, otherwise `Ok(())`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Steps:
        // 1. If `txid` is empty, return `Err(ValidationError::EmptyTxId)`.
        // 2. If the transaction is not coinbase and has no inputs, return
        //    `Err(ValidationError::MissingInputs)`.
        // 3. If there are no outputs, return `Err(ValidationError::MissingOutputs)`.
        // 4. If any output has value 0, return `Err(ValidationError::ZeroValueOutput)`.
        // 5. Otherwise return `Ok(())`.
        let mut empty_output = false;
        for x in &self.outputs {
            if x.value_sats == 0 {
                empty_output = true
            }
        }
        if self.txid.is_empty() {
            return Err(ValidationError::EmptyTxId);
        } else if !self.is_coinbase() && self.inputs.len() == 0 {
            return Err(ValidationError::MissingInputs);
        } else if self.outputs.len() == 0 {
            return Err(ValidationError::MissingOutputs);
        } else if empty_output {
            return Err(ValidationError::ZeroValueOutput);
        } else {
            Ok(())
        }
    }
}

impl Identifiable for Transaction {
    /// Return this transaction's txid.
    fn id(&self) -> &str {
        // Steps:
        // 1. Return `self.txid.as_str()`.
        // 2. Do not allocate a new string.
        &self.txid
    }
}

impl BlockHeader {
    /// Build a block header by copying the string fields and storing timestamp
    /// and nonce.
    pub fn new(
        block_hash: &str,
        previous_block_hash: &str,
        merkle_root: &str,
        timestamp: u64,
        nonce: u64,
    ) -> Self {
        // Steps:
        // 1. Convert `block_hash`, `previous_block_hash`, and `merkle_root`
        //    into owned `String`s.
        // 2. Store `timestamp` and `nonce` unchanged.
        // 3. Return a `BlockHeader`.
        let block_header = Self {
            block_hash: block_hash.to_string(),
            merkle_root: merkle_root.to_string(),
            nonce,
            previous_block_hash: previous_block_hash.to_string(),
            timestamp,
        };
        block_header
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
        let block = Self {
            header,
            height,
            network,
            transactions,
        };
        block
    }

    /// Return how many transactions are in this block.
    pub fn transaction_count(&self) -> usize {
        // Steps:
        // 1. Return the length of `self.transactions`.
        let mut total = 0;
        for x in &self.transactions {
            total += 1
        }
        total
    }

    /// Sum the total output value of all transactions in this block.
    pub fn total_output_value(&self) -> u64 {
        // Steps:
        // 1. Start a total at 0.
        // 2. For each transaction, add `transaction.total_output_value()`.
        // 3. Return the total.
        let mut total = 0;
        for x in &self.transactions {
            total += x.total_output_value()
        }
        total
    }

    /// Return the first coinbase transaction in this block, if one exists.
    pub fn coinbase_transaction(&self) -> Option<&Transaction> {
        // Steps:
        // 1. Walk through transactions in order.
        // 2. Return `Some(transaction)` for the first transaction where
        //    `transaction.is_coinbase()` is true.
        // 3. Return `None` if no coinbase transaction exists.
        let mut coinbase = None;
        for x in &self.transactions {
            if x.is_coinbase() {
                coinbase = Some(x);
            }
        }
        coinbase
    }

    /// Return a borrowed transaction with the matching txid, if one exists.
    pub fn find_transaction(&self, txid: &str) -> Option<&Transaction> {
        // Steps:
        // 1. Walk through transactions in order.
        // 2. Compare each transaction's `txid` with the requested txid.
        // 3. Return `Some(transaction)` for the first match.
        // 4. Return `None` if no match exists.
        let mut tx = None;
        for x in &self.transactions {
            if x.txid == txid {
                tx = Some(x);
                break;
            }
        }
        return tx;
    }

    /// Validate this block using the rules in the README.
    ///
    /// Return the first matching `ValidationError`, otherwise `Ok(())`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Steps:
        // 1. If there are no transactions, return `Err(ValidationError::EmptyBlock)`.
        // 2. Check for duplicate transaction ids and return
        //    `Err(ValidationError::DuplicateTxId)` if any id repeats.
        // 3. Validate each transaction using `transaction.validate()`.
        // 4. Return the first transaction validation error if one occurs.
        // 5. Otherwise return `Ok(())`.
        let mut duplicate = false;

        for x in &self.transactions {
            let k: Vec<&Transaction> = self
                .transactions
                .iter()
                .filter(|l| l.txid == x.txid)
                .collect();
            if k.len() > 1 {
                duplicate = true;
            }
        }
        if self.transactions.is_empty() {
            return Err(ValidationError::EmptyBlock);
        } else if duplicate == true {
            return Err(ValidationError::DuplicateTxId);
        } else {
            Ok(())
        }
    }
}

impl Identifiable for Block {
    /// Return this block's block hash.
    fn id(&self) -> &str {
        // Steps:
        // 1. Return `self.header.block_hash.as_str()`.
        // 2. Do not allocate a new string.
        &self.header.block_hash
    }
}

/// Return the Bitcoin network magic value for a network.
pub fn network_magic(network: Network) -> u32 {
    // Steps:
    // 1. Match on the `Network` enum.
    // 2. Return the exact magic value listed in the README.
    // 3. Keep the values as `u32`.
    match network {
        Network::Mainnet => {
            return 0xD9B4BEF9;
        }
        Network::Testnet => {
            return 0x0709110B;
        }
        Network::Signet => {
            return 0x40CF030A;
        }
        Network::Regtest => {
            return 0xDAB5BFFA;
        }
    }
}

/// Convert a known network magic value back to a `Network`.
///
/// Return `None` for unknown magic values.
pub fn network_from_magic(magic: u32) -> Option<Network> {
    // Steps:
    // 1. Compare `magic` against each known magic value.
    // 2. Return `Some(Network::...)` for a match.
    // 3. Return `None` when the value is unknown.
    if magic == 0xD9B4BEF9 {
        return Some(Network::Mainnet);
    } else if magic == 0x0709110B {
        return Some(Network::Testnet);
    } else if magic == 0x40CF030A {
        return Some(Network::Signet);
    } else if magic == 0xDAB5BFFA {
        return Some(Network::Regtest);
    } else {
        return None;
    }
}

/// Count unspent outputs across all transactions.
pub fn count_unspent_outputs(transactions: &[Transaction]) -> usize {
    // Steps:
    // 1. Walk through every transaction.
    // 2. Add that transaction's unspent output count.
    // 3. Return the combined count.
    let mut unspent = 0;
    for x in transactions {
        unspent = unspent + x.unspent_output_count();
    }
    unspent
}

/// Sum output values whose recipient exactly matches `recipient`.
pub fn total_value_for_recipient(transactions: &[Transaction], recipient: &str) -> u64 {
    // Steps:
    // 1. Walk through every transaction and every output.
    // 2. Add `value_sats` only when `output.recipient == recipient`.
    // 3. Return 0 if no outputs match.
    let mut value = 0;
    for x in transactions {
        for j in &x.outputs {
            if j.recipient == recipient {
                value = value + j.value_sats;
            }
        }
    }
    value
}

/// Compare two values through the `Identifiable` trait.
pub fn have_same_id<T: Identifiable, U: Identifiable>(left: &T, right: &U) -> bool {
    // Steps:
    // 1. Call `id()` on both values.
    // 2. Compare the returned string slices.
    // 3. Return true if they are equal.
    if left.id() == right.id() {
        return true;
    } else {
        return false;
    }
}

/// Collect ids from dynamic trait objects into owned strings.
pub fn collect_ids(items: &[Box<dyn Identifiable>]) -> Vec<String> {
    // Steps:
    // 1. Create a new `Vec<String>`.
    // 2. For each trait object, call `id()`.
    // 3. Convert the borrowed id into an owned `String`.
    // 4. Preserve the input order.

    let mut ids = Vec::new();
    for x in items {
        ids.push(x.id().to_string());
    }
    ids
}
