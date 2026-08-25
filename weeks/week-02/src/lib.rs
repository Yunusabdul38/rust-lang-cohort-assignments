#![allow(unused_variables)]

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedOutpoint<'a> {
    pub txid: &'a str,
    pub vout: u32,
}

/// Return true when `input` reads the same forward and backward.
///
/// Ignore ASCII case, whitespace, and punctuation.
pub fn is_palindrome(input: &str) -> bool {
    // Steps:
    // 1. Keep only ASCII alphanumeric characters from `input`.
    // 2. Convert those characters to lowercase.
    // 3. Compare the cleaned sequence with its reverse.
    // 4. Empty cleaned input should count as a palindrome.
    let cleaned: Vec<char> = input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    cleaned.iter().eq(cleaned.iter().rev())
}

/// Compute the assignment toy hash.
///
/// Start at zero and, for each byte, update with `hash = hash * 31 + byte`
/// using wrapping arithmetic.
pub fn simple_hash(input: &str) -> u64 {
    // Steps:
    // 1. Start `hash` at 0_u64.
    // 2. Loop over `input.bytes()`.
    // 3. For each byte, use `wrapping_mul(31)` and `wrapping_add(byte as u64)`.
    // 4. Return the final hash value.
    let mut hash = 0_u64;
    for i in input.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(i as u64);
    }
    return hash;
}

/// Return `input_sats - output_sats` when inputs cover outputs.
///
/// Return `None` if outputs are larger than inputs.
pub fn calculate_fee(input_sats: u64, output_sats: u64) -> Option<u64> {
    // Steps:
    // 1. Compare `input_sats` and `output_sats`.
    // 2. If outputs are larger, return `None`.
    // 3. Otherwise return `Some(input_sats - output_sats)`.
    if output_sats > input_sats {
        return None;
    } else {
        return Some(input_sats - output_sats);
    }
}

/// Return the fee rate in sats/vbyte, rounded up.
///
/// Return `None` when `vbytes` is zero.
pub fn fee_rate(fee_sats: u64, vbytes: u64) -> Option<u64> {
    // Steps:
    // 1. If `vbytes` is 0, return `None`.
    // 2. Otherwise divide `fee_sats` by `vbytes`, rounding up.
    // 3. Return the result in `Some(...)`.
    // 4. Example: 251 sats over 100 vbytes should return 3.
    if vbytes == 0 {
        return None;
    } else {
        let cal = fee_sats.div_ceil(vbytes);
        return Some(cal);
    }
}

/// Return the longer borrowed string slice.
///
/// If both have the same length, return `left`.
pub fn select_longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    // Steps:
    // 1. Compare `left.len()` and `right.len()`.
    // 2. Return `right` only when it is strictly longer.
    // 3. Return `left` when it is longer or equal.
    if right.len() > left.len() {
        return right;
    } else {
        return left;
    }
}

/// Return the first whitespace-separated word from `input`.
///
/// Skip leading whitespace and return an empty slice for empty or all-whitespace
/// input.
pub fn first_word(input: &str) -> &str {
    // Steps:
    // 1. Ignore leading whitespace.
    // 2. Find the first word separated by whitespace.
    // 3. Return a borrowed slice from `input`, not a new `String`.
    // 4. Return "" if no word exists.
    if input.is_empty() {
        return "";
    } else {
        let m: Vec<&str> = input.trim().split(" ").collect();
        return m[0];
    }
}

/// Return the last whitespace-separated word from `input`.
///
/// Ignore trailing whitespace and return an empty slice for empty or all-whitespace
/// input.
pub fn last_word(input: &str) -> &str {
    // Steps:
    // 1. Ignore trailing whitespace.
    // 2. Find the final word separated by whitespace.
    // 3. Return a borrowed slice from `input`, not a new `String`.
    // 4. Return "" if no word exists.
    if input.is_empty() {
        return "";
    } else {
        let m: Vec<&str> = input.trim().split(" ").collect();
        return m.last().unwrap();
    }
}

/// Remove `prefix` from the front of `input` when it is present.
///
/// Return the original borrowed `input` slice when the prefix is missing.
pub fn trim_prefix<'a>(input: &'a str, prefix: &str) -> &'a str {
    // Steps:
    // 1. Check whether `input` starts with `prefix`.
    // 2. If it does, return the part of `input` after the prefix.
    // 3. If it does not, return `input` unchanged.
    // 4. An empty prefix should return `input` unchanged.
    if input.starts_with(prefix) {
        let m = input.strip_prefix(prefix).unwrap();
        return m;
    } else {
        return input;
    }
}

/// Parse a trimmed unsigned satoshi amount.
///
/// Return `None` for empty, negative, or non-numeric input.
pub fn parse_sats(input: &str) -> Option<u64> {
    // Steps:
    // 1. Trim whitespace from `input`.
    // 2. Return `None` if the trimmed string is empty.
    // 3. Try to parse the trimmed string as `u64`.
    // 4. Return `Some(value)` on success, `None` on parse failure.
    let m = input.trim();
    if m.is_empty() {
        return None;
    } else {
        let num = m.parse::<u64>();
        match num {
            Ok(x) => {
                return Some(x);
            }
            Err(_) => {
                return None;
            }
        }
    }
}

/// Split `input` once on the first colon and trim both sides.
///
/// Return `None` when no colon exists.
pub fn split_once_colon(input: &str) -> Option<(&str, &str)> {
    // Steps:
    // 1. Find the first `:` in `input`.
    // 2. Split into left and right borrowed slices.
    // 3. Trim whitespace from both slices.
    // 4. Return `Some((left, right))`, or `None` if there is no colon.
    if input.contains(":") {
        let m: Vec<&str> = input.trim().split(":").collect();
        let mut k = Vec::new();
        for x in m {
            k.push(x.trim());
        }
        return Some((k[0], k[1]));
    } else {
        return None;
    }
}

/// Join transaction ids with commas.
///
/// Return an empty string for an empty slice.
pub fn join_txids(txids: &[&str]) -> String {
    // Steps:
    // 1. Keep the txids in their original order.
    // 2. Join them using a comma with no spaces.
    // 3. Example: ["a", "b", "c"] becomes "a,b,c".
    let m = txids.join(",");
    m
}

/// Trim, lowercase, and replace runs of whitespace with single hyphens.
pub fn normalize_label(input: &str) -> String {
    // Steps:
    // 1. Trim leading and trailing whitespace.
    let m = input.trim();
    // 2. Split the remaining text on whitespace.
    let m: Vec<&str> = m.split(" ").collect();
    println!("{:?}", m);
    // 3. Lowercase each word.
    let mut new = String::new();
    for x in m {
        if x != "" {
            new.push_str(" ");
            new.push_str(&x.to_ascii_lowercase());
        }
    }
    let m = new.trim().replace(" ", "-");
    m.to_string()
}

/// Return true when `needle` exactly matches one of the owned txids.
pub fn contains_txid(txids: &[String], needle: &str) -> bool {
    // Steps:
    // 1. Walk through the borrowed slice of `String`s.
    // 2. Compare each value with `needle`.
    // 3. Return true on the first exact match, otherwise false.
    let mut is_include = false;
    for x in txids {
        if x == needle {
            is_include = true;
        }
    }
    return is_include;
}

/// Return a newly allocated string containing `input` followed by `suffix`.
pub fn duplicate_with_suffix(input: &str, suffix: &str) -> String {
    // Steps:
    // 1. Create a new `String`.
    // 2. Put `input` first and `suffix` immediately after it.
    // 3. Do not add spaces or punctuation unless they are part of `suffix`.
    let new_string = format!("{}{}", input, suffix);
    new_string
}

/// Sum the byte lengths of all string slices in `parts`.
pub fn total_byte_len(parts: &[&str]) -> usize {
    // Steps:
    // 1. Start a total at 0.
    // 2. Add `part.len()` for each string slice.
    // 3. Return the total number of bytes.
    let mut total_byte = 0;
    for x in parts {
        total_byte = total_byte + x.len();
    }
    total_byte
}

/// Return the borrowed value when present, otherwise return the borrowed default.
pub fn borrowed_or_default<'a>(value: Option<&'a str>, default: &'a str) -> &'a str {
    // Steps:
    // 1. If `value` is `Some(text)`, return `text`.
    // 2. If `value` is `None`, return `default`.
    // 3. Do not allocate a new string.
    match value {
        Some(x) => {
            return x;
        }
        None => return default,
    }
}

/// Find `key` in a slice of `(name, amount)` pairs and return the amount.
pub fn lookup_amount(pairs: &[(&str, u64)], key: &str) -> Option<u64> {
    // Steps:
    // 1. Walk through the pairs in order.
    // 2. Compare the name part with `key`.
    // 3. Return `Some(amount)` for the first exact match.
    // 4. Return `None` if the key is missing.
    let mut is_include = None;
    for x in pairs {
        if x.0 == key {
            is_include = Some(x.1);
        }
    }
    is_include
}

/// Parse an outpoint written as `txid:vout`.
///
/// Trim both fields, borrow the txid from the input, and return `None` for
/// missing separators, empty txids, or non-numeric vouts.
pub fn parse_outpoint(input: &str) -> Option<ParsedOutpoint<'_>> {
    // Steps:
    // 1. Split the input on a single `:` using the helper logic above.
    let value = None;
    if !input.contains(":") {
        return None;
    }
    let split: Vec<&str> = input.trim().split(":").collect();
    let mut m = Vec::new();
    for x in split {
        m.push(x.trim());
    }
    print!("{:?}", m);
    // 2. Reject the input if the txid side is empty after trimming.
    if m.len() < 2 || m[0] == "" {
        return value;
    }
    // 3. Parse the vout side as `u32`.
    // 4. Return `Some(ParsedOutpoint { txid, vout })` on success.
    // 5. Return `None` for any invalid input.
    let vout = m[1].parse::<u32>();
    match vout {
        Ok(x) => {
            return Some(ParsedOutpoint {
                txid: m[0],
                vout: x,
            });
        }
        Err(_) => {
            return None;
        }
    }
}
