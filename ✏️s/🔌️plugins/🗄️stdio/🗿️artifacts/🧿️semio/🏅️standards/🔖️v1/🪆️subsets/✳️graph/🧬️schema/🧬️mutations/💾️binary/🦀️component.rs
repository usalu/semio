//! ⚡️ Semio graph artifact — hand-rolled `OpBinary` for `SemioGraphMutation`. `format u8`
//! (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, [`OP_KEYWORDS`]) are two REAL
//! fixed fields; the variant's own argument payload follows as one opaque trailing `bytes` chain —
//! reuses the already-real, already-tested `../📝️text/🦀️component.rs` text codec (`print_op`'s
//! argument tail) rather than re-deriving a second independent encoding, mirroring `✳️text`'s own
//! established convention.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;

//#region 📡️SemioProtocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

//#region 🔖️OpBinary
/// 🧾️ Keyword table + variant ordinal, 0-indexed in enum declaration order — the binary frame's
/// `tag` byte, `📖️grammar/component.grammar.semio`'s `op` alternatives, and this array must all
/// agree (see `committed_facet_files_parse`/`ops_grammar_conformance_law` in `🚪️io/🦀️component.rs`).
const OP_KEYWORDS: [&str; 11] = [
    "createNode", "deleteNode", "changeNodeKind", "changeNodeLabel", "moveNode",
    "addNodePort", "removeNodePort", "addNodeProperty", "removeNodeProperty",
    "createEdge", "deleteEdge",
];

fn variant_ordinal(m: &SemioGraphMutation) -> u8 {
    match m {
        SemioGraphMutation::CreateNode(_) => 0,
        SemioGraphMutation::DeleteNode(_) => 1,
        SemioGraphMutation::ChangeNodeKind(_) => 2,
        SemioGraphMutation::ChangeNodeLabel(_) => 3,
        SemioGraphMutation::MoveNode(_) => 4,
        SemioGraphMutation::AddNodePort(_) => 5,
        SemioGraphMutation::RemoveNodePort(_) => 6,
        SemioGraphMutation::AddNodeProperty(_) => 7,
        SemioGraphMutation::RemoveNodeProperty(_) => 8,
        SemioGraphMutation::CreateEdge(_) => 9,
        SemioGraphMutation::DeleteEdge(_) => 10,
    }
}

/// ✂️ Just the argument tail of `print_op` — the binary frame's `tag` byte already carries the
/// keyword, so the text keyword itself (and its `:` separator) is redundant in the binary payload.
fn print_op_args(m: &SemioGraphMutation) -> String {
    use protocol::OpText;
    match m.print_op().split_once(':') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

impl protocol::OpBinary for SemioGraphMutation {
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
    use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::text::demo_mutation_cases;
    use protocol::OpBinary;

    #[test]
    fn op_binary_roundtrip_law() {
        for m in demo_mutation_cases() {
            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioGraphMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
}
//#endregion 🧪️Tests
