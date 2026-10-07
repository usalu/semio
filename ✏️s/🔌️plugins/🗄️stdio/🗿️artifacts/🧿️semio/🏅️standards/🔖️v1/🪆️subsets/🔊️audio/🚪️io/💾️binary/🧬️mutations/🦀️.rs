//! 💾️ Binary representation codec surface for `s.stdio.semio.audio` (mutations).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::audio::schema::mutations::*;
use crate::standards::v1::subsets::audio::schema::diff::{self, SemioAudioChannelDiff, SemioAudioDiff};
use crate::standards::v1::subsets::audio::io::text::diff::{hex_decode_string};
use crate::standards::v1::subsets::audio::io::text::snapshot::{dec_tag};
use crate::standards::v1::subsets::audio::io::text::snapshot::{enc_tag};
use crate::standards::v1::subsets::animation::io::text::snapshot::{dec_channel};
use crate::standards::v1::subsets::animation::io::text::snapshot::{enc_channel};
use crate::standards::v1::subsets::audio::io::text::snapshot::{dec_f32_list};
use crate::standards::v1::subsets::audio::io::text::snapshot::{enc_f32_list};
use crate::standards::v1::subsets::audio::io::text::snapshot::{dec_format};
use crate::standards::v1::subsets::audio::io::text::snapshot::{enc_format};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{parse_u32};
use crate::standards::v1::subsets::audio::io::text::snapshot::{dec_snapshot};
use crate::standards::v1::subsets::audio::io::text::snapshot::{enc_snapshot};
use crate::standards::v1::subsets::document::io::text::mutations::{parse_usize};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{hex_encode};
use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
/// 🔧️ Unconditional — `impl protocol::OpBinary for SemioAudioMutation` below's `encode_op`/
/// `decode_op` are now real production code (binary upgrade, this wave), not test-only.
use protocol::{Mutation, OpBinary, OpText};
use crate::standards::v1::subsets::audio::io::text::mutations::{print_audio_mutation};
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn wire_tag(m: &SemioAudioMutation) -> u8 {
    match m {
        SemioAudioMutation::SetSnapshot(_) => TAG_SET_SNAPSHOT,
        SemioAudioMutation::PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
        SemioAudioMutation::SetSampleRate(_) => TAG_SET_SAMPLE_RATE,
        SemioAudioMutation::SetFormat(_) => TAG_SET_FORMAT,
        SemioAudioMutation::InsertChannel(_) => TAG_INSERT_CHANNEL,
        SemioAudioMutation::RemoveChannel(_) => TAG_REMOVE_CHANNEL,
        SemioAudioMutation::SetChannelSamples(_) => TAG_SET_CHANNEL_SAMPLES,
        SemioAudioMutation::InsertTag(_) => TAG_INSERT_TAG,
        SemioAudioMutation::RemoveTag(_) => TAG_REMOVE_TAG,
        SemioAudioMutation::SetTagValue(_) => TAG_SET_TAG_VALUE,
    }
}

/// ✂️ Just the argument tail of `print_audio_mutation` — the binary frame's `tag` byte already
/// carries the keyword, so the text keyword itself (and its separating space) is redundant in the
/// binary payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_audio_mutation_args(m: &SemioAudioMutation) -> String {
    match print_audio_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ Real binary op frame, replacing the old `print_op().into_bytes()` text-as-binary shortcut.
/// `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (its kind's record tag in `💾️binary/📡️.protocol.semio`) are two REAL fixed fields; the variant's own argument payload follows as one
/// opaque trailing `bytes` chain — reuses the already-real, already-tested `print_audio_mutation`/
/// `parse_audio_mutation` text codec rather than re-deriving a second independent encoding.
impl OpBinary for SemioAudioMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        if let Self::PatchSnapshot(payload) = self {
            let mut out = vec![1, TAG_PATCH_SNAPSHOT];
            out.extend(protocol::OpBinary::encode_op(&payload.patch)?);
            return Ok(out);
        }
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, wire_tag(self)];
        out.extend_from_slice(print_audio_mutation_args(self).as_bytes());
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
            return Ok(Self::PatchSnapshot(crate::standards::v1::subsets::audio::schema::mutations::patch_snapshot::PatchSnapshot { patch: protocol::OpBinary::decode_op(&bytes[2..])? }));
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
/// 🏷️ Op tags of `SemioAudioMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_PATCH_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "patch-snapshot");
const TAG_SET_SAMPLE_RATE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-sample-rate");
const TAG_SET_FORMAT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-format");
const TAG_INSERT_CHANNEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-channel");
const TAG_REMOVE_CHANNEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-channel");
const TAG_SET_CHANNEL_SAMPLES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-channel-samples");
const TAG_INSERT_TAG: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-tag");
const TAG_REMOVE_TAG: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-tag");
const TAG_SET_TAG_VALUE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-tag-value");
//#endregion 🏷️WireTags
