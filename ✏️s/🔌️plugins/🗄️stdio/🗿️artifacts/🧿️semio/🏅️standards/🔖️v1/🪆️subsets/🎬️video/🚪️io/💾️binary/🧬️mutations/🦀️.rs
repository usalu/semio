//! 💾️ Binary representation grammar surface for `stdio.semio.video` (mutations): real binary op
//! frame — `format u8` + `tag u8` (the `SemioVideoMutation` variant ordinal) real and fully
//! described, the variant's own argument text an opaque trailing payload (video wave, replacing
//! the old `print_op().into_bytes()` text-as-binary shortcut) — actual encode/decode lives on
//! `SemioVideoMutation`'s `protocol::OpBinary` impl in the facet root `🦀️.rs`; this leaf
//! carries the normative protocol description.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::video::schema::mutations::*;
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::video::schema::diff::{diff_insert_sample, diff_insert_stream, diff_remove_sample, diff_remove_stream, diff_set_sample_data, diff_set_sample_flags, diff_set_snapshot, diff_set_stream_meta, SemioVideoDiff};
use crate::standards::v1::subsets::video::io::text::snapshot::{dec_stream};
use crate::standards::v1::subsets::video::io::text::snapshot::{enc_stream};
use crate::standards::v1::subsets::video::io::text::snapshot::{dec_sample};
use crate::standards::v1::subsets::video::io::text::snapshot::{enc_sample};
use crate::standards::v1::subsets::video::io::text::snapshot::{dec_rational};
use crate::standards::v1::subsets::video::io::text::snapshot::{enc_rational};
use crate::standards::v1::subsets::video::io::text::snapshot::{dec_kind};
use crate::standards::v1::subsets::video::io::text::snapshot::{enc_kind};
use crate::standards::v1::subsets::video::io::text::snapshot::{dec_bool};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_bool};
use crate::standards::v1::subsets::document::io::text::mutations::{parse_usize};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{hex_decode};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{hex_encode};
use crate::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind};
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use crate::standards::v1::subsets::video::io::text::mutations::{print_semio_video_mutation};
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn wire_tag(m: &SemioVideoMutation) -> u8 {
    match m {
        SemioVideoMutation::SetSnapshot(_) => TAG_SET_SNAPSHOT,
        SemioVideoMutation::PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
        SemioVideoMutation::InsertStream(_) => TAG_INSERT_STREAM,
        SemioVideoMutation::RemoveStream(_) => TAG_REMOVE_STREAM,
        SemioVideoMutation::SetStreamMeta(_) => TAG_SET_STREAM_META,
        SemioVideoMutation::InsertSample(_) => TAG_INSERT_SAMPLE,
        SemioVideoMutation::RemoveSample(_) => TAG_REMOVE_SAMPLE,
        SemioVideoMutation::SetSampleData(_) => TAG_SET_SAMPLE_DATA,
        SemioVideoMutation::SetSampleFlags(_) => TAG_SET_SAMPLE_FLAGS,
    }
}

/// ✂️ Just the `key=value ...` argument tail of `print_semio_video_mutation` — the binary frame's
/// `tag` byte already carries the keyword, so the text keyword itself is redundant in the binary
/// payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_semio_video_mutation_args(m: &SemioVideoMutation) -> String {
    match print_semio_video_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ Real binary op frame, replacing the old `print_op().into_bytes()` text-as-binary shortcut
/// (same treatment flow's/mesh's own upgraded mutations facets use). `format u8`
/// (`OP_BINARY_FORMAT` convention) + `tag u8` (its kind's record tag in `💾️binary/📡️.protocol.semio`) are two
/// REAL fixed fields; the variant's own `key=value ...` argument payload follows as one opaque
/// trailing `bytes` chain — reusing the already-real, already-tested `print_semio_video_mutation`/
/// `parse_semio_video_mutation` text codec rather than re-deriving a second independent encoding.
impl OpBinary for SemioVideoMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        if let Self::PatchSnapshot(payload) = self {
            let mut out = vec![1, TAG_PATCH_SNAPSHOT];
            out.extend(protocol::OpBinary::encode_op(&payload.patch)?);
            return Ok(out);
        }
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, wire_tag(self)];
        out.extend_from_slice(print_semio_video_mutation_args(self).as_bytes());
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
            return Ok(Self::PatchSnapshot(crate::standards::v1::subsets::video::schema::mutations::patch_snapshot::PatchSnapshot { patch: protocol::OpBinary::decode_op(&bytes[2..])? }));
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
/// 🏷️ Op tags of `SemioVideoMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_PATCH_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "patch-snapshot");
const TAG_INSERT_STREAM: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-stream");
const TAG_REMOVE_STREAM: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-stream");
const TAG_SET_STREAM_META: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-stream-meta");
const TAG_INSERT_SAMPLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-sample");
const TAG_REMOVE_SAMPLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-sample");
const TAG_SET_SAMPLE_DATA: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-sample-data");
const TAG_SET_SAMPLE_FLAGS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-sample-flags");
//#endregion 🏷️WireTags
