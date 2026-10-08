//! 💾️ Binary representation codec surface for `stdio.semio.image` (mutations) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::image::io::text::mutations::TEXT_KEYWORDS;
use crate::standards::v1::subsets::image::schema::mutations::*;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, NamedModified};
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::image::schema::diff::{SemioImageDiff, SemioImageFrameDiff, SemioImageFramesDiff, SemioImageMetadataDiff};
use crate::standards::v1::subsets::image::io::text::snapshot::{dec_metadata_entry};
use crate::standards::v1::subsets::image::io::text::snapshot::{enc_metadata_entry};
use crate::standards::v1::subsets::image::io::text::snapshot::{dec_frame};
use crate::standards::v1::subsets::image::io::text::snapshot::{enc_frame};
use crate::standards::v1::subsets::image::io::text::snapshot::{dec_colorspace};
use crate::standards::v1::subsets::image::io::text::snapshot::{enc_colorspace};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{decode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{encode_option};
use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};
use protocol::Mutation;
/// 🔧️ Unconditional — `impl protocol::OpBinary for SemioImageMutation` below calls
/// `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in scope in
/// production code (was missing entirely, even test-gated) (W2b closer fix).
use protocol::OpText;
use crate::standards::v1::subsets::image::io::text::mutations::{print_image_mutation};
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn wire_tag(m: &SemioImageMutation) -> u8 {
    match m {
        SemioImageMutation::SetDimensions(_) => TAG_SET_DIMENSIONS,
        SemioImageMutation::SetColorspace(_) => TAG_SET_COLORSPACE,
        SemioImageMutation::SetBitDepth(_) => TAG_SET_BIT_DEPTH,
        SemioImageMutation::SetIcc(_) => TAG_SET_ICC,
        SemioImageMutation::InsertFrame(_) => TAG_INSERT_FRAME,
        SemioImageMutation::RemoveFrame(_) => TAG_REMOVE_FRAME,
        SemioImageMutation::MoveFrame(_) => TAG_MOVE_FRAME,
        SemioImageMutation::SetFrameDelay(_) => TAG_SET_FRAME_DELAY,
        SemioImageMutation::SetFramePixels(_) => TAG_SET_FRAME_PIXELS,
        SemioImageMutation::SetMetadataEntry(_) => TAG_SET_METADATA_ENTRY,
        SemioImageMutation::RemoveMetadataEntry(_) => TAG_REMOVE_METADATA_ENTRY,
    }
}

/// ✂️ Just the argument tail of `print_image_mutation` — the binary frame's `tag` byte already
/// carries the keyword, so the text keyword itself (and its `:` separator) is redundant in the
/// binary payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_image_mutation_args(m: &SemioImageMutation) -> String {
    match print_image_mutation(m).split_once(':') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ Real binary op frame, replacing the old `print_op().into_bytes()` text-as-binary shortcut.
/// `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (its kind's record tag in `💾️binary/📡️.protocol.semio`) are two REAL fixed fields; the variant's own argument payload follows as one
/// opaque trailing `bytes` chain — reuses the already-real, already-tested `print_image_mutation`/
/// `parse_image_mutation` text codec rather than re-deriving a second independent encoding.
impl protocol::OpBinary for SemioImageMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, wire_tag(self)];
        out.extend_from_slice(print_image_mutation_args(self).as_bytes());
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
        let kind = dsl::protocol_record::kind(WIRE_PROTOCOL, u64::from(tag)).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} names no record of 📡️.protocol.semio") })?;
        let keyword = TEXT_KEYWORDS.iter().find(|(record, _)| *record == kind).map(|(_, keyword)| *keyword).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("record {kind} has no text keyword") })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword}:{args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioImageMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_DIMENSIONS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-dimensions");
const TAG_SET_COLORSPACE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-colorspace");
const TAG_SET_BIT_DEPTH: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-bit-depth");
const TAG_SET_ICC: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-icc");
const TAG_INSERT_FRAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-frame");
const TAG_REMOVE_FRAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-frame");
const TAG_MOVE_FRAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "move-frame");
const TAG_SET_FRAME_DELAY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-frame-delay");
const TAG_SET_FRAME_PIXELS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-frame-pixels");
const TAG_SET_METADATA_ENTRY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-metadata-entry");
const TAG_REMOVE_METADATA_ENTRY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-metadata-entry");
//#endregion 🏷️WireTags
