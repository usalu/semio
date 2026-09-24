//! ⚡️ Semio kit artifact — hand-rolled `OpBinary` for `SemioKitMutation`. `format u8` + `tag u8`
//! (variant ordinal) are two REAL fixed fields; the variant's own argument payload follows as one
//! opaque trailing `bytes` chain — reuses the already-real `../📝️text/🦀️.rs` text codec's
//! argument tail, same convention every sibling subset uses.

use crate::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;

//#region 📡️SemioProtocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 🧾️ Each record kind's text-grammar keyword, the head `decode_op` re-prefixes onto the argument tail before `parse_op`.
const TEXT_KEYWORDS: [(&str, &str); 15] = [
    ("create-object", "createObject"),
    ("delete-object", "deleteObject"),
    ("create-model", "createModel"),
    ("delete-model", "deleteModel"),
    ("create-properties", "createProperties"),
    ("delete-properties", "deleteProperties"),
    ("bind-representation", "bindRepresentation"),
    ("unbind-representation", "unbindRepresentation"),
    ("change-representation-pin", "changeRepresentationPin"),
    ("add-type", "addType"),
    ("remove-type", "removeType"),
    ("rename-type", "renameType"),
    ("add-design", "addDesign"),
    ("remove-design", "removeDesign"),
    ("edit-design", "editDesign"),
];

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioKitMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = COMPONENT_PROTOCOL_SEMIO;
const TAG_CREATE_OBJECT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-object");
const TAG_DELETE_OBJECT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-object");
const TAG_CREATE_MODEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-model");
const TAG_DELETE_MODEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-model");
const TAG_CREATE_PROPERTIES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-properties");
const TAG_DELETE_PROPERTIES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-properties");
const TAG_BIND_REPRESENTATION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "bind-representation");
const TAG_UNBIND_REPRESENTATION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "unbind-representation");
const TAG_CHANGE_REPRESENTATION_PIN: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-representation-pin");
const TAG_ADD_TYPE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-type");
const TAG_REMOVE_TYPE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-type");
const TAG_RENAME_TYPE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "rename-type");
const TAG_ADD_DESIGN: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-design");
const TAG_REMOVE_DESIGN: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-design");
const TAG_EDIT_DESIGN: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "edit-design");
//#endregion 🏷️WireTags

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wire_tag(m: &SemioKitMutation) -> u8 {
    match m {
        SemioKitMutation::CreateObject(_) => TAG_CREATE_OBJECT,
        SemioKitMutation::DeleteObject(_) => TAG_DELETE_OBJECT,
        SemioKitMutation::CreateModel(_) => TAG_CREATE_MODEL,
        SemioKitMutation::DeleteModel(_) => TAG_DELETE_MODEL,
        SemioKitMutation::CreateProperties(_) => TAG_CREATE_PROPERTIES,
        SemioKitMutation::DeleteProperties(_) => TAG_DELETE_PROPERTIES,
        SemioKitMutation::BindRepresentation(_) => TAG_BIND_REPRESENTATION,
        SemioKitMutation::UnbindRepresentation(_) => TAG_UNBIND_REPRESENTATION,
        SemioKitMutation::ChangeRepresentationPin(_) => TAG_CHANGE_REPRESENTATION_PIN,
        SemioKitMutation::AddType(_) => TAG_ADD_TYPE,
        SemioKitMutation::RemoveType(_) => TAG_REMOVE_TYPE,
        SemioKitMutation::RenameType(_) => TAG_RENAME_TYPE,
        SemioKitMutation::AddDesign(_) => TAG_ADD_DESIGN,
        SemioKitMutation::RemoveDesign(_) => TAG_REMOVE_DESIGN,
        SemioKitMutation::EditDesign(_) => TAG_EDIT_DESIGN,
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
