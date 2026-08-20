//! ⚡️ Semio table artifact — hand-rolled `OpBinary` for `SemioTableMutation`. `format u8`
//! (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, [`OP_KEYWORDS`]) are two REAL
//! fixed fields; the variant's own argument payload follows as one opaque trailing `bytes` chain —
//! reuses the already-real, already-tested `../📝️text/🦀️component.rs` text codec (`print_op`'s
//! argument tail) rather than re-deriving a second independent encoding, mirroring `✳️text`'s own
//! established convention.

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;

//#region 📡️SemioProtocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

//#region 🔖️OpBinary
/// 🧾️ Keyword table + variant ordinal, 0-indexed in enum declaration order — the binary frame's
/// `tag` byte, `📖️grammar/component.grammar.semio`'s `op` alternatives, and this array must all
/// agree (see `committed_facet_files_parse`/`ops_grammar_conformance_law` in `🚪️io/🦀️component.rs`).
const OP_KEYWORDS: [&str; 8] = ["createColumn", "deleteColumn", "renameColumn", "reorderColumns", "insertRow", "removeRow", "reorderRows", "editCell"];

async fn variant_ordinal(m: &SemioTableMutation) -> u8 {
    match m {
        SemioTableMutation::CreateColumn(_) => 0,
        SemioTableMutation::DeleteColumn(_) => 1,
        SemioTableMutation::RenameColumn(_) => 2,
        SemioTableMutation::ReorderColumns(_) => 3,
        SemioTableMutation::InsertRow(_) => 4,
        SemioTableMutation::RemoveRow(_) => 5,
        SemioTableMutation::ReorderRows(_) => 6,
        SemioTableMutation::EditCell(_) => 7,
    }
}

/// ✂️ Just the argument tail of `print_op` — the binary frame's `tag` byte already carries the
/// keyword, so the text keyword itself (and its `:` separator) is redundant in the binary payload.
async fn print_op_args(m: &SemioTableMutation) -> String {
    use protocol::OpText;
    match m.print_op().await.split_once(':') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

impl protocol::OpBinary for SemioTableMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self).await];
        out.extend_from_slice(print_op_args(self).await.as_bytes());
        Ok(out)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
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
        Self::parse_op(&line).await.map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
//#endregion 🔖️OpBinary

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::text::demo_mutation_cases;
    use protocol::OpBinary;

    #[semio_framework_async_macros::async_test]
    async fn op_binary_roundtrip_law() {
        for m in demo_mutation_cases() {
            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioTableMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
}
//#endregion 🧪️Tests
