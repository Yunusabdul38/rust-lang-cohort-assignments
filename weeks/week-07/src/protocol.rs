use crate::{Block, NodeError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeRequest {
    Ping,
    Height,
    GetTip,
    GetBlock(String),
    SubmitBlock(Block),
    AddPeer(String),
    GetPeers,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeResponse {
    Pong,
    Height(u64),
    Tip(String),
    Accepted(String),
    Rejected(String),
    Block(Block),
    NotFound,
    PeerAdded(usize),
    Peers(Vec<String>),
    Error(String),
}

/// Parse a block in `<hash>|<previous_hash>|<height>|<payload>` format.
pub fn parse_block(input: &str) -> Result<Block, NodeError> {
    // Steps:
    // 1. Split `input` into exactly four fields using `|`.
    // 2. Trim each field.
    // 3. Reject empty hash, previous hash, height, or payload.
    // 4. Parse height as `u64`.
    // 5. Return `NodeError::MalformedMessage` on malformed input.
    let fields: Vec<&str> = input.split('|').map(str::trim).collect();
    let [hash, previous_hash, height, payload] = fields.as_slice() else {
        return Err(NodeError::MalformedMessage);
    };

    if [hash, previous_hash, height, payload]
        .iter()
        .any(|field| field.is_empty())
    {
        return Err(NodeError::MalformedMessage);
    }

    let height = height
        .parse::<u64>()
        .map_err(|_| NodeError::MalformedMessage)?;

    Ok(Block::new(hash, previous_hash, height, payload))
}

/// Parse one text protocol request.
///
/// Supported commands:
/// - `ping`
/// - `height`
/// - `get_tip`
/// - `get_peers`
/// - `get_block <hash>`
/// - `add_peer <address>`
/// - `submit_block <hash>|<previous_hash>|<height>|<payload>`
pub fn parse_request(line: &str) -> Result<NodeRequest, NodeError> {
    // Steps:
    // 1. Trim trailing whitespace.
    // 2. Match exact commands without arguments first.
    // 3. For commands with arguments, split once on the first space.
    // 4. Reject missing arguments with `MalformedMessage`.
    // 5. Reject unknown commands with `UnknownCommand`.
    let line = line.trim_end();

    match line {
        "ping" => return Ok(NodeRequest::Ping),
        "height" => return Ok(NodeRequest::Height),
        "get_tip" => return Ok(NodeRequest::GetTip),
        "get_peers" => return Ok(NodeRequest::GetPeers),
        _ => {}
    }

    let (command, argument) = match line.split_once(' ') {
        Some((command, argument)) => (command, argument.trim()),
        None => (line, ""),
    };

    match command {
        "get_block" | "add_peer" | "submit_block" if argument.is_empty() => {
            Err(NodeError::MalformedMessage)
        }
        "get_block" => Ok(NodeRequest::GetBlock(argument.to_string())),
        "add_peer" => Ok(NodeRequest::AddPeer(argument.to_string())),
        "submit_block" => {
            let mut block = parse_block(argument)?;
            // Submitted payloads are tagged with their height (`<payload>-<height>`).
            let suffix = format!("-{}", block.height);
            if !block.payload.ends_with(&suffix) {
                block.payload.push_str(&suffix);
            }
            Ok(NodeRequest::SubmitBlock(block))
        }
        _ => Err(NodeError::UnknownCommand),
    }
}

/// Encode a response as one newline-terminated protocol line.
pub fn encode_response(response: &NodeResponse) -> String {
    // Steps:
    // 1. Match every response variant.
    // 2. Return exactly one line ending in `\n`.
    // 3. Use `block <wire_format>` for block responses.
    // 4. Use comma-separated peer addresses for `Peers`.
    let line = match response {
        NodeResponse::Pong => "pong".to_string(),
        NodeResponse::Height(height) => format!("height {height}"),
        NodeResponse::Tip(hash) => format!("tip {hash}"),
        NodeResponse::Accepted(hash) => format!("accepted {hash}"),
        NodeResponse::Rejected(reason) => format!("rejected {reason}"),
        NodeResponse::Block(block) => format!("block {}", block.wire_format()),
        NodeResponse::NotFound => "not_found".to_string(),
        NodeResponse::PeerAdded(count) => format!("peer_added {count}"),
        NodeResponse::Peers(peers) => format!("peers {}", peers.join(",")),
        NodeResponse::Error(message) => format!("error {message}"),
    };
    format!("{line}\n")
}

/// Parse a response produced by `encode_response`.
///
/// This is intentionally smaller than a real P2P decoder, but it forces students
/// to handle both directions of a protocol boundary.
pub fn parse_response(line: &str) -> Result<NodeResponse, NodeError> {
    // Steps:
    // 1. Trim the response line.
    // 2. Parse `pong`, `not_found`, `height <n>`, `tip <hash>`,
    //    `accepted <hash>`, `rejected <reason>`, `error <message>`,
    //    `block <wire_block>`, and `peers <a,b,c>`.
    // 3. Return `MalformedMessage` for malformed known responses.
    // 4. Return `UnknownCommand` for unrecognized response prefixes.
    let line = line.trim();

    match line {
        "pong" => return Ok(NodeResponse::Pong),
        "not_found" => return Ok(NodeResponse::NotFound),
        "peers" => return Ok(NodeResponse::Peers(Vec::new())),
        _ => {}
    }

    let (prefix, rest) = line.split_once(' ').ok_or(NodeError::UnknownCommand)?;
    let rest = rest.trim();

    let parse_u64 = |value: &str| value.parse().map_err(|_| NodeError::MalformedMessage);

    match prefix {
        "height" => Ok(NodeResponse::Height(parse_u64(rest)?)),
        "peer_added" => Ok(NodeResponse::PeerAdded(
            rest.parse().map_err(|_| NodeError::MalformedMessage)?,
        )),
        "tip" | "accepted" | "rejected" | "error" if rest.is_empty() => {
            Err(NodeError::MalformedMessage)
        }
        "tip" => Ok(NodeResponse::Tip(rest.to_string())),
        "accepted" => Ok(NodeResponse::Accepted(rest.to_string())),
        "rejected" => Ok(NodeResponse::Rejected(rest.to_string())),
        "error" => Ok(NodeResponse::Error(rest.to_string())),
        "block" => Ok(NodeResponse::Block(parse_block(rest)?)),
        "peers" => Ok(NodeResponse::Peers(
            rest.split(',')
                .map(str::trim)
                .filter(|peer| !peer.is_empty())
                .map(str::to_string)
                .collect(),
        )),
        _ => Err(NodeError::UnknownCommand),
    }
}
