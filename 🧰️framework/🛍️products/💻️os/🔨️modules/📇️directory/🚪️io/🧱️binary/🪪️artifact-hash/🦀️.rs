//! 🪪️ Explicit hash text admission and frontier projection.
use crate::os_directory::schema::{ArtifactHash, ArtifactFrontier, EditedArtifactFrontierV1, valid_document_open_hash};

/// 🔤️ Admits a canonical nonzero lowercase SHA256 text representation.
pub fn parse_artifact_hash_hex(value: &str) -> Option<ArtifactHash> {
    if !valid_document_open_hash(value) { return None; }
    let mut bytes = [0u8; 32];
    for (index, slot) in bytes.iter_mut().enumerate() {
        *slot = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16).ok()?;
    }
    Some(ArtifactHash(bytes))
}

/// 🔡️ Emits one typed hash as canonical hexadecimal text.
pub fn artifact_hash_hex(hash: &ArtifactHash) -> String { hex_lower(&hash.0) }

/// 🔡️ Renders canonical lowercase hexadecimal bytes for fixtures and private storage keys.
pub fn hex_lower(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    output
}

/// 🌊️ Admits the validated textual edited frontier as typed authority metadata.
pub fn decode_edited_artifact_frontier_v1(wire: &EditedArtifactFrontierV1) -> Option<ArtifactFrontier> {
    if !wire.validate() { return None; }
    Some(ArtifactFrontier {
        document_id: wire.document_id.clone(), head_edit_ordinal: wire.head_edit_ordinal,
        head_edit_id: wire.head_edit_id.clone(), last_commit_seq: wire.last_commit_seq,
        chain_hash: parse_artifact_hash_hex(&wire.chain_sha256)?,
    })
}

/// 🌊️ Emits a non-genesis authority frontier into its textual transport grammar.
pub fn encode_edited_artifact_frontier_v1(frontier: &ArtifactFrontier) -> Option<EditedArtifactFrontierV1> {
    let wire = EditedArtifactFrontierV1 {
        document_id: frontier.document_id.clone(), head_edit_ordinal: frontier.head_edit_ordinal,
        head_edit_id: frontier.head_edit_id.clone(), last_commit_seq: frontier.last_commit_seq,
        chain_sha256: artifact_hash_hex(&frontier.chain_hash),
    };
    wire.validate().then_some(wire)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
