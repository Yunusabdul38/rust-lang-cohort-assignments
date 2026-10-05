use crate::{CandidateBlock, Hashable, MinerError, Transaction};

/// Return true when a hash starts with the configured difficulty prefix.
pub fn hash_meets_difficulty(hash: &str, difficulty_prefix: &str) -> Result<bool, MinerError> {
    // Steps:
    // 1. Reject a prefix containing non-ASCII-hex characters with `InvalidDifficulty`.
    // 2. Compare using lowercase text so `A` and `a` are treated the same.
    // 3. Return whether `hash` starts with the normalized prefix.
    if !difficulty_prefix.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(MinerError::InvalidDifficulty);
    }

    // 2. Compare using lowercase text so `A` and `a` are treated the same.
    let normalized_prefix = difficulty_prefix.to_ascii_lowercase();
    let normalized_hash = hash.to_ascii_lowercase();

    Ok(normalized_hash.starts_with(&normalized_prefix))
}

/// Calculate a simple merkle root from transaction hashes.
pub fn calculate_merkle_root(transactions: &[Transaction]) -> Result<String, MinerError> {
    // Steps:
    // 1. Reject an empty list with `MinerError::EmptyCandidate`.
    // 2. Start with each transaction's `hash_hex()`.
    // 3. Pair hashes left-to-right and hash the concatenated pair.
    // 4. If a level has an odd count, duplicate the final hash.
    // 5. Return the final remaining hash.
    if transactions.is_empty() {
        return Err(MinerError::EmptyCandidate);
    }

    let mut current_level: Vec<String> = transactions.iter().map(|tx| tx.hash_hex()).collect();
    while current_level.len() > 1 {
        let mut next_level = Vec::with_capacity((current_level.len() + 1) / 2);

        for chunk in current_level.chunks(2) {
            let combined = match chunk {
                [left, right] => format!("{}{}", left, right),
                // 4. If a level has an odd count, duplicate the final hash.
                [left] => format!("{}{}", left, left),
                _ => unreachable!(),
            };

            next_level.push(sha256::digest(combined));
        }

        current_level = next_level;
    }

    Ok(current_level.remove(0))
}

/// Build deterministic candidate hash material for a nonce.
///
/// Use exactly:
/// `candidate:<previous_hash>|height:<height>|merkle:<merkle>|time:<timestamp>|nonce:<nonce>|txs:<txid>;...`
pub fn candidate_hash_material(
    candidate: &CandidateBlock,
    nonce: u64,
) -> Result<String, MinerError> {
    // Steps:
    // 1. Calculate the merkle root for `candidate.transactions`.
    // 2. Start the string with previous hash, height, merkle root, timestamp, and nonce.
    // 3. Append every transaction id followed by `;`.
    // 4. Return the final string.
    let merkle_root = calculate_merkle_root(&candidate.transactions)?;
    let mut material = format!(
        "candidate:{}|height:{}|merkle:{}|time:{}|nonce:{}|txs:",
        candidate.previous_block_hash, candidate.height, merkle_root, candidate.timestamp, nonce
    );

    for tx in &candidate.transactions {
        material.push_str(&tx.txid);
        material.push(';');
    }

    Ok(material)
}

/// Hash a candidate block at one nonce.
pub fn hash_candidate(candidate: &CandidateBlock, nonce: u64) -> Result<String, MinerError> {
    // Steps:
    // 1. Build candidate hash material with `candidate_hash_material`.
    // 2. Return `sha256::digest(material)`.
    let material = candidate_hash_material(candidate, nonce)?;
    Ok(sha256::digest(material))
}
