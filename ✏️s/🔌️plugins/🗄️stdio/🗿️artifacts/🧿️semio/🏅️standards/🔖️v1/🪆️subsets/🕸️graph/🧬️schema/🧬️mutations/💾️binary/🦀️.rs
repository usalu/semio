//! ⚡️ Semio graph artifact — hand-rolled `OpBinary` for `SemioGraphMutation`. `format u8`
//! (`OP_BINARY_FORMAT` convention) + `tag u8` (its kind's record tag in `💾️binary/📡️.protocol.semio`) are two REAL
//! fixed fields; the variant's own argument payload follows as one opaque trailing `bytes` chain —
//! reuses the already-real, already-tested `../📝️text/🦀️.rs` text codec (`print_op`'s
//! argument tail) rather than re-deriving a second independent encoding, mirroring `🔤️text`'s own
//! established convention.

use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;

//#region 📡️SemioProtocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 🧾️ Each record kind's text-grammar keyword, the head `decode_op` re-prefixes onto the argument tail before `parse_op`.
const TEXT_KEYWORDS: [(&str, &str); 11] = [
    ("create-node", "createNode"),
    ("delete-node", "deleteNode"),
    ("change-node-kind", "changeNodeKind"),
    ("change-node-label", "changeNodeLabel"),
    ("move-node", "moveNode"),
    ("add-node-port", "addNodePort"),
    ("remove-node-port", "removeNodePort"),
    ("add-node-property", "addNodeProperty"),
    ("remove-node-property", "removeNodeProperty"),
    ("create-edge", "createEdge"),
    ("delete-edge", "deleteEdge"),
];

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioGraphMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = COMPONENT_PROTOCOL_SEMIO;
const TAG_CREATE_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-node");
const TAG_DELETE_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-node");
const TAG_CHANGE_NODE_KIND: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-node-kind");
const TAG_CHANGE_NODE_LABEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-node-label");
const TAG_MOVE_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "move-node");
const TAG_ADD_NODE_PORT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-node-port");
const TAG_REMOVE_NODE_PORT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-node-port");
const TAG_ADD_NODE_PROPERTY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-node-property");
const TAG_REMOVE_NODE_PROPERTY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-node-property");
const TAG_CREATE_EDGE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-edge");
const TAG_DELETE_EDGE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-edge");
//#endregion 🏷️WireTags

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wire_tag(m: &SemioGraphMutation) -> u8 {
    match m {
        SemioGraphMutation::CreateNode(_) => TAG_CREATE_NODE,
        SemioGraphMutation::DeleteNode(_) => TAG_DELETE_NODE,
        SemioGraphMutation::ChangeNodeKind(_) => TAG_CHANGE_NODE_KIND,
        SemioGraphMutation::ChangeNodeLabel(_) => TAG_CHANGE_NODE_LABEL,
        SemioGraphMutation::MoveNode(_) => TAG_MOVE_NODE,
        SemioGraphMutation::AddNodePort(_) => TAG_ADD_NODE_PORT,
        SemioGraphMutation::RemoveNodePort(_) => TAG_REMOVE_NODE_PORT,
        SemioGraphMutation::AddNodeProperty(_) => TAG_ADD_NODE_PROPERTY,
        SemioGraphMutation::RemoveNodeProperty(_) => TAG_REMOVE_NODE_PROPERTY,
        SemioGraphMutation::CreateEdge(_) => TAG_CREATE_EDGE,
        SemioGraphMutation::DeleteEdge(_) => TAG_DELETE_EDGE,
    }
}

/// ✂️ Just the argument tail of `print_op` — the binary frame's `tag` byte already carries the
/// keyword, so the text keyword itself (and its `:` separator) is redundant in the binary payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
        let mut out = vec![OP_BINARY_FORMAT, wire_tag(self)];
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
        let kind = dsl::protocol_record::kind(WIRE_PROTOCOL, u64::from(tag)).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} names no record of 📡️.protocol.semio") })?;
        let keyword = TEXT_KEYWORDS.iter().find(|(record, _)| *record == kind).map(|(_, keyword)| *keyword).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("record {kind} has no text keyword") })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword}:{args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
//#endregion 🔖️OpBinary

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
