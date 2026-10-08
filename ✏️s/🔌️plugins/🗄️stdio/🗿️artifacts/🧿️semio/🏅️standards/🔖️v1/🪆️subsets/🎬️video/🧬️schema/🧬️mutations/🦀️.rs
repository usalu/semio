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
#[path = "📋set-stream-meta/🦀️.rs"]
pub mod set_stream_meta;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioVideoSnapshot, diff = SemioVideoDiff, schema = "SemioVideoMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioVideoMutation {
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
pub const KINDS: &[&str] = &["insert-stream", "remove-stream", "set-stream-meta", "insert-sample", "remove-sample", "set-sample-data", "set-sample-flags", "patch-snapshot"];
//#endregion 🔖️Mutations

/// 🧮️ Pure diff face of [`Mutation::diff`], named only in this subset's own reachable types (`protocol` is a private
/// `extern crate` alias, so an owner-root test adapter cannot bring the `Mutation` trait into scope).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_semio_video_mutation(mutation: &SemioVideoMutation, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<SemioVideoDiff> {
    <SemioVideoMutation as protocol::Mutation<SemioVideoSnapshot>>::diff(mutation, base)
}


/// ↩️ Free-function face of [`SemioVideoMutation`]'s own `protocol::Mutation::inverse`. `Mutation` is
/// declared by the os-kernel, which is an INTERNAL dependency of this plugin (aliased `protocol` in
/// `🦀️.rs`) and is therefore not nameable by a consumer that links only this crate — a
/// generated test host being the concrete case. Paired with `diff_semio_*_mutation` it makes the
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

#[cfg(test)]
use protocol::{OpBinary,OpText};
