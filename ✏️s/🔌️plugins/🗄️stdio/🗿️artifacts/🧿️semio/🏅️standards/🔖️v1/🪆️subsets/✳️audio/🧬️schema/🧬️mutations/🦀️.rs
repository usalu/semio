//! 🧬️ SemioAudioMutation — the full named-variant vocabulary replacing the W1b `SetSnapshot`-only
//! scaffold: scalar setters (`SetSampleRate`/`SetFormat`), channel insert/remove/set-samples, and
//! tag insert/remove/set-value. Every variant's `diff()`/`inverse()` is handcrafted directly
//! against the sparse `SemioAudioDiff` shape — never apply-and-capture (per the schema-design
//! recipe's own svg infinite-recursion warning).
//!
//! `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one
//! leaf payload, and its sentinel verb `no` is not in `APPROVED_VERBS` — see
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`. Every variant is now a
//! tuple variant wrapping its own mutation leaf (`./*/🦀️.rs`), and this file's `agg_diff`/
//! `agg_inverse` carry the handcrafted semantics every leaf's `MutationKind` impl delegates back to.

use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::diff::{
    self, dec_channel, dec_f32_list, dec_format, dec_snapshot, dec_tag, enc_channel, enc_f32_list, enc_format, enc_snapshot, enc_tag, hex_decode_string, hex_encode, parse_u32, parse_usize, SemioAudioChannelDiff, SemioAudioDiff,
};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
/// 🔧️ Unconditional — `impl protocol::OpBinary for SemioAudioMutation` below's `encode_op`/
/// `decode_op` are now real production code (binary upgrade, this wave), not test-only.
use protocol::{Mutation, OpBinary, OpText};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `s.stdio.semio.audio`.
//#region 🔖️Leaves
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🎚set-sample-rate/🦀️.rs"]
pub mod set_sample_rate;
#[path = "💽set-format/🦀️.rs"]
pub mod set_format;
#[path = "🎙insert-channel/🦀️.rs"]
pub mod insert_channel;
#[path = "🔇remove-channel/🦀️.rs"]
pub mod remove_channel;
#[path = "🌊set-channel-samples/🦀️.rs"]
pub mod set_channel_samples;
#[path = "🏷insert-tag/🦀️.rs"]
pub mod insert_tag;
#[path = "✂remove-tag/🦀️.rs"]
pub mod remove_tag;
#[path = "💬set-tag-value/🦀️.rs"]
pub mod set_tag_value;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioAudioSnapshot, diff = SemioAudioDiff, schema = "SemioAudioMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioAudioMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetSampleRate(set_sample_rate::SetSampleRate),
    SetFormat(set_format::SetFormat),
    InsertChannel(insert_channel::InsertChannel),
    RemoveChannel(remove_channel::RemoveChannel),
    SetChannelSamples(set_channel_samples::SetChannelSamples),
    InsertTag(insert_tag::InsertTag),
    RemoveTag(remove_tag::RemoveTag),
    SetTagValue(set_tag_value::SetTagValue),
}

/// 🏷️ The declared kebab-case mutation vocabulary of `s.stdio.semio.audio`, in enum declaration
/// order — what the `mutate-semio-audio` case's completeness gate counts against and what
/// `../../🔣️oracle.json`'s catalog repeats. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this declaration honest.
pub const KINDS: &[&str] = &["set-snapshot", "set-sample-rate", "set-format", "insert-channel", "remove-channel", "set-channel-samples", "insert-tag", "remove-tag", "set-tag-value"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Out-of-range channel/tag indices are no-ops rather than
/// panics — a stale index (e.g. from a concurrent edit) degrades gracefully.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_audio_mutation(snapshot: &mut SemioAudioSnapshot, mutation: &SemioAudioMutation) -> protocol::MutationOutcome<SemioAudioDiff> {
    let outcome = <SemioAudioMutation as Mutation<SemioAudioSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Free-function face of [`SemioAudioMutation`]'s own `protocol::Mutation::inverse`. `Mutation` is
/// declared by the os-kernel, which is an INTERNAL dependency of this plugin (aliased `protocol` in
/// `🦀️.rs`) and is therefore not nameable by a consumer that links only this crate — a
/// generated test host being the concrete case. Paired with [`apply_semio_audio_mutation`] it makes the
/// undo law reachable without importing a trait the caller cannot name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_audio_mutation(mutation: &SemioAudioMutation, base: &SemioAudioSnapshot) -> Vec<SemioAudioMutation> {
    <SemioAudioMutation as Mutation<SemioAudioSnapshot>>::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioAudioMutation, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<SemioAudioDiff> {
    protocol::MutationOutcome::new(match this {
        SemioAudioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff::diff_set_snapshot(base, snapshot),
        SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate }) => SemioAudioDiff { sample_rate: (*sample_rate != base.sample_rate).then_some(*sample_rate), ..Default::default() },
        SemioAudioMutation::SetFormat(set_format::SetFormat { format }) => SemioAudioDiff { format: (*format != base.format).then_some(*format), ..Default::default() },
        SemioAudioMutation::InsertChannel(insert_channel::InsertChannel { index, channel }) => {
            SemioAudioDiff { channels: Some(IndexedTripleDiff { added: vec![IndexAdded { index: (*index).min(base.channels.len()), item: channel.clone() }], ..Default::default() }), ..Default::default() }
        }
        SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index }) => SemioAudioDiff { channels: Some(IndexedTripleDiff { removed: vec![*index], ..Default::default() }), ..Default::default() },
        SemioAudioMutation::SetChannelSamples(set_channel_samples::SetChannelSamples { index, samples }) => {
            let d = SemioAudioChannelDiff { samples: Some(samples.clone()) };
            SemioAudioDiff { channels: Some(IndexedTripleDiff { modified: vec![IndexModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        }
        SemioAudioMutation::InsertTag(insert_tag::InsertTag { index, tag }) => SemioAudioDiff { tags: Some(IndexedTripleDiff { added: vec![IndexAdded { index: (*index).min(base.tags.len()), item: tag.clone() }], ..Default::default() }), ..Default::default() },
        SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index }) => SemioAudioDiff { tags: Some(IndexedTripleDiff { removed: vec![*index], ..Default::default() }), ..Default::default() },
        SemioAudioMutation::SetTagValue(set_tag_value::SetTagValue { index, value }) => match base.tags.get(*index) {
            Some(t) => SemioAudioDiff { tags: Some(IndexedTripleDiff { modified: vec![IndexModified { index: *index, diff: SemioAudioTag { key: t.key.clone(), value: value.clone() } }], ..Default::default() }), ..Default::default() },
            None => SemioAudioDiff::default(),
        },
    })
}

/// ↩️ Real, round-trippable inverses: `apply(inverse(m, base), apply(m, base)) == base` for every
/// variant, including the channel/tag-index ops. An index that no longer exists in `base` has
/// nothing to restore, so those arms return the empty inverse rather than a sentinel no-op mutation
/// — the convention this migration adopted once `NoMutation` stopped being an available payload.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioAudioMutation, base: &SemioAudioSnapshot) -> Vec<SemioAudioMutation> {
    vec![match this {
        SemioAudioMutation::SetSnapshot(_) => SemioAudioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        SemioAudioMutation::SetSampleRate(_) => SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: base.sample_rate }),
        SemioAudioMutation::SetFormat(_) => SemioAudioMutation::SetFormat(set_format::SetFormat { format: base.format }),
        SemioAudioMutation::InsertChannel(insert_channel::InsertChannel { index, .. }) => SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index: (*index).min(base.channels.len()) }),
        SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index }) => match base.channels.get(*index) {
            Some(c) => SemioAudioMutation::InsertChannel(insert_channel::InsertChannel { index: *index, channel: c.clone() }),
            None => return Vec::new(),
        },
        SemioAudioMutation::SetChannelSamples(set_channel_samples::SetChannelSamples { index, .. }) => match base.channels.get(*index) {
            Some(c) => SemioAudioMutation::SetChannelSamples(set_channel_samples::SetChannelSamples { index: *index, samples: c.samples.clone() }),
            None => return Vec::new(),
        },
        SemioAudioMutation::InsertTag(insert_tag::InsertTag { index, .. }) => SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index: (*index).min(base.tags.len()) }),
        SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index }) => match base.tags.get(*index) {
            Some(t) => SemioAudioMutation::InsertTag(insert_tag::InsertTag { index: *index, tag: t.clone() }),
            None => return Vec::new(),
        },
        SemioAudioMutation::SetTagValue(set_tag_value::SetTagValue { index, .. }) => match base.tags.get(*index) {
            Some(t) => SemioAudioMutation::SetTagValue(set_tag_value::SetTagValue { index: *index, value: t.value.clone() }),
            None => return Vec::new(),
        },
    }]
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Hand-rolled `OpText`/`OpBinary` per the ticket's blanket instruction — real one-line
/// `keyword payload...` grammar (not `serde_json`), reusing the diff module's own bracket value
/// codecs (`enc_channel`/`enc_tag`/`enc_format`/`enc_snapshot`/…) so a mutation's embedded payload
/// (e.g. `SetSnapshot`'s whole snapshot, `InsertChannel`'s channel) prints identically to how the
/// same value would print inside a diff's `added` triple. Binary = the text bytes verbatim, same
/// simplification `SemioAudioDiff::encode_diff`/gif 89a's `GifDiff::encode_diff` both use.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_audio_mutation(m: &SemioAudioMutation) -> String {
    match m {
        SemioAudioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot {}", enc_snapshot(snapshot)),
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
fn parse_audio_mutation(line: &str) -> Result<SemioAudioMutation, String> {
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
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_audio_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        print_audio_mutation(self)
    }
}

/// 🧾️ Keyword table + variant ordinal, 0-indexed in enum declaration order — the binary frame's
/// `tag` byte, `📖️grammar/component.grammar.semio`'s `op` alternatives, and this array must all
/// agree (see `committed_facet_files_parse`/`ops_grammar_conformance_law` in
/// `🎹️composer/🦀️.rs`).
const OP_KEYWORDS: [&str; 9] = ["set-snapshot", "set-sample-rate", "set-format", "insert-channel", "remove-channel", "set-channel-samples", "insert-tag", "remove-tag", "set-tag-value"];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn variant_ordinal(m: &SemioAudioMutation) -> u8 {
    match m {
        SemioAudioMutation::SetSnapshot(_) => 0,
        SemioAudioMutation::SetSampleRate(_) => 1,
        SemioAudioMutation::SetFormat(_) => 2,
        SemioAudioMutation::InsertChannel(_) => 3,
        SemioAudioMutation::RemoveChannel(_) => 4,
        SemioAudioMutation::SetChannelSamples(_) => 5,
        SemioAudioMutation::InsertTag(_) => 6,
        SemioAudioMutation::RemoveTag(_) => 7,
        SemioAudioMutation::SetTagValue(_) => 8,
    }
}
/// ✂️ Just the argument tail of `print_audio_mutation` — the binary frame's `tag` byte already
/// carries the keyword, so the text keyword itself (and its separating space) is redundant in the
/// binary payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_audio_mutation_args(m: &SemioAudioMutation) -> String {
    match print_audio_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ Real binary op frame, replacing the old `print_op().into_bytes()` text-as-binary shortcut.
/// `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, see
/// [`OP_KEYWORDS`]) are two REAL fixed fields; the variant's own argument payload follows as one
/// opaque trailing `bytes` chain — reuses the already-real, already-tested `print_audio_mutation`/
/// `parse_audio_mutation` text codec rather than re-deriving a second independent encoding.
impl OpBinary for SemioAudioMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self)];
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
        let tag = bytes[1];
        let keyword = OP_KEYWORDS.get(tag as usize).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} out of range for {} declared variants", OP_KEYWORDS.len()) })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 Representative `SemioAudioMutation` cases, one per variant — single source of truth for
/// `ops_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️.rs` and this
/// file's own `op_text_binary_roundtrip_law`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioAudioMutation> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn channel(seed: f32) -> SemioAudioChannel {
        SemioAudioChannel { samples: vec![seed, seed + 1.0, seed + 2.0] }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fixture() -> SemioAudioSnapshot {
        SemioAudioSnapshot { sample_rate: 44_100, format: SemioAudioFormat::Pcm16, channels: vec![channel(1.0), channel(2.0), channel(3.0)], tags: vec![SemioAudioTag { key: "title".into(), value: "t0".into() }], ..SemioAudioSnapshot::default() }
    }
    vec![
        SemioAudioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: SemioAudioSnapshot { sample_rate: 9_000, ..fixture() } }),
        SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: 48_000 }),
        SemioAudioMutation::SetFormat(set_format::SetFormat { format: SemioAudioFormat::Float32 }),
        SemioAudioMutation::InsertChannel(insert_channel::InsertChannel { index: 1, channel: channel(9.0) }),
        SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index: 1 }),
        SemioAudioMutation::SetChannelSamples(set_channel_samples::SetChannelSamples { index: 0, samples: vec![0.25, 0.5, 0.75] }),
        SemioAudioMutation::InsertTag(insert_tag::InsertTag { index: 0, tag: SemioAudioTag { key: "artist".into(), value: "a".into() } }),
        SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index: 0 }),
        SemioAudioMutation::SetTagValue(set_tag_value::SetTagValue { index: 0, value: "changed".into() }),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_channel(seed: f32) -> SemioAudioChannel {
        SemioAudioChannel { samples: vec![seed, seed + 1.0, seed + 2.0] }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> SemioAudioSnapshot {
        SemioAudioSnapshot {
            sample_rate: 44_100,
            format: SemioAudioFormat::Pcm16,
            channels: vec![sample_channel(1.0), sample_channel(2.0), sample_channel(3.0)],
            tags: vec![SemioAudioTag { key: "title".into(), value: "t0".into() }],
            ..SemioAudioSnapshot::default()
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn round_trips(base: &SemioAudioSnapshot, mutation: SemioAudioMutation) {
        let diff = mutation.diff(base);
        let mutated = <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(diff.diff(), base).expect("apply must succeed for a well-formed fixture");
        let inverses = mutation.inverse(base);
        let mut restored = mutated.clone();
        for inv in &inverses {
            let inv_diff = inv.diff(&restored);
            restored = <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(inv_diff.diff(), &restored).expect("apply must succeed for a well-formed fixture");
        }
        assert_eq!(&restored, base, "apply(inverse(m), apply(m, base)) must recover base for {mutation:?}");
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn all_variants(base: &SemioAudioSnapshot) -> Vec<SemioAudioMutation> {
        vec![
            SemioAudioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: SemioAudioSnapshot { sample_rate: 9_000, ..base.clone() } }),
            SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: 48_000 }),
            SemioAudioMutation::SetFormat(set_format::SetFormat { format: SemioAudioFormat::Float32 }),
            SemioAudioMutation::InsertChannel(insert_channel::InsertChannel { index: 1, channel: sample_channel(9.0) }),
            SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index: 1 }),
            SemioAudioMutation::SetChannelSamples(set_channel_samples::SetChannelSamples { index: 0, samples: vec![0.25, 0.5, 0.75] }),
            SemioAudioMutation::InsertTag(insert_tag::InsertTag { index: 0, tag: SemioAudioTag { key: "artist".into(), value: "a".into() } }),
            SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index: 0 }),
            SemioAudioMutation::SetTagValue(set_tag_value::SetTagValue { index: 0, value: "changed".into() }),
        ]
    }

    /// 🧪️ `mutation_diff_law`: every variant's `diff()` matches what `apply_semio_audio_mutation`
    /// returns.
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = base_snapshot();
        for mutation in all_variants(&base) {
            let mut snap = base.clone();
            let returned_diff = apply_semio_audio_mutation(&mut snap, &mutation);
            let expected_diff = mutation.diff(&base);
            assert_eq!(returned_diff, expected_diff, "returned diff must equal mutation.diff(base) for {mutation:?}");
            assert_eq!(
                snap,
                <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(expected_diff.diff(), &base).expect("apply must succeed for a well-formed fixture"),
                "apply_semio_audio_mutation must match diff.diff().apply(base) for {mutation:?}"
            );
        }
    }

    /// 🧪️ `inverse_law` (mutation-level): every variant round-trips.
    #[semio_framework_async_macros::async_test]
    async fn mutation_apply_inverse_round_trips_every_variant() {
        let base = base_snapshot();
        for mutation in all_variants(&base) {
            round_trips(&base, mutation);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_channel_out_of_range_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_semio_audio_mutation(&mut snap, &SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index: 99 }));
        assert_eq!(snap, base);
    }

    /// 🧪️ `kinds_match_the_enum_and_the_catalog`: `KINDS` names every declared variant, in the
    /// declaration order `variant_ordinal` assigns and the spelling `print_audio_mutation` emits,
    /// and every one of those names also appears in the committed oracle manifest's catalog. The
    /// bijection against `all_variants` is what makes a newly added variant fail here instead of
    /// silently shrinking the vocabulary the `mutate-semio-audio` case claims to cover.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        assert_eq!(KINDS, &OP_KEYWORDS[..], "KINDS must be exactly the op keyword table — one kebab-case name per declared variant, in declaration order");
        let base = base_snapshot();
        let mut seen = vec![false; KINDS.len()];
        for mutation in all_variants(&base) {
            let ordinal = variant_ordinal(&mutation) as usize;
            assert!(!seen[ordinal], "ordinal {ordinal} is represented twice — all_variants must carry exactly one case per declared variant");
            seen[ordinal] = true;
            assert_eq!(KINDS[ordinal], print_audio_mutation(&mutation).split(' ').next().unwrap_or_default(), "KINDS[{ordinal}] must be the keyword {mutation:?} prints");
        }
        assert!(seen.iter().all(|hit| *hit), "every declared variant must be represented in all_variants");
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }

    /// 🧪️ `op_text_binary_roundtrip_law`: hand-rolled `OpText`/`OpBinary` round-trip over the
    /// full variant vocabulary.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
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

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📄set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📄set-snapshot/🧪️tests/rerates-to-48-khz-and-rewrites-the-right-channel/🦀️.rs"]
mod set_snapshot_rerates_to_48_khz_and_rewrites_the_right_channel;
//#endregion 🧪️FixtureCases
