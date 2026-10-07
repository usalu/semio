//! 📝️ Text representation codec surface for `s.stdio.semio.audio` (mutations).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::audio::schema::mutations::*;
use crate::standards::v1::subsets::audio::schema::diff::{self, SemioAudioChannelDiff, SemioAudioDiff};
use crate::standards::v1::subsets::audio::io::text::diff::{hex_decode_string};
use crate::standards::v1::subsets::audio::io::text::snapshot::{dec_tag};
use crate::standards::v1::subsets::audio::io::text::snapshot::{enc_tag};
use crate::standards::v1::subsets::audio::io::text::snapshot::{dec_channel};
use crate::standards::v1::subsets::audio::io::text::snapshot::{enc_channel};
use crate::standards::v1::subsets::audio::io::text::snapshot::{dec_f32_list};
use crate::standards::v1::subsets::audio::io::text::snapshot::{enc_f32_list};
use crate::standards::v1::subsets::audio::io::text::snapshot::{dec_format};
use crate::standards::v1::subsets::audio::io::text::snapshot::{enc_format};
use crate::standards::v1::subsets::audio::io::text::snapshot::{parse_u32};
use crate::standards::v1::subsets::audio::io::text::snapshot::{dec_snapshot};
use crate::standards::v1::subsets::audio::io::text::snapshot::{enc_snapshot};
use crate::standards::v1::subsets::document::io::text::mutations::{parse_usize};
use crate::standards::v1::subsets::audio::io::text::snapshot::{hex_encode};
use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
/// 🔧️ Unconditional — `impl protocol::OpBinary for SemioAudioMutation` below's `encode_op`/
/// `decode_op` are now real production code (binary upgrade, this wave), not test-only.
use protocol::{Mutation, OpBinary, OpText};

/// 📥️ Decodes this subset's internally tagged (`{"mutation": "<camelCaseVariant>", ...}`) wire value — the shape
/// `🔊️mutate-semio-audio`'s committed specification vectors and doc strings carry — into a real [`SemioAudioMutation`]. A thin
/// `pack::from_json_str` wrapper over `ToValue`/`FromValue`, so the test adapter reads the committed wire value instead of
/// re-declaring it field by field beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_audio_mutation_json(text: &str) -> Result<SemioAudioMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 🎙️ Hand-rolled `OpText`/`OpBinary` per the ticket's blanket instruction — real one-line
/// `keyword payload...` grammar (not `serde_json`), reusing the diff module's own bracket value
/// codecs (`enc_channel`/`enc_tag`/`enc_format`/`enc_snapshot`/…) so a mutation's embedded payload
/// (e.g. `SetSnapshot`'s whole snapshot, `InsertChannel`'s channel) prints identically to how the
/// same value would print inside a diff's `added` triple. Binary = the text bytes verbatim, same
/// simplification `SemioAudioDiff::encode_diff`/gif 89a's `GifDiff::encode_diff` both use.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_audio_mutation(m: &SemioAudioMutation) -> String {
    match m {
        SemioAudioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot {}", enc_snapshot(snapshot)),
        SemioAudioMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),
        SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate }) => format!("set-sample-rate {sample_rate}"),
        SemioAudioMutation::SetFormat(set_format::SetFormat { format }) => format!("set-format {}", enc_format(*format)),
        SemioAudioMutation::InsertChannel(insert_channel::InsertChannel { index, channel }) => format!("insert-channel {index} {}", enc_channel(channel)),
        SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index }) => format!("remove-channel {index}"),
        SemioAudioMutation::SetChannelSamples(set_channel_samples::SetChannelSamples { index, samples }) => format!("set-channel-samples {index} {}", enc_f32_list(samples)),
        SemioAudioMutation::InsertTag(insert_tag::InsertTag { index, tag }) => format!("insert-tag {index} {}", enc_tag(tag)),
        SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index }) => format!("remove-tag {index}"),
        SemioAudioMutation::SetTagValue(set_tag_value::SetTagValue { index, value }) => format!("set-tag-value {index} {}", hex_encode(value.as_bytes())),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_audio_mutation(line: &str) -> Result<SemioAudioMutation, String> {
    if let Some(source) = line.strip_prefix("patch-snapshot patch=") {
        let patch = semio_s_artifact_stdio_contract::editing::snapshot_patch_from_hex(source)?;
        return Ok(SemioAudioMutation::PatchSnapshot(crate::standards::v1::subsets::audio::schema::mutations::patch_snapshot::PatchSnapshot { patch }));
    }
    let (keyword, rest) = line.split_once(' ').ok_or_else(|| format!("audio mutation: missing payload in {line:?}"))?;
    match keyword {
        "set-snapshot" => Ok(SemioAudioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_snapshot(rest)? })),
        "set-sample-rate" => Ok(SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: parse_u32(rest)? })),
        "set-format" => Ok(SemioAudioMutation::SetFormat(set_format::SetFormat { format: dec_format(rest)? })),
        "insert-channel" => {
            let (idx, enc) = rest.split_once(' ').ok_or_else(|| "insert-channel: missing channel payload".to_string())?;
            Ok(SemioAudioMutation::InsertChannel(insert_channel::InsertChannel { index: parse_usize(idx)?, channel: dec_channel(enc)? }))
        }
        "remove-channel" => Ok(SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index: parse_usize(rest)? })),
        "set-channel-samples" => {
            let (idx, enc) = rest.split_once(' ').ok_or_else(|| "set-channel-samples: missing payload".to_string())?;
            Ok(SemioAudioMutation::SetChannelSamples(set_channel_samples::SetChannelSamples { index: parse_usize(idx)?, samples: dec_f32_list(enc)? }))
        }
        "insert-tag" => {
            let (idx, enc) = rest.split_once(' ').ok_or_else(|| "insert-tag: missing payload".to_string())?;
            Ok(SemioAudioMutation::InsertTag(insert_tag::InsertTag { index: parse_usize(idx)?, tag: dec_tag(enc)? }))
        }
        "remove-tag" => Ok(SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index: parse_usize(rest)? })),
        "set-tag-value" => {
            let (idx, enc) = rest.split_once(' ').ok_or_else(|| "set-tag-value: missing payload".to_string())?;
            Ok(SemioAudioMutation::SetTagValue(set_tag_value::SetTagValue { index: parse_usize(idx)?, value: hex_decode_string(enc)? }))
        }
        other => Err(format!("audio mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioAudioMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_audio_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        print_audio_mutation(self)
    }
}
}
pub use mutations_codec::*;
