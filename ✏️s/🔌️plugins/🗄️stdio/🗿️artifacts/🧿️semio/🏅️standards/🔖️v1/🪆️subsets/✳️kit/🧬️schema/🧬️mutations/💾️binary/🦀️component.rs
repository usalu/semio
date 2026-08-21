//! ⚡️ Semio kit artifact — hand-rolled `OpBinary` for `SemioKitMutation`. `format u8` + `tag u8`
//! (variant ordinal) are two REAL fixed fields; the variant's own argument payload follows as one
//! opaque trailing `bytes` chain — reuses the already-real `../📝️text/🦀️component.rs` text codec's
//! argument tail, same convention every sibling subset uses.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;

//#region 📡️SemioProtocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

//#region 🔖️OpBinary
const OP_KEYWORDS: [&str; 15] = [
    "createObject",
    "deleteObject",
    "createModel",
    "deleteModel",
    "createProperties",
    "deleteProperties",
    "bindRepresentation",
    "unbindRepresentation",
    "changeRepresentationPin",
    "addType",
    "removeType",
    "renameType",
    "addDesign",
    "removeDesign",
    "editDesign",
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn variant_ordinal(m: &SemioKitMutation) -> u8 {
    match m {
        SemioKitMutation::CreateObject(_) => 0,
        SemioKitMutation::DeleteObject(_) => 1,
        SemioKitMutation::CreateModel(_) => 2,
        SemioKitMutation::DeleteModel(_) => 3,
        SemioKitMutation::CreateProperties(_) => 4,
        SemioKitMutation::DeleteProperties(_) => 5,
        SemioKitMutation::BindRepresentation(_) => 6,
        SemioKitMutation::UnbindRepresentation(_) => 7,
        SemioKitMutation::ChangeRepresentationPin(_) => 8,
        SemioKitMutation::AddType(_) => 9,
        SemioKitMutation::RemoveType(_) => 10,
        SemioKitMutation::RenameType(_) => 11,
        SemioKitMutation::AddDesign(_) => 12,
        SemioKitMutation::RemoveDesign(_) => 13,
        SemioKitMutation::EditDesign(_) => 14,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_op_args(m: &SemioKitMutation) -> String {
    use protocol::OpText;
    match m.print_op().split_once(':') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

impl protocol::OpBinary for SemioKitMutation {
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
    use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::text::demo_mutation_cases;
    use protocol::OpBinary;

    #[semio_framework_async_macros::async_test]
    async fn op_binary_roundtrip_law() {
        for m in demo_mutation_cases() {
            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioKitMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
}
//#endregion 🧪️Tests
