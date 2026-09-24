//! ⚡️ Semio drawing artifact — hand-rolled `OpBinary` for `SemioDrawingMutation`. `format u8`
//! (`OP_BINARY_FORMAT` convention) + `tag u8` (its kind's record tag in `💾️binary/📡️.protocol.semio`) are two REAL
//! fixed fields; the variant's own argument payload follows as one opaque trailing `bytes` chain —
//! reuses the already-real, already-tested `../📝️text/🦀️.rs` text codec (`print_op`'s
//! argument tail) rather than re-deriving a second independent encoding, mirroring `🔤️text`'s own
//! established convention.

use crate::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;

//#region 📡️SemioProtocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 🧾️ Each record kind's text-grammar keyword, the head `decode_op` re-prefixes onto the argument tail before `parse_op`.
const TEXT_KEYWORDS: [(&str, &str); 17] = [
    ("create-layer", "createLayer"),
    ("delete-layer", "deleteLayer"),
    ("create-node", "createNode"),
    ("delete-node", "deleteNode"),
    ("move-node", "moveNode"),
    ("drag-nodes", "dragNodes"),
    ("rotate-node", "rotate"),
    ("scale-node", "scale"),
    ("reorder-nodes", "reorderNodes"),
    ("group-nodes", "group"),
    ("ungroup-node", "ungroup"),
    ("flatten-node", "flatten"),
    ("unflatten-node", "unflatten"),
    ("replace-path", "replacePath"),
    ("replace-fill", "replaceFill"),
    ("change-stroke-color", "changeStrokeColor"),
    ("change-stroke-width", "changeStrokeWidth"),
];

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioDrawingMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = COMPONENT_PROTOCOL_SEMIO;
const TAG_CREATE_LAYER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-layer");
const TAG_DELETE_LAYER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-layer");
const TAG_CREATE_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-node");
const TAG_DELETE_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-node");
const TAG_MOVE_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "move-node");
const TAG_DRAG_NODES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "drag-nodes");
const TAG_ROTATE_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "rotate-node");
const TAG_SCALE_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "scale-node");
const TAG_REORDER_NODES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "reorder-nodes");
const TAG_GROUP_NODES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "group-nodes");
const TAG_UNGROUP_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "ungroup-node");
const TAG_FLATTEN_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "flatten-node");
const TAG_UNFLATTEN_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "unflatten-node");
const TAG_REPLACE_PATH: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "replace-path");
const TAG_REPLACE_FILL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "replace-fill");
const TAG_CHANGE_STROKE_COLOR: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-stroke-color");
const TAG_CHANGE_STROKE_WIDTH: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-stroke-width");
//#endregion 🏷️WireTags

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wire_tag(m: &SemioDrawingMutation) -> u8 {
    match m {
        SemioDrawingMutation::CreateLayer(_) => TAG_CREATE_LAYER,
        SemioDrawingMutation::DeleteLayer(_) => TAG_DELETE_LAYER,
        SemioDrawingMutation::CreateNode(_) => TAG_CREATE_NODE,
        SemioDrawingMutation::DeleteNode(_) => TAG_DELETE_NODE,
        SemioDrawingMutation::MoveNode(_) => TAG_MOVE_NODE,
        SemioDrawingMutation::DragNodes(_) => TAG_DRAG_NODES,
        SemioDrawingMutation::RotateNode(_) => TAG_ROTATE_NODE,
        SemioDrawingMutation::ScaleNode(_) => TAG_SCALE_NODE,
        SemioDrawingMutation::ReorderNodes(_) => TAG_REORDER_NODES,
        SemioDrawingMutation::GroupNodes(_) => TAG_GROUP_NODES,
        SemioDrawingMutation::UngroupNode(_) => TAG_UNGROUP_NODE,
        SemioDrawingMutation::FlattenNode(_) => TAG_FLATTEN_NODE,
        SemioDrawingMutation::UnflattenNode(_) => TAG_UNFLATTEN_NODE,
        SemioDrawingMutation::ReplacePath(_) => TAG_REPLACE_PATH,
        SemioDrawingMutation::ReplaceFill(_) => TAG_REPLACE_FILL,
        SemioDrawingMutation::ChangeStrokeColor(_) => TAG_CHANGE_STROKE_COLOR,
        SemioDrawingMutation::ChangeStrokeWidth(_) => TAG_CHANGE_STROKE_WIDTH,
    }
}

/// ✂️ Just the argument tail of `print_op` — the binary frame's `tag` byte already carries the
/// keyword, so the text keyword itself (and its `:` separator) is redundant in the binary payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_op_args(m: &SemioDrawingMutation) -> String {
    use protocol::OpText;
    match m.print_op().split_once(':') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

impl protocol::OpBinary for SemioDrawingMutation {
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
