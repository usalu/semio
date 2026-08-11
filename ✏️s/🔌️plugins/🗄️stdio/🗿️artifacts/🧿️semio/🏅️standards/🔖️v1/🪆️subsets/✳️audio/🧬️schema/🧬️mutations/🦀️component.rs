//! 🧬️ SemioAudioMutation — the full named-variant vocabulary replacing the W1b `SetSnapshot`-only
//! scaffold: scalar setters (`SetSampleRate`/`SetFormat`), channel insert/remove/set-samples, and
//! tag insert/remove/set-value. Every variant's `diff()`/`inverse()` is handcrafted directly
//! against the sparse `SemioAudioDiff` shape — never apply-and-capture (per the schema-design
//! recipe's own svg infinite-recursion warning).

use crate::artifacts::semio::standards::v1::engine::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::diff::{
    self, SemioAudioChannelDiff, SemioAudioDiff, dec_channel, dec_f32_list, dec_format, dec_snapshot, dec_tag,
    enc_channel, enc_f32_list, enc_format, enc_snapshot, enc_tag, hex_decode_string, hex_encode,
    parse_u32, parse_usize,
};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
use protocol::Mutation;
#[cfg(test)]
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `s.stdio.semio.audio`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioAudioMutation {
    #[default]
    NoMutation,
    SetSnapshot { snapshot: SemioAudioSnapshot },
    SetSampleRate { sample_rate: u32 },
    SetFormat { format: SemioAudioFormat },
    InsertChannel { index: usize, channel: SemioAudioChannel },
    RemoveChannel { index: usize },
    SetChannelSamples { index: usize, samples: Vec<f32> },
    InsertTag { index: usize, tag: SemioAudioTag },
    RemoveTag { index: usize },
    SetTagValue { index: usize, value: String },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Out-of-range channel/tag indices are no-ops rather than
/// panics — a stale index (e.g. from a concurrent edit) degrades gracefully.
pub fn apply_semio_audio_mutation(snapshot: &mut SemioAudioSnapshot, mutation: &SemioAudioMutation) -> SemioAudioDiff {
    let __diff = <SemioAudioMutation as Mutation<SemioAudioSnapshot>>::diff(mutation, snapshot);
    *snapshot = <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(&__diff, snapshot);
    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<SemioAudioSnapshot> for SemioAudioMutation {
    type Diff = SemioAudioDiff;

    fn diff(&self, base: &SemioAudioSnapshot) -> Self::Diff {
        match self {
            SemioAudioMutation::NoMutation => SemioAudioDiff::default(),
            SemioAudioMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            SemioAudioMutation::SetSampleRate { sample_rate } => SemioAudioDiff {
                sample_rate: (*sample_rate != base.sample_rate).then_some(*sample_rate),
                ..Default::default()
            },
            SemioAudioMutation::SetFormat { format } => SemioAudioDiff {
                format: (*format != base.format).then_some(*format),
                ..Default::default()
            },
            SemioAudioMutation::InsertChannel { index, channel } => SemioAudioDiff {
                channels: Some(IndexedTripleDiff { added: vec![IndexAdded { index: (*index).min(base.channels.len()), item: channel.clone() }], ..Default::default() }),
                ..Default::default()
            },
            SemioAudioMutation::RemoveChannel { index } => SemioAudioDiff {
                channels: Some(IndexedTripleDiff { removed: vec![*index], ..Default::default() }),
                ..Default::default()
            },
            SemioAudioMutation::SetChannelSamples { index, samples } => {
                let d = SemioAudioChannelDiff { samples: Some(samples.clone()) };
                SemioAudioDiff { channels: Some(IndexedTripleDiff { modified: vec![IndexModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
            }
            SemioAudioMutation::InsertTag { index, tag } => SemioAudioDiff {
                tags: Some(IndexedTripleDiff { added: vec![IndexAdded { index: (*index).min(base.tags.len()), item: tag.clone() }], ..Default::default() }),
                ..Default::default()
            },
            SemioAudioMutation::RemoveTag { index } => SemioAudioDiff {
                tags: Some(IndexedTripleDiff { removed: vec![*index], ..Default::default() }),
                ..Default::default()
            },
            SemioAudioMutation::SetTagValue { index, value } => match base.tags.get(*index) {
                Some(t) => SemioAudioDiff {
                    tags: Some(IndexedTripleDiff { modified: vec![IndexModified { index: *index, diff: SemioAudioTag { key: t.key.clone(), value: value.clone() } }], ..Default::default() }),
                    ..Default::default()
                },
                None => SemioAudioDiff::default(),
            },
        }
    }

    /// ↩️ Real, round-trippable inverses: `apply(inverse(m, base), apply(m, base)) == base` for
    /// every variant, including the channel/tag-index ops.
    fn inverse(&self, base: &SemioAudioSnapshot) -> Vec<Self> {
        match self {
            SemioAudioMutation::NoMutation => vec![SemioAudioMutation::NoMutation],
            SemioAudioMutation::SetSnapshot { .. } => vec![SemioAudioMutation::SetSnapshot { snapshot: base.clone() }],
            SemioAudioMutation::SetSampleRate { .. } => vec![SemioAudioMutation::SetSampleRate { sample_rate: base.sample_rate }],
            SemioAudioMutation::SetFormat { .. } => vec![SemioAudioMutation::SetFormat { format: base.format }],
            SemioAudioMutation::InsertChannel { index, .. } => vec![SemioAudioMutation::RemoveChannel { index: (*index).min(base.channels.len()) }],
            SemioAudioMutation::RemoveChannel { index } => match base.channels.get(*index) {
                Some(c) => vec![SemioAudioMutation::InsertChannel { index: *index, channel: c.clone() }],
                None => vec![SemioAudioMutation::NoMutation],
            },
            SemioAudioMutation::SetChannelSamples { index, .. } => match base.channels.get(*index) {
                Some(c) => vec![SemioAudioMutation::SetChannelSamples { index: *index, samples: c.samples.clone() }],
                None => vec![SemioAudioMutation::NoMutation],
            },
            SemioAudioMutation::InsertTag { index, .. } => vec![SemioAudioMutation::RemoveTag { index: (*index).min(base.tags.len()) }],
            SemioAudioMutation::RemoveTag { index } => match base.tags.get(*index) {
                Some(t) => vec![SemioAudioMutation::InsertTag { index: *index, tag: t.clone() }],
                None => vec![SemioAudioMutation::NoMutation],
            },
            SemioAudioMutation::SetTagValue { index, .. } => match base.tags.get(*index) {
                Some(t) => vec![SemioAudioMutation::SetTagValue { index: *index, value: t.value.clone() }],
                None => vec![SemioAudioMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Hand-rolled `OpText`/`OpBinary` per the ticket's blanket instruction — real one-line
/// `keyword payload...` grammar (not `serde_json`), reusing the diff module's own bracket value
/// codecs (`enc_channel`/`enc_tag`/`enc_format`/`enc_snapshot`/…) so a mutation's embedded payload
/// (e.g. `SetSnapshot`'s whole snapshot, `InsertChannel`'s channel) prints identically to how the
/// same value would print inside a diff's `added` triple. Binary = the text bytes verbatim, same
/// simplification `SemioAudioDiff::encode_diff`/gif 89a's `GifDiff::encode_diff` both use.
fn print_audio_mutation(m: &SemioAudioMutation) -> String {
    match m {
        SemioAudioMutation::NoMutation => "no-mutation".to_string(),
        SemioAudioMutation::SetSnapshot { snapshot } => format!("set-snapshot {}", enc_snapshot(snapshot)),
        SemioAudioMutation::SetSampleRate { sample_rate } => format!("set-sample-rate {sample_rate}"),
        SemioAudioMutation::SetFormat { format } => format!("set-format {}", enc_format(*format)),
        SemioAudioMutation::InsertChannel { index, channel } => format!("insert-channel {index} {}", enc_channel(channel)),
        SemioAudioMutation::RemoveChannel { index } => format!("remove-channel {index}"),
        SemioAudioMutation::SetChannelSamples { index, samples } => format!("set-channel-samples {index} {}", enc_f32_list(samples)),
        SemioAudioMutation::InsertTag { index, tag } => format!("insert-tag {index} {}", enc_tag(tag)),
        SemioAudioMutation::RemoveTag { index } => format!("remove-tag {index}"),
        SemioAudioMutation::SetTagValue { index, value } => format!("set-tag-value {index} {}", hex_encode(value.as_bytes())),
    }
}

fn parse_audio_mutation(line: &str) -> Result<SemioAudioMutation, String> {
    if line == "no-mutation" { return Ok(SemioAudioMutation::NoMutation); }
    let (keyword, rest) = line.split_once(' ').ok_or_else(|| format!("audio mutation: missing payload in {line:?}"))?;
    match keyword {
        "set-snapshot" => Ok(SemioAudioMutation::SetSnapshot { snapshot: dec_snapshot(rest)? }),
        "set-sample-rate" => Ok(SemioAudioMutation::SetSampleRate { sample_rate: parse_u32(rest)? }),
        "set-format" => Ok(SemioAudioMutation::SetFormat { format: dec_format(rest)? }),
        "insert-channel" => {
            let (idx, enc) = rest.split_once(' ').ok_or_else(|| "insert-channel: missing channel payload".to_string())?;
            Ok(SemioAudioMutation::InsertChannel { index: parse_usize(idx)?, channel: dec_channel(enc)? })
        }
        "remove-channel" => Ok(SemioAudioMutation::RemoveChannel { index: parse_usize(rest)? }),
        "set-channel-samples" => {
            let (idx, enc) = rest.split_once(' ').ok_or_else(|| "set-channel-samples: missing payload".to_string())?;
            Ok(SemioAudioMutation::SetChannelSamples { index: parse_usize(idx)?, samples: dec_f32_list(enc)? })
        }
        "insert-tag" => {
            let (idx, enc) = rest.split_once(' ').ok_or_else(|| "insert-tag: missing payload".to_string())?;
            Ok(SemioAudioMutation::InsertTag { index: parse_usize(idx)?, tag: dec_tag(enc)? })
        }
        "remove-tag" => Ok(SemioAudioMutation::RemoveTag { index: parse_usize(rest)? }),
        "set-tag-value" => {
            let (idx, enc) = rest.split_once(' ').ok_or_else(|| "set-tag-value: missing payload".to_string())?;
            Ok(SemioAudioMutation::SetTagValue { index: parse_usize(idx)?, value: hex_decode_string(enc)? })
        }
        other => Err(format!("audio mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for SemioAudioMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_audio_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String { print_audio_mutation(self) }
}

impl protocol::OpBinary for SemioAudioMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(print_audio_mutation(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        parse_audio_mutation(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_channel(seed: f32) -> SemioAudioChannel {
        SemioAudioChannel { samples: vec![seed, seed + 1.0, seed + 2.0] }
    }

    fn base_snapshot() -> SemioAudioSnapshot {
        SemioAudioSnapshot {
            sample_rate: 44_100,
            format: SemioAudioFormat::Pcm16,
            channels: vec![sample_channel(1.0), sample_channel(2.0), sample_channel(3.0)],
            tags: vec![SemioAudioTag { key: "title".into(), value: "t0".into() }],
            ..SemioAudioSnapshot::default()
        }
    }

    fn round_trips(base: &SemioAudioSnapshot, mutation: SemioAudioMutation) {
        let diff = mutation.diff(base);
        let mutated = <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(&diff, base);
        let inverses = mutation.inverse(base);
        let mut restored = mutated.clone();
        for inv in &inverses {
            let inv_diff = inv.diff(&restored);
            restored = <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(&inv_diff, &restored);
        }
        assert_eq!(&restored, base, "apply(inverse(m), apply(m, base)) must recover base for {mutation:?}");
    }

    fn all_variants(base: &SemioAudioSnapshot) -> Vec<SemioAudioMutation> {
        vec![
            SemioAudioMutation::NoMutation,
            SemioAudioMutation::SetSnapshot { snapshot: SemioAudioSnapshot { sample_rate: 9_000, ..base.clone() } },
            SemioAudioMutation::SetSampleRate { sample_rate: 48_000 },
            SemioAudioMutation::SetFormat { format: SemioAudioFormat::Float32 },
            SemioAudioMutation::InsertChannel { index: 1, channel: sample_channel(9.0) },
            SemioAudioMutation::RemoveChannel { index: 1 },
            SemioAudioMutation::SetChannelSamples { index: 0, samples: vec![0.25, 0.5, 0.75] },
            SemioAudioMutation::InsertTag { index: 0, tag: SemioAudioTag { key: "artist".into(), value: "a".into() } },
            SemioAudioMutation::RemoveTag { index: 0 },
            SemioAudioMutation::SetTagValue { index: 0, value: "changed".into() },
        ]
    }

    /// 🧪️ `mutation_diff_law`: every variant's `diff()` matches what `apply_semio_audio_mutation`
    /// returns.
    #[test]
    fn mutation_diff_law() {
        let base = base_snapshot();
        for mutation in all_variants(&base) {
            let mut snap = base.clone();
            let returned_diff = apply_semio_audio_mutation(&mut snap, &mutation);
            let expected_diff = mutation.diff(&base);
            assert_eq!(returned_diff, expected_diff, "returned diff must equal mutation.diff(base) for {mutation:?}");
            assert_eq!(snap, <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(&expected_diff, &base), "apply_semio_audio_mutation must match diff.apply(base) for {mutation:?}");
        }
    }

    /// 🧪️ `inverse_law` (mutation-level): every variant round-trips.
    #[test]
    fn mutation_apply_inverse_round_trips_every_variant() {
        let base = base_snapshot();
        for mutation in all_variants(&base) {
            round_trips(&base, mutation);
        }
    }

    #[test]
    fn remove_channel_out_of_range_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_semio_audio_mutation(&mut snap, &SemioAudioMutation::RemoveChannel { index: 99 });
        assert_eq!(snap, base);
    }

    /// 🧪️ `op_text_binary_roundtrip_law`: hand-rolled `OpText`/`OpBinary` round-trip over the
    /// full variant vocabulary.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        for mutation in all_variants(&base) {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioAudioMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = SemioAudioMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
}
//#endregion 🔖️Tests
