//! ⚡️ Semio object artifact — hand-rolled `OpBinary` for `SemioObjectMutation`. `format u8` +
//! `tag u8` (variant ordinal) are two REAL fixed fields; the variant's own argument payload
//! follows as one opaque trailing `bytes` chain — reuses the already-real `../📝️text/🦀️.rs`
//! text codec's argument tail, same convention every sibling subset uses.

use crate::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;

//#region 📡️SemioProtocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 🧾️ Each record kind's text-grammar keyword, the head `decode_op` re-prefixes onto the argument tail before `parse_op`.
const TEXT_KEYWORDS: [(&str, &str); 9] = [
    ("move-object", "moveObject"),
    ("rotate-object", "rotateObject"),
    ("scale-object", "scaleObject"),
    ("create-brep", "createBrep"),
    ("delete-brep", "deleteBrep"),
    ("create-mesh", "createMesh"),
    ("delete-mesh", "deleteMesh"),
    ("create-properties", "createProperties"),
    ("delete-properties", "deleteProperties"),
];

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioObjectMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = COMPONENT_PROTOCOL_SEMIO;
const TAG_MOVE_OBJECT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "move-object");
const TAG_ROTATE_OBJECT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "rotate-object");
const TAG_SCALE_OBJECT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "scale-object");
const TAG_CREATE_BREP: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-brep");
const TAG_DELETE_BREP: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-brep");
const TAG_CREATE_MESH: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-mesh");
const TAG_DELETE_MESH: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-mesh");
const TAG_CREATE_PROPERTIES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-properties");
const TAG_DELETE_PROPERTIES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-properties");
//#endregion 🏷️WireTags

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wire_tag(m: &SemioObjectMutation) -> u8 {
    match m {
        SemioObjectMutation::MoveObject(_) => TAG_MOVE_OBJECT,
        SemioObjectMutation::RotateObject(_) => TAG_ROTATE_OBJECT,
        SemioObjectMutation::ScaleObject(_) => TAG_SCALE_OBJECT,
        SemioObjectMutation::CreateBrep(_) => TAG_CREATE_BREP,
        SemioObjectMutation::DeleteBrep(_) => TAG_DELETE_BREP,
        SemioObjectMutation::CreateMesh(_) => TAG_CREATE_MESH,
        SemioObjectMutation::DeleteMesh(_) => TAG_DELETE_MESH,
        SemioObjectMutation::CreateProperties(_) => TAG_CREATE_PROPERTIES,
        SemioObjectMutation::DeleteProperties(_) => TAG_DELETE_PROPERTIES,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_op_args(m: &SemioObjectMutation) -> String {
    use protocol::OpText;
    match m.print_op().split_once(':') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

impl protocol::OpBinary for SemioObjectMutation {
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
