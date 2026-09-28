use crate::{Block, BtcLibError, Transaction, TxStatus};

pub trait Hashable {
    /// Return stable material that will be hashed.
    fn hash_material(&self) -> String;

    /// Hash `hash_material()` with SHA-256 and return lowercase hex.
    fn hash_hex(&self) -> String {
        sha256::digest(self.hash_material())
    }
}

impl Hashable for Transaction {
    /// Return deterministic transaction material.
    ///
    /// Use exactly:
    /// `tx:<txid>|inputs:<previous_txid>:<previous_vout>;...|outputs:<value>:<recipient>:<status>;...`
    ///
    /// Status text must be lowercase: `spent` or `unspent`.
    fn hash_material(&self) -> String {
        // Steps:
        // 1. Start with `tx:<txid>|inputs:`.
        // 2. Append every input as `<previous_txid>:<previous_vout>;`.
        // 3. Append `|outputs:`.
        // 4. Append every output as `<value_sats>:<recipient>:<status>;`.
        // 5. Return the final string.
        let txid = self.txid.clone();
        let mut hash = format!("tx:{}|inputs:", txid);
        for x in &self.inputs {
            let input = format!("{}:{};", x.previous_txid, x.previous_vout);
            hash.push_str(&input);
        }
        hash.push_str("|outputs:");
        for x in &self.outputs {
            let mut status = "";
            if x.status == TxStatus::Spent {
                status = "spent"
            } else {
                status = "unspent"
            }
            let input = format!("{}:{}:{};", x.value_sats, x.recipient, status);
            hash.push_str(&input);
        }

        hash
    }
}

impl Hashable for Block {
    /// Return deterministic block material.
    ///
    /// Use exactly:
    /// `block:<block_hash>|prev:<previous_block_hash>|merkle:<merkle_root>|height:<height>|txs:<txid>;...`
    fn hash_material(&self) -> String {
        // Steps:
        // 1. Start with block hash, previous hash, merkle root, and height in the format above.
        // 2. Append each transaction id followed by `;`.
        // 3. Return the final string.
        let block_hash = self.header.block_hash.clone();
        let mut hash = format!(
            "block:{}|prev:{}:|merkle:{}|height:{}|",
            block_hash, self.header.previous_block_hash, self.header.merkle_root, self.height
        );
        for x in &self.transactions {
            let input = format!("txs:{};", x.txid,);
            hash.push_str(&input);
        }
        hash.push_str(";");
        hash
    }
}

/// Hash two child hashes into their parent merkle node.
pub fn pair_hash(left: &str, right: &str) -> String {
    // Steps:
    // 1. Build the exact string `<left><right>` with no separator.
    // 2. Return `sha256::digest(joined_string)`.
    let joined_string = format!("{}{}", left, right);
    sha256::digest(joined_string)
}

/// Calculate a simple merkle root from transaction hashes.
///
/// If a level has an odd number of hashes, duplicate the last hash before pairing.
pub fn calculate_merkle_root(transactions: &[Transaction]) -> Result<String, BtcLibError> {
    // Steps:
    // 1. Reject an empty transaction slice with `BtcLibError::EmptyBlock`.
    // 2. Start with each transaction's `hash_hex()`.
    // 3. While more than one hash remains, pair hashes left-to-right.
    // 4. When a level has an odd count, pair the last hash with itself.
    // 5. Return the only remaining hash.
    if transactions.len() == 0 {
        return Err(BtcLibError::EmptyBlock);
    }
    let mut current_level: Vec<String> = transactions.iter().map(|tx| tx.hash_hex()).collect();

    // 3. While more than one hash remains, pair hashes left-to-right.
    while current_level.len() > 1 {
        let mut next_level = Vec::new();

        // 4. Use .chunks(2) to safely grab pairs.
        for chunk in current_level.chunks(2) {
            let left = &chunk[0];

            // If there's no right element (odd count), pair the last hash with itself.
            let right = if chunk.len() == 2 {
                &chunk[1]
            } else {
                &chunk[0]
            };

            let parent_hash = pair_hash(left, right);
            next_level.push(parent_hash);
        }

        current_level = next_level;
    }

    // 5. Return the only remaining hash.
    Ok(current_level.remove(0))
}

/// Validate that the block header stores the merkle root for its transactions.
pub fn validate_merkle_root(block: &Block) -> Result<(), BtcLibError> {
    // Steps:
    // 1. Calculate the expected merkle root for `block.transactions`.
    // 2. Compare it with `block.header.merkle_root`.
    // 3. Return `Ok(())` on an exact match.
    // 4. Return `Err(BtcLibError::InvalidMerkleRoot)` on mismatch.
    match calculate_merkle_root(&block.transactions) {
        Ok(root) => {
            if root == block.header.merkle_root {
                return Ok(());
            } else {
                return Err(BtcLibError::InvalidMerkleRoot);
            }
        }
        Err(err) => {
            return Err(err);
        }
    }
}
