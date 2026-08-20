#![allow(unused_variables)]

use std::{collections::HashMap, vec};

pub const SATS_PER_BTC: u64 = 100_000_000;
pub const GENESIS_HASH: &str = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";
pub const GENESIS_TIMESTAMP: u64 = 1_231_006_505;
pub const GENESIS_REWARD_SATS: u64 = 50 * SATS_PER_BTC;
pub const GENESIS_MESSAGE: &str =
    "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockTransaction {
    pub txid: String,
    pub sender: String,
    pub recipient: String,
    pub amount_sats: u64,
    pub confirmed: bool,
}

impl MockTransaction {
    /// Build a mock transaction by copying the borrowed string inputs into owned
    /// `String` fields and storing the amount and confirmation flag unchanged.
    pub fn new(
        txid: &str,
        sender: &str,
        recipient: &str,
        amount_sats: u64,
        confirmed: bool,
    ) -> Self {
        // Steps:
        // 1. Convert `txid`, `sender`, and `recipient` into owned `String`s.
        // 2. Store `amount_sats` and `confirmed` exactly as they are passed in.
        // 3. Return a `MockTransaction` with all five fields filled.
        Self {
            txid: txid.to_string(),
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount_sats,
            confirmed,
        }
    }
}

/// Return the hardcoded Bitcoin genesis block hash.
pub fn genesis_hash() -> &'static str {
    // Steps:
    // 1. Return the `GENESIS_HASH` constant.
    // 2. Do not allocate a new string.
    GENESIS_HASH
}

/// Return the hardcoded Unix timestamp for the Bitcoin genesis block.
pub fn genesis_timestamp() -> u64 {
    // Steps:
    // 1. Return the `GENESIS_TIMESTAMP` constant.
    GENESIS_TIMESTAMP
}

/// Return the genesis block reward in satoshis.
pub fn genesis_reward_sats() -> u64 {
    // Steps:
    // 1. Return the `GENESIS_REWARD_SATS` constant.
    // 2. Keep the unit in satoshis, not BTC.
    GENESIS_REWARD_SATS
}

/// Return the newspaper headline embedded in the genesis block coinbase data.
pub fn genesis_message() -> &'static str {
    // Steps:
    // 1. Return the `GENESIS_MESSAGE` constant.
    // 2. Do not allocate a new string.
    GENESIS_MESSAGE
}

/// Build a human-readable summary string containing the genesis hash,
/// timestamp, reward, and message.
pub fn genesis_summary() -> String {
    // Steps:
    // 1. Build and return an owned `String`.
    // 2. Use this format:
    //    Genesis block: hash=<hash>, timestamp=<timestamp>, reward_sats=<reward>, message="<message>"
    // 3. Fill each placeholder from the constants above.
    let message = format!(
        "Genesis block: hash=<{}>, timestamp=<{}>, reward_sats=<{}>, message=<{}>",
        GENESIS_HASH, GENESIS_TIMESTAMP, GENESIS_REWARD_SATS, GENESIS_MESSAGE
    );
    message
}

/// Calculate the Bitcoin block subsidy for a height.
///
/// Start at 50 BTC, halve every 210,000 blocks, and return zero after
/// 64 or more halvings.
pub fn block_subsidy(height: u64) -> u64 {
    let halvings = height / 210_000;
    if halvings >= 64 {
        return 0;
    }
    GENESIS_REWARD_SATS >> halvings
}

/// Format satoshis as a BTC string with exactly eight decimal places.
pub fn format_sats(sats: u64) -> String {
    // Steps:
    // 1. Divide by `SATS_PER_BTC` to get the whole BTC part.
    let btc = sats / SATS_PER_BTC;
    let remainder = sats % SATS_PER_BTC;
    // 2. Use the remainder for the fractional satoshi part.
    let full_btc = format!("{}.{:08} BTC", btc, remainder);
    // 3. Return exactly this format: "<whole>.<fraction padded to 8 digits> BTC".
    // 4. Example: 1 sat becomes "0.00000001 BTC".
    full_btc.to_string()
}

/// Count transactions where `confirmed` is true.
pub fn count_confirmed(transactions: &[MockTransaction]) -> usize {
    // Steps:
    // 1. Look through every transaction in the slice.
    // 2. Count only transactions where `confirmed` is true.
    // 3. Return the count.
    let mut confirmation_num = 0;
    for x in transactions {
        if x.confirmed == true {
            confirmation_num = confirmation_num + 1
        }
    }
    confirmation_num
}

/// Count transactions where `confirmed` is false.
pub fn count_unconfirmed(transactions: &[MockTransaction]) -> usize {
    // Steps:
    // 1. Look through every transaction in the slice.
    // 2. Count only transactions where `confirmed` is false.
    // 3. Return the count.
    let mut unconfirmation_num = 0;
    for x in transactions {
        if x.confirmed == false {
            unconfirmation_num = unconfirmation_num + 1
        }
    }
    unconfirmation_num
}

/// Sum the amount of every transaction, confirmed and unconfirmed.
pub fn total_amount(transactions: &[MockTransaction]) -> u64 {
    // Steps:
    // 1. Start a running total at 0.
    // 2. Add every transaction's `amount_sats`, regardless of confirmation.
    // 3. Return the total.
    let mut txs_amount = 0;
    for x in transactions {
        txs_amount = txs_amount + x.amount_sats
    }
    txs_amount
}

/// Return the integer average transaction amount.
///
/// Return zero when the input slice is empty.
pub fn average_amount(transactions: &[MockTransaction]) -> u64 {
    // Steps:
    // 1. If the slice is empty, return 0.
    if transactions.is_empty() {
        return 0;
    } else {
        let m = total_amount(transactions) / transactions.len() as u64;
        return m;
    }
    // 2. Otherwise compute `total_amount(transactions) / transactions.len()`.
    // 3. Integer division should round down naturally.
}

/// Return cloned transactions whose sender exactly matches `sender`.
///
/// Preserve the original order.
pub fn filter_by_sender(transactions: &[MockTransaction], sender: &str) -> Vec<MockTransaction> {
    // Steps:
    // 1. Create a new `Vec<MockTransaction>`.
    let mut new = vec![];
    // 2. Walk through the input slice in order.
    for x in transactions {
        if x.sender == sender {
            new.push(x.clone());
        }
    }
    // 3. Clone and push transactions whose `sender` equals the requested sender.
    // 4. Return the new vector.
    new
}

/// Return cloned transactions whose recipient exactly matches `recipient`.
///
/// Preserve the original order.
pub fn filter_by_recipient(
    transactions: &[MockTransaction],
    recipient: &str,
) -> Vec<MockTransaction> {
    // Steps:
    // 1. Create a new `Vec<MockTransaction>`.
    let mut new = vec![];
    // 2. Walk through the input slice in order.
    for x in transactions {
        if x.recipient == recipient {
            new.push(x.clone());
        }
    }
    // 3. Clone and push transactions whose `recipient` equals the requested recipient.
    // 4. Return the new vector.
    new
}

/// Return cloned transactions that are confirmed.
///
/// Preserve the original order.
pub fn filter_confirmed(transactions: &[MockTransaction]) -> Vec<MockTransaction> {
    // Steps:
    // 1. Create a new vector.
    // 2. Add cloned transactions only when `confirmed` is true.
    // 3. Keep the same order as the input slice.
    let mut new = vec![];
    for x in transactions {
        if x.confirmed == true {
            new.push(x.clone());
        }
    }
    new
}

/// Return all transaction ids as owned strings in their original order.
pub fn transaction_ids(transactions: &[MockTransaction]) -> Vec<String> {
    // Steps:
    // 1. Create a new `Vec<String>`.
    // 2. For each transaction, clone its `txid`.
    // 3. Push the cloned txid in the same order as the input.
    let mut new = vec![];
    for x in transactions {
        new.push(x.txid.clone());
    }
    new
}

/// Find the first transaction with a matching txid and return an owned clone.
///
/// Return `None` when no transaction matches.
pub fn find_transaction(transactions: &[MockTransaction], txid: &str) -> Option<MockTransaction> {
    // Steps:
    // 1. Walk through transactions from first to last.
    // 2. When `transaction.txid == txid`, return `Some(transaction.clone())`.
    // 3. If no match is found, return `None`.
    let m = transactions.iter().find(|x| x.txid == txid).cloned();
    m
}

/// Return amounts that are strictly greater than `minimum_sats`.
///
/// Preserve the original order.
pub fn amounts_over(transactions: &[MockTransaction], minimum_sats: u64) -> Vec<u64> {
    // Steps:
    // 1. Create a new `Vec<u64>`.
    // 2. For each transaction, compare `amount_sats` with `minimum_sats`.
    // 3. Push only amounts strictly greater than the minimum.
    // 4. Do not include amounts equal to the minimum.
    let mut new = vec![];
    for x in transactions {
        if x.amount_sats > minimum_sats {
            new.push(x.amount_sats);
        }
    }
    new
}

/// Build a balance map from confirmed transactions only.
///
/// Subtract each confirmed amount from the sender and add it to the recipient.
pub fn build_balances(transactions: &[MockTransaction]) -> HashMap<String, i64> {
    // Steps:
    // 1. Create an empty `HashMap<String, i64>`.
    // 2. Skip every transaction where `confirmed` is false.
    // 3. For confirmed transactions, subtract `amount_sats` from the sender.
    // 4. Add `amount_sats` to the recipient.
    // 5. Convert amounts to `i64` before applying negative changes.
    let mut new: HashMap<String, i64> = HashMap::new();
    for x in transactions {
        if x.confirmed == true {
            // let balance = x.amount_sats -
            // recipient
            let m = x.amount_sats as i64;
            new.entry(x.recipient.clone()).or_insert(0);
            let recipient = new.get(&x.recipient).unwrap().clone();
            new.entry(x.recipient.clone()).insert_entry(recipient + m);

            // sender
            new.entry(x.sender.clone()).or_insert(0);
            let sender = new.get(&x.sender).unwrap().clone();
            new.entry(x.sender.clone())
                .insert_entry(sender - x.amount_sats as i64);
        }
    }
    new
}

/// Sum confirmed amounts received by `address`.
pub fn address_received_total(transactions: &[MockTransaction], address: &str) -> u64 {
    // Steps:
    // 1. Look only at confirmed transactions.
    // 2. Add `amount_sats` when `recipient == address`.
    // 3. Return 0 if there are no matches.
    let mut amount = 0;
    for x in transactions {
        if x.confirmed == true && x.recipient == address {
            amount = amount + x.amount_sats;
        }
    }
    amount
}

/// Sum confirmed amounts sent by `address`.
pub fn address_sent_total(transactions: &[MockTransaction], address: &str) -> u64 {
    // Steps:
    // 1. Look only at confirmed transactions.
    // 2. Add `amount_sats` when `sender == address`.
    // 3. Return 0 if there are no matches.
    let mut amount = 0;
    for x in transactions {
        if x.confirmed == true && x.sender == address {
            amount = amount + x.amount_sats;
        }
    }
    amount
}

/// Return confirmed received total minus confirmed sent total for `address`.
pub fn net_balance_change(transactions: &[MockTransaction], address: &str) -> i64 {
    // Steps:
    // 1. Reuse or mirror the received-total and sent-total calculations.
    let mut received: i64 = 0;
    let mut sent: i64 = 0;
    for x in transactions {
        if x.recipient == address {
            received = received + x.amount_sats as i64;
        } else if x.sender == address {
            sent = sent + x.amount_sats as i64;
        }
    }
    // 2. Convert both totals to `i64`.
    // 3. Return `received - sent`.
    return received - sent;
}

/// Return true when the transaction sender is exactly `"coinbase"`.
pub fn is_coinbase(transaction: &MockTransaction) -> bool {
    // Steps:
    // 1. Compare `transaction.sender` with the string literal `"coinbase"`.
    // 2. Return the boolean result.
    if transaction.sender == "coinbase" {
        return true;
    } else {
        return false;
    }
}

/// Classify an amount as `"dust"`, `"micro"`, `"standard"`, or `"large"`.
///
/// Use the thresholds described in this week's README.
pub fn classify_amount(sats: u64) -> &'static str {
    // Steps:
    // 1. Return "dust" for values below 546.
    // 2. Return "micro" for values from 546 up to 99_999.
    // 3. Return "standard" for values from 100_000 up to 99_999_999.
    // 4. Return "large" for values at or above 100_000_000.
    if sats < 567 {
        return "dust";
    } else if sats <= 99_999 {
        return "micro";
    } else if sats <= 99_999_999 {
        return "standard";
    } else {
        return "large";
    }
}
