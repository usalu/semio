//! 🧬️ SemioVideoMutation — video mutation dispatch. Every variant's `diff()` is handcrafted
//! (never apply-and-capture) and every variant's `inverse()` is handcrafted, index-aware.
//!
//! `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one
//! leaf payload, and its sentinel verb `no` is not in `APPROVED_VERBS` — see
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`. Every variant is now a
//! tuple variant wrapping its own mutation leaf (`./*/🦀️.rs`), and this file's `agg_diff`/
//! `agg_inverse` carry the handcrafted semantics every leaf's `MutationKind` impl delegates back to.



use crate::standards::v1::subsets::video::schema::diff::{diff_insert_sample, diff_insert_stream, diff_remove_sample, diff_remove_stream, diff_set_sample_data, diff_set_sample_flags, diff_set_snapshot, diff_set_stream_meta, SemioVideoDiff};

















use crate::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind};

use protocol::{Mutation};

//#region 🔖️Mutations
#[path = "➕️insert-sample/🦀️.rs"]
pub mod insert_sample;
#[path = "🎥insert-stream/🦀️.rs"]
pub mod insert_stream;
#[path = "🚮remove-sample/🦀️.rs"]
pub mod remove_sample;
#[path = "🗑️remove-stream/🦀️.rs"]
pub mod remove_stream;
#[path = "📀set-sample-data/🦀️.rs"]
pub mod set_sample_data;
#[path = "🚩set-sample-flags/🦀️.rs"]
pub mod set_sample_flags;
/// 📐️ Typed content mutation for `stdio.semio.video`. Beyond the baseline `SetSnapshot`, this
/// addresses `streams` by index and, within a stream, `samples` by index — the same index-only
/// addressing scheme the diff grammar uses (neither collection carries a spec-mandated key). No
/// `#[derive(dsl::DslOps)]` attempted (this ticket's own instruction: hand-roll all op codecs) —
/// `SetSnapshot{snapshot: SemioVideoSnapshot}` alone would hit the same
/// `Vec<SemioVideoStream>`-of-`Vec<SemioVideoSample>` nesting the diff side's own doc comment
/// documents as blocking a derive attempt; `OpText`/`OpBinary` are hand-rolled below instead.
//#region 🔖️Leaves
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "📋set-stream-meta/🦀️.rs"]
pub mod set_stream_meta;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioVideoSnapshot, diff = SemioVideoDiff, schema = "SemioVideoMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioVideoMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
    /// ➕️ Inserts `stream` at `index` (FINAL state).
    InsertStream(insert_stream::InsertStream),
    /// ➖️ Removes the stream at `index` (BASE-state index).
    RemoveStream(remove_stream::RemoveStream),
    /// ✍️ Sets the container-level metadata (kind/codec/dimensions/rate) of the stream at `index`.
    SetStreamMeta(set_stream_meta::SetStreamMeta),
    /// ➕️ Inserts `sample` at `index` within the stream at `stream_index` (FINAL state).
    InsertSample(insert_sample::InsertSample),
    /// ➖️ Removes the sample at `index` (BASE-state index) within `stream_index`.
    RemoveSample(remove_sample::RemoveSample),
    /// ✍️ Replaces one sample's opaque payload.
    SetSampleData(set_sample_data::SetSampleData),
    /// 🏳️ Sets one sample's `pts`/`key` flags.
    SetSampleFlags(set_sample_flags::SetSampleFlags),
}

/// 🏷️ The declared kebab-case mutation vocabulary of `s.stdio.semio.video`, in enum declaration
/// order — what the `🎥️mutate-semio-video` case's completeness gate counts against and what
/// `../../🔣️oracle.json`'s catalog repeats. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this declaration honest.
pub const KINDS: &[&str] = &["set-snapshot", "insert-stream", "remove-stream", "set-stream-meta", "insert-sample", "remove-sample", "set-sample-data", "set-sample-flags", "patch-snapshot"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` -- the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_video_mutation(snapshot: &mut SemioVideoSnapshot, mutation: &SemioVideoMutation) -> protocol::MutationOutcome<SemioVideoDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Free-function face of [`SemioVideoMutation`]'s own `protocol::Mutation::inverse`. `Mutation` is
/// declared by the os-kernel, which is an INTERNAL dependency of this plugin (aliased `protocol` in
/// `🦀️.rs`) and is therefore not nameable by a consumer that links only this crate — a
/// generated test host being the concrete case. Paired with [`apply_semio_video_mutation`] it makes the
/// undo law reachable without importing a trait the caller cannot name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_video_mutation(mutation: &SemioVideoMutation, base: &SemioVideoSnapshot) -> Result<Vec<SemioVideoMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioVideoMutation as Mutation<SemioVideoSnapshot>>::inverse(mutation, base)?

    })
}


//#endregion 🔖️Apply

//#region 🔖️Helpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stream_at(base: &SemioVideoSnapshot, index: usize) -> Option<&SemioVideoStream> {
    base.streams.get(index)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sample_at(base: &SemioVideoSnapshot, stream_index: usize, index: usize) -> Option<&SemioVideoSample> {
    base.streams.get(stream_index)?.samples.get(index)
}
//#endregion 🔖️Helpers

//#region 🔖️MutationTrait
/// ↩️ An index that no longer exists in `base` has nothing to restore, so those arms return the
/// empty inverse rather than a sentinel no-op mutation — the convention this migration adopted once
/// `NoMutation` stopped being an available payload.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioVideoMutation, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<SemioVideoDiff> {
    protocol::MutationOutcome::new(match this {
        SemioVideoMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        SemioVideoMutation::PatchSnapshot(patch) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioVideoSnapshot, SemioVideoMutation>>::diff(patch, base),
        SemioVideoMutation::InsertStream(insert_stream::InsertStream { index, stream }) => diff_insert_stream(*index, stream.clone()),
        SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index }) => diff_remove_stream(*index),
        SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index, kind, codec, width, height, rate }) => match stream_at(base, *index) {
            Some(old) => diff_set_stream_meta(old, *index, *kind, codec, *width, *height, *rate),
            None => SemioVideoDiff::default(),
        },
        SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index, index, sample }) => diff_insert_sample(*stream_index, *index, sample.clone()),
        SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index, index }) => diff_remove_sample(*stream_index, *index),
        SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index, index, data }) => match sample_at(base, *stream_index, *index) {
            Some(old) => diff_set_sample_data(old, *stream_index, *index, data.clone()),
            None => SemioVideoDiff::default(),
        },
        SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags { stream_index, index, pts, key }) => match sample_at(base, *stream_index, *index) {
            Some(old) => diff_set_sample_flags(old, *stream_index, *index, *pts, *key),
            None => SemioVideoDiff::default(),
        },
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioVideoMutation, base: &SemioVideoSnapshot) -> Result<Vec<SemioVideoMutation>, semio_framework_value::ValueError> {
    Ok({
    vec![match this {
        SemioVideoMutation::SetSnapshot(_) => SemioVideoMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        SemioVideoMutation::PatchSnapshot(patch) => return Ok(<patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioVideoSnapshot, SemioVideoMutation>>::inverse(patch, base)?),
        SemioVideoMutation::InsertStream(insert_stream::InsertStream { index, .. }) => SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index: *index }),
        SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index }) => match stream_at(base, *index) {
            Some(stream) => SemioVideoMutation::InsertStream(insert_stream::InsertStream { index: *index, stream: stream.clone() }),
            None => return Ok(Vec::new()),
        },
        SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index, .. }) => match stream_at(base, *index) {
            Some(stream) => SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index: *index, kind: stream.kind, codec: stream.codec.clone(), width: stream.width, height: stream.height, rate: stream.rate }),
            None => return Ok(Vec::new()),
        },
        SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index, index, .. }) => SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index: *stream_index, index: *index }),
        SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index, index }) => match sample_at(base, *stream_index, *index) {
            Some(sample) => SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index: *stream_index, index: *index, sample: sample.clone() }),
            None => return Ok(Vec::new()),
        },
        SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index, index, .. }) => match sample_at(base, *stream_index, *index) {
            Some(sample) => SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index: *stream_index, index: *index, data: sample.data.clone() }),
            None => return Ok(Vec::new()),
        },
        SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags { stream_index, index, .. }) => match sample_at(base, *stream_index, *index) {
            Some(sample) => SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags { stream_index: *stream_index, index: *index, pts: sample.pts, key: sample.key }),
            None => return Ok(Vec::new()),
        },
    }]

    })
}
//#endregion 🔖️MutationTrait

//#region OpCodecs














//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 Representative `SemioVideoMutation` cases (one per variant, `pub(crate)` module-scope) for
/// the conformance-law tests — delegates to the existing test module's own `sample_mutations()`
/// (byte-identical) rather than keep an independent copy, same dedupe flow's/mesh's own waves
/// perform.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioVideoMutation> {
    tests::sample_mutations()
}
//#endregion 🔖️Demo

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/⏱️retimes/🦀️.rs"]
mod set_snapshot_retimes_the_track_and_promotes_a_sample_to_a_keyframe;
//#endregion 🧪️FixtureCases

#[cfg(test)]
use protocol::{OpBinary,OpText};
