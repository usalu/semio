//! 💾️ Binary representation codec surface for `s.stdio.semio.document.mutations` — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::document::schema::mutations::*;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::document::schema::diff::{BlocksDiff, DocBlockDiff, DocHeadingDiff, DocParagraphDiff, DocQuoteDiff, DocRunDiff, DocTableCellDiff, DocTableRowDiff, ListItemsDiff, RunsDiff, SemioDocumentDiff, TableCellsDiff, TableRowsDiff};
use crate::standards::v1::subsets::document::io::text::diff::{dec_run_style};
use crate::standards::v1::subsets::document::io::text::diff::{enc_run_style};
use crate::standards::v1::subsets::document::io::text::diff::{dec_u8};
use crate::standards::v1::subsets::document::io::text::diff::{enc_u8};
use crate::standards::v1::subsets::document::io::text::diff::{dec_image};
use crate::standards::v1::subsets::document::io::text::diff::{enc_image};
use crate::standards::v1::subsets::video::io::text::snapshot::{dec_bool};
use crate::standards::v1::subsets::document::io::text::diff::{dec_style};
use crate::standards::v1::subsets::document::io::text::diff::{enc_style};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_bool};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{decode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{encode_option};
use crate::standards::v1::subsets::flow::io::text::snapshot::{dec_f64};
use crate::standards::v1::subsets::flow::io::text::snapshot::{enc_f64};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{hex_decode};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{hex_encode};
use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocRun, DocStyle, RunStyle, SemioDocumentSnapshot};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioDocumentMutation` block
/// below calls `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in
/// scope in production code too, not merely under `#[cfg(test)]` (W2b closer fix).
use protocol::{OpBinary, OpText};
use crate::standards::v1::subsets::document::io::text::mutations::{print_document_mutation};
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn wire_tag(m: &SemioDocumentMutation) -> u8 {
    match m {
        SemioDocumentMutation::InsertBlock(..) => TAG_INSERT_BLOCK,
        SemioDocumentMutation::RemoveBlock(..) => TAG_REMOVE_BLOCK,
        SemioDocumentMutation::SetBlockContent(..) => TAG_SET_BLOCK_CONTENT,
        SemioDocumentMutation::SetParagraphStyle(..) => TAG_SET_PARAGRAPH_STYLE,
        SemioDocumentMutation::SetHeadingLevel(..) => TAG_SET_HEADING_LEVEL,
        SemioDocumentMutation::SetListOrdered(..) => TAG_SET_LIST_ORDERED,
        SemioDocumentMutation::SetRunText(..) => TAG_SET_RUN_TEXT,
        SemioDocumentMutation::SetRunStyle(..) => TAG_SET_RUN_STYLE,
        SemioDocumentMutation::SetImageBlock(..) => TAG_SET_IMAGE_BLOCK,
        SemioDocumentMutation::InsertStyle(..) => TAG_INSERT_STYLE,
        SemioDocumentMutation::RemoveStyle(..) => TAG_REMOVE_STYLE,
        SemioDocumentMutation::SetStyleName(..) => TAG_SET_STYLE_NAME,
        SemioDocumentMutation::SetStyleBasedOn(..) => TAG_SET_STYLE_BASED_ON,
        SemioDocumentMutation::InsertImage(..) => TAG_INSERT_IMAGE,
        SemioDocumentMutation::RemoveImage(..) => TAG_REMOVE_IMAGE,
        SemioDocumentMutation::SetImageBytes(..) => TAG_SET_IMAGE_BYTES,
    }
}

/// ✂️ Just the `key=value ...` argument tail of `print_document_mutation` (empty for
/// `no-mutation`) — the binary frame's `tag` byte already carries the keyword, so the text keyword
/// itself is redundant in the binary payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_document_mutation_args(m: &SemioDocumentMutation) -> String {
    match print_document_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION document wave: real binary op
/// frame, replacing the old `print_op().into_bytes()` text-as-binary shortcut. `format u8`
/// (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, see [`KINDS`]) are two
/// REAL fixed fields; the variant's own `key=value ...` argument payload follows as one opaque
/// trailing `bytes` chain — reusing the already-real, already-tested
/// `print_document_mutation`/`parse_document_mutation` text codec rather than re-deriving a second
/// independent encoding.
impl OpBinary for SemioDocumentMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, wire_tag(self)];
        out.extend_from_slice(print_document_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = dsl::protocol_record::kind(WIRE_PROTOCOL, u64::from(tag)).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} names no record of 📡️.protocol.semio") })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioDocumentMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_INSERT_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-block");
const TAG_REMOVE_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-block");
const TAG_SET_BLOCK_CONTENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-block-content");
const TAG_SET_PARAGRAPH_STYLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-paragraph-style");
const TAG_SET_HEADING_LEVEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-heading-level");
const TAG_SET_LIST_ORDERED: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-list-ordered");
const TAG_SET_RUN_TEXT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-run-text");
const TAG_SET_RUN_STYLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-run-style");
const TAG_SET_IMAGE_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-image-block");
const TAG_INSERT_STYLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-style");
const TAG_REMOVE_STYLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-style");
const TAG_SET_STYLE_NAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-style-name");
const TAG_SET_STYLE_BASED_ON: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-style-based-on");
const TAG_INSERT_IMAGE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-image");
const TAG_REMOVE_IMAGE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-image");
const TAG_SET_IMAGE_BYTES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-image-bytes");
//#endregion 🏷️WireTags
