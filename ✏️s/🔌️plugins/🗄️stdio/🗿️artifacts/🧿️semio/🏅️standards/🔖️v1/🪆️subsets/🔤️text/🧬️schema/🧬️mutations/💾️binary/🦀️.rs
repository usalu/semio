//! ⚡️ Semio text artifact — hand-rolled `OpBinary` for `SemioTextMutation`. `format u8`
//! (`OP_BINARY_FORMAT` convention) + `tag u8` (its kind's record tag in `💾️binary/📡️.protocol.semio`) are two REAL
//! fixed fields; the variant's own argument payload follows as one opaque trailing `bytes` chain —
//! reuses the already-real, already-tested `../📝️text/🦀️.rs` text codec (`print_op`'s
//! argument tail) rather than re-deriving a second independent encoding, mirroring `🖼️image`'s own
//! established convention.

use crate::standards::v1::subsets::text::schema::mutations::SemioTextMutation;

//#region 📡️SemioProtocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 🧾️ Each record kind's text-grammar keyword, the head `decode_op` re-prefixes onto the argument tail before `parse_op`.
const TEXT_KEYWORDS: [(&str, &str); 7] = [
    ("insert-run", "insertRun"),
    ("remove-run", "removeRun"),
    ("edit-run", "editRun"),
    ("change-run-language", "changeRunLanguage"),
    ("reorder-runs", "reorderRuns"),
    ("add-mark", "addMark"),
    ("remove-mark", "removeMark"),
];

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioTextMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = COMPONENT_PROTOCOL_SEMIO;
const TAG_INSERT_RUN: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-run");
const TAG_REMOVE_RUN: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-run");
const TAG_EDIT_RUN: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "edit-run");
const TAG_CHANGE_RUN_LANGUAGE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-run-language");
const TAG_REORDER_RUNS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "reorder-runs");
const TAG_ADD_MARK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-mark");
const TAG_REMOVE_MARK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-mark");
//#endregion 🏷️WireTags

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wire_tag(m: &SemioTextMutation) -> u8 {
    match m {
        SemioTextMutation::InsertRun(_) => TAG_INSERT_RUN,
        SemioTextMutation::RemoveRun(_) => TAG_REMOVE_RUN,
        SemioTextMutation::EditRun(_) => TAG_EDIT_RUN,
        SemioTextMutation::ChangeRunLanguage(_) => TAG_CHANGE_RUN_LANGUAGE,
        SemioTextMutation::ReorderRuns(_) => TAG_REORDER_RUNS,
        SemioTextMutation::AddMark(_) => TAG_ADD_MARK,
        SemioTextMutation::RemoveMark(_) => TAG_REMOVE_MARK,
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
