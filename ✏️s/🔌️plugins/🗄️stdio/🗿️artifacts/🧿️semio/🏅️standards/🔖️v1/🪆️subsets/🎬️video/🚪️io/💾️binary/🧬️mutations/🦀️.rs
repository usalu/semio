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
use crate::schema::mutations::*;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_video_model::schema::diff::diff_insert_sample;
use semio_s_artifact_stdio_semio_video_model::schema::diff::diff_insert_stream;
use semio_s_artifact_stdio_semio_video_model::schema::diff::diff_remove_sample;
use semio_s_artifact_stdio_semio_video_model::schema::diff::diff_remove_stream;
use semio_s_artifact_stdio_semio_video_model::schema::diff::diff_set_sample_data;
use semio_s_artifact_stdio_semio_video_model::schema::diff::diff_set_sample_flags;
use semio_s_artifact_stdio_semio_video_model::schema::diff::diff_set_stream_meta;
use semio_s_artifact_stdio_semio_video_model::schema::diff::SemioVideoDiff;
use semio_s_artifact_stdio_semio_video_model::io::text::snapshot::dec_stream;
use semio_s_artifact_stdio_semio_video_model::io::text::snapshot::enc_stream;
use semio_s_artifact_stdio_semio_video_model::io::text::snapshot::dec_sample;
use semio_s_artifact_stdio_semio_video_model::io::text::snapshot::enc_sample;
use semio_s_artifact_stdio_semio_video_model::io::text::snapshot::dec_rational;
use semio_s_artifact_stdio_semio_video_model::io::text::snapshot::enc_rational;
use semio_s_artifact_stdio_semio_video_model::io::text::snapshot::dec_kind;
use semio_s_artifact_stdio_semio_video_model::io::text::snapshot::enc_kind;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_bool;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_bool;
use semio_s_artifact_stdio_semio_base::io::text::codec::parse_usize;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_list;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_list;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::hex_decode;
use semio_s_artifact_stdio_semio_base::io::text::codec::hex_encode;
use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioRational;
use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSample;
use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot;
use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoStream;
use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoStreamKind;
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use crate::io::text::mutations::print_semio_video_mutation;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn wire_tag(m: &SemioVideoMutation) -> u8 {
    match m {
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
        let tag = bytes[1];
        let keyword = dsl::protocol_record::kind(WIRE_PROTOCOL, u64::from(tag)).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} names no record of 📡️.protocol.semio") })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
}
#[cfg(test)] pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioVideoMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_INSERT_STREAM: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-stream");
const TAG_REMOVE_STREAM: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-stream");
const TAG_SET_STREAM_META: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-stream-meta");
const TAG_INSERT_SAMPLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-sample");
const TAG_REMOVE_SAMPLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-sample");
const TAG_SET_SAMPLE_DATA: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-sample-data");
const TAG_SET_SAMPLE_FLAGS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-sample-flags");
//#endregion 🏷️WireTags
