//! 💾️ Binary representation codec surface for `stdio.semio.presentation` (mutations).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::presentation::schema::mutations::*;
use crate::standards::v1::subsets::document::schema::snapshot::DocBlock;
use crate::standards::v1::subsets::presentation::schema::diff::{diff_insert_layout, diff_insert_master, diff_insert_shape, diff_insert_slide, diff_remove_layout, diff_remove_master, diff_remove_shape, diff_remove_slide, diff_set_layout_master, diff_set_shape_frame, diff_set_slide_layout, diff_set_slide_notes, diff_set_snapshot, diff_set_textbox_blocks, frame_of, SemioPresentationDiff};
use crate::standards::v1::subsets::presentation::io::text::diff::{dec_shape};
use crate::standards::v1::subsets::presentation::io::text::diff::{enc_shape};
use crate::standards::v1::subsets::presentation::io::text::diff::{dec_slide};
use crate::standards::v1::subsets::presentation::io::text::diff::{enc_slide};
use crate::standards::v1::subsets::presentation::io::text::diff::{dec_layout};
use crate::standards::v1::subsets::presentation::io::text::diff::{enc_layout};
use crate::standards::v1::subsets::presentation::io::text::diff::{dec_master};
use crate::standards::v1::subsets::presentation::io::text::diff::{enc_master};
use crate::standards::v1::subsets::presentation::io::text::diff::{dec_frame};
use crate::standards::v1::subsets::presentation::io::text::diff::{enc_frame};
use crate::standards::v1::subsets::document::io::text::diff::{dec_block};
use crate::standards::v1::subsets::document::io::text::diff::{enc_block};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{decode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{encode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot, Slide, SlideFrame, SlideLayout, SlideMaster, SlideShape};
/// 🔧️ `OpBinary`/`OpText` both unconditional (not `#[cfg(test)]`-gated): the real
/// `impl protocol::OpBinary for SemioPresentationMutation` below (production code) calls
/// `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs both traits in scope.
use protocol::{Mutation, OpBinary, OpText};
use crate::standards::v1::subsets::presentation::io::text::mutations::{print_presentation_mutation};
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn wire_tag(m: &SemioPresentationMutation) -> u8 {
    match m {
        SemioPresentationMutation::SetSnapshot(_) => TAG_SET_SNAPSHOT,
        SemioPresentationMutation::PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
        SemioPresentationMutation::InsertSlide(_) => TAG_INSERT_SLIDE,
        SemioPresentationMutation::RemoveSlide(_) => TAG_REMOVE_SLIDE,
        SemioPresentationMutation::SetSlideLayout(_) => TAG_SET_SLIDE_LAYOUT,
        SemioPresentationMutation::SetSlideNotes(_) => TAG_SET_SLIDE_NOTES,
        SemioPresentationMutation::InsertShape(_) => TAG_INSERT_SHAPE,
        SemioPresentationMutation::RemoveShape(_) => TAG_REMOVE_SHAPE,
        SemioPresentationMutation::SetShapeFrame(_) => TAG_SET_SHAPE_FRAME,
        SemioPresentationMutation::SetTextBoxBlocks(_) => TAG_SET_TEXT_BOX_BLOCKS,
        SemioPresentationMutation::InsertMaster(_) => TAG_INSERT_MASTER,
        SemioPresentationMutation::RemoveMaster(_) => TAG_REMOVE_MASTER,
        SemioPresentationMutation::InsertLayout(_) => TAG_INSERT_LAYOUT,
        SemioPresentationMutation::RemoveLayout(_) => TAG_REMOVE_LAYOUT,
        SemioPresentationMutation::SetLayoutMaster(_) => TAG_SET_LAYOUT_MASTER,
    }
}

/// ✂️ Just the `key=value ...` argument tail of `print_presentation_mutation` — the binary frame's
/// `tag` byte already carries the keyword, so the text keyword itself is redundant in the binary
/// payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_presentation_mutation_args(m: &SemioPresentationMutation) -> String {
    match print_presentation_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION presentation wave: real binary
/// op frame, replacing the old `print_op().into_bytes()` text-as-binary shortcut. `format u8`
/// (`OP_BINARY_FORMAT` convention) + `tag u8` (its kind's record tag in `💾️binary/📡️.protocol.semio`) are two
/// REAL fixed fields; the variant's own `key=value ...` argument payload follows as one opaque
/// trailing `bytes` chain — reusing the already-real, already-tested
/// `print_presentation_mutation`/`parse_presentation_mutation` text codec rather than re-deriving a
/// second independent encoding (`protocol-array-of-records`/`protocol-prim-ref-recursion`, per the
/// grammar recipe's own gap table — same honest boundary the sibling `../../🔺️diff/💾️binary/
/// 📡️.protocol.semio` uses).
impl OpBinary for SemioPresentationMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        if let Self::PatchSnapshot(payload) = self {
            let mut out = vec![1, TAG_PATCH_SNAPSHOT];
            out.extend(protocol::OpBinary::encode_op(&payload.patch)?);
            return Ok(out);
        }
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, wire_tag(self)];
        out.extend_from_slice(print_presentation_mutation_args(self).as_bytes());
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
        if bytes[1] == TAG_PATCH_SNAPSHOT {
            return Ok(Self::PatchSnapshot(crate::standards::v1::subsets::presentation::schema::mutations::patch_snapshot::PatchSnapshot { patch: protocol::OpBinary::decode_op(&bytes[2..])? }));
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
/// 🏷️ Op tags of `SemioPresentationMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_PATCH_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "patch-snapshot");
const TAG_INSERT_SLIDE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-slide");
const TAG_REMOVE_SLIDE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-slide");
const TAG_SET_SLIDE_LAYOUT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-slide-layout");
const TAG_SET_SLIDE_NOTES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-slide-notes");
const TAG_INSERT_SHAPE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-shape");
const TAG_REMOVE_SHAPE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-shape");
const TAG_SET_SHAPE_FRAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-shape-frame");
const TAG_SET_TEXT_BOX_BLOCKS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-text-box-blocks");
const TAG_INSERT_MASTER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-master");
const TAG_REMOVE_MASTER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-master");
const TAG_INSERT_LAYOUT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-layout");
const TAG_REMOVE_LAYOUT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-layout");
const TAG_SET_LAYOUT_MASTER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-layout-master");
//#endregion 🏷️WireTags
