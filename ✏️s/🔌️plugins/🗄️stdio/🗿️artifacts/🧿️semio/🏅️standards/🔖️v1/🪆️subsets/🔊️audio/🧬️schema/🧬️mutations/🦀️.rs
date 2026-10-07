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

use crate::standards::v1::subsets::audio::schema::diff::{self, SemioAudioChannelDiff, SemioAudioDiff};














use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
/// 🔧️ Unconditional — `impl protocol::OpBinary for SemioAudioMutation` below's `encode_op`/
/// `decode_op` are now real production code (binary upgrade, this wave), not test-only.
use protocol::{Mutation};

//#region 🔖️Mutations
#[path = "🎙️insert-channel/🦀️.rs"]
pub mod insert_channel;
#[path = "🏷️insert-tag/🦀️.rs"]
pub mod insert_tag;
#[path = "🔇remove-channel/🦀️.rs"]
pub mod remove_channel;
#[path = "✂️remove-tag/🦀️.rs"]
pub mod remove_tag;
#[path = "🌊set-channel-samples/🦀️.rs"]
pub mod set_channel_samples;
#[path = "💽set-format/🦀️.rs"]
pub mod set_format;
#[path = "🎚️set-sample-rate/🦀️.rs"]
pub mod set_sample_rate;
/// 📐️ Typed content mutation for `s.stdio.semio.audio`.
//#region 🔖️Leaves
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "💬set-tag-value/🦀️.rs"]
pub mod set_tag_value;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioAudioSnapshot, diff = SemioAudioDiff, schema = "SemioAudioMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioAudioMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
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
/// order — what the `🔊️mutate-semio-audio` case's completeness gate counts against and what
/// `../../🔣️oracle.json`'s catalog repeats. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this declaration honest.
pub const KINDS: &[&str] = &["set-snapshot", "set-sample-rate", "set-format", "insert-channel", "remove-channel", "set-channel-samples", "insert-tag", "remove-tag", "set-tag-value", "patch-snapshot"];
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
pub fn inverse_semio_audio_mutation(mutation: &SemioAudioMutation, base: &SemioAudioSnapshot) -> Result<Vec<SemioAudioMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioAudioMutation as Mutation<SemioAudioSnapshot>>::inverse(mutation, base)?

    })
}


//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioAudioMutation, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<SemioAudioDiff> {
    protocol::MutationOutcome::new(match this {
        SemioAudioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff::diff_set_snapshot(base, snapshot),
        SemioAudioMutation::PatchSnapshot(patch) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioAudioSnapshot, SemioAudioMutation>>::diff(patch, base),
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
        SemioAudioMutation::InsertTag(insert_tag::InsertTag { index, tag }) => {
            SemioAudioDiff { tags: Some(IndexedTripleDiff { added: vec![IndexAdded { index: (*index).min(base.tags.len()), item: tag.clone() }], ..Default::default() }), ..Default::default() }
        }
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
pub(crate) fn agg_inverse(this: &SemioAudioMutation, base: &SemioAudioSnapshot) -> Result<Vec<SemioAudioMutation>, semio_framework_value::ValueError> {
    Ok({
    vec![match this {
        SemioAudioMutation::SetSnapshot(_) => SemioAudioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        SemioAudioMutation::PatchSnapshot(patch) => return Ok(<patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioAudioSnapshot, SemioAudioMutation>>::inverse(patch, base)?),
        SemioAudioMutation::SetSampleRate(_) => SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: base.sample_rate }),
        SemioAudioMutation::SetFormat(_) => SemioAudioMutation::SetFormat(set_format::SetFormat { format: base.format }),
        SemioAudioMutation::InsertChannel(insert_channel::InsertChannel { index, .. }) => SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index: (*index).min(base.channels.len()) }),
        SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index }) => match base.channels.get(*index) {
            Some(c) => SemioAudioMutation::InsertChannel(insert_channel::InsertChannel { index: *index, channel: c.clone() }),
            None => return Ok(Vec::new()),
        },
        SemioAudioMutation::SetChannelSamples(set_channel_samples::SetChannelSamples { index, .. }) => match base.channels.get(*index) {
            Some(c) => SemioAudioMutation::SetChannelSamples(set_channel_samples::SetChannelSamples { index: *index, samples: c.samples.clone() }),
            None => return Ok(Vec::new()),
        },
        SemioAudioMutation::InsertTag(insert_tag::InsertTag { index, .. }) => SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index: (*index).min(base.tags.len()) }),
        SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index }) => match base.tags.get(*index) {
            Some(t) => SemioAudioMutation::InsertTag(insert_tag::InsertTag { index: *index, tag: t.clone() }),
            None => return Ok(Vec::new()),
        },
        SemioAudioMutation::SetTagValue(set_tag_value::SetTagValue { index, .. }) => match base.tags.get(*index) {
            Some(t) => SemioAudioMutation::SetTagValue(set_tag_value::SetTagValue { index: *index, value: t.value.clone() }),
            None => return Ok(Vec::new()),
        },
    }]

    })
}
//#endregion 🔖️MutationTrait

//#region OpCodecs












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
        SemioAudioMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/🎧️rerates/🦀️.rs"]
mod set_snapshot_rerates_to_48_khz_and_rewrites_the_right_channel;
//#endregion 🧪️FixtureCases

#[cfg(test)]
use protocol::{OpBinary,OpText};
