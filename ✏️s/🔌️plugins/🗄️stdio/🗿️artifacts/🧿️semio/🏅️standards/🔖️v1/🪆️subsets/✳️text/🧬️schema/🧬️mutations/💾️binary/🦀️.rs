//! ⚡️ Semio text artifact — hand-rolled `OpBinary` for `SemioTextMutation`. `format u8`
//! (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, [`OP_KEYWORDS`]) are two REAL
//! fixed fields; the variant's own argument payload follows as one opaque trailing `bytes` chain —
//! reuses the already-real, already-tested `../📝️text/🦀️.rs` text codec (`print_op`'s
//! argument tail) rather than re-deriving a second independent encoding, mirroring `✳️image`'s own
//! established convention.

use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;

//#region 📡️SemioProtocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

//#region 🔖️OpBinary
/// 🧾️ Keyword table + variant ordinal, 0-indexed in enum declaration order — the binary frame's
/// `tag` byte, `📖️grammar/component.grammar.semio`'s `op` alternatives, and this array must all
/// agree (see `committed_facet_files_parse`/`ops_grammar_conformance_law` in `🚪️io/🦀️.rs`).
const OP_KEYWORDS: [&str; 7] = ["insertRun", "removeRun", "editRun", "changeRunLanguage", "reorderRuns", "addMark", "removeMark"];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn variant_ordinal(m: &SemioTextMutation) -> u8 {
    match m {
        SemioTextMutation::InsertRun(_) => 0,
        SemioTextMutation::RemoveRun(_) => 1,
        SemioTextMutation::EditRun(_) => 2,
        SemioTextMutation::ChangeRunLanguage(_) => 3,
        SemioTextMutation::ReorderRuns(_) => 4,
        SemioTextMutation::AddMark(_) => 5,
        SemioTextMutation::RemoveMark(_) => 6,
    }
}

/// ✂️ Just the argument tail of `print_op` — the binary frame's `tag` byte already carries the
/// keyword, so the text keyword itself (and its `:` separator) is redundant in the binary payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_op_args(m: &SemioTextMutation) -> String {
    use protocol::OpText;
    match m.print_op().split_once(':') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

impl protocol::OpBinary for SemioTextMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self)];
        out.extend_from_slice(print_op_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        use protocol::OpText;
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = OP_KEYWORDS.get(tag as usize).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} out of range for {} declared variants", OP_KEYWORDS.len()) })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword}:{args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
//#endregion 🔖️OpBinary

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::text::demo_mutation_cases;
    use protocol::OpBinary;

    #[semio_framework_async_macros::async_test]
    async fn op_binary_roundtrip_law() {
        for m in demo_mutation_cases() {
            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioTextMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
}
//#endregion 🧪️Tests
