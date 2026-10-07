//! 🧬️ WavMutation — the real per-field mutation vocabulary over `WavSnapshot`'s three
//! top-level fields (`fmt`/`data`/`other_chunks`), plus `SetSnapshot` for full replace.

use crate::standards::riff_pcm::subsets::any::schema::diff::{diff_set_data, diff_set_fmt, diff_set_other_chunks, diff_set_snapshot, WavDiff};
use crate::standards::riff_pcm::subsets::any::schema::snapshot::{validate_wav_serialization, RiffChunk, WavData, WavFmt, WavSnapshot};
use protocol::Mutation;


//#region 🔖️Mutation
//#region 🔖️Leaves
#[path = "🔊️set-data/🦀️.rs"]
pub mod set_data;
#[path = "🩹️patch-data/🦀️.rs"]
pub mod patch_data;
#[path = "🎚️set-fmt/🦀️.rs"]
pub mod set_fmt;
#[path = "📎️set-other-chunks/🦀️.rs"]
pub mod set_other_chunks;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none — and `no` is not an
/// approved semantic verb.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = WavSnapshot, diff = WavDiff, schema = "WavMutation")]
pub enum WavMutation {
    /// 🔁️ Full-snapshot replace.
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
    /// 🎚️ Replaces the `fmt ` chunk's typed fields wholesale.
    SetFmt(set_fmt::SetFmt),
    /// 🔊️ Replaces the typed sample data wholesale (may also change `WavData`'s variant, e.g.
    /// `Pcm16` → `Float32`, mirroring a real re-encode).
    SetData(set_data::SetData),
    /// 🩹️ Replaces, inserts, removes, or moves a bounded sample range without retaining the full data chunk in history.
    PatchData(patch_data::PatchData),
    /// 📎️ Replaces the verbatim-retained non-`fmt `/`data` chunk list wholesale.
    SetOtherChunks(set_other_chunks::SetOtherChunks),
}

/// 🦠️ Kebab-case spelling of every `WavMutation` variant — the exhaustive vocabulary the mutation
/// oracle catalog (`../../🔣️oracle.json`) is measured against. Order matches the enum.
pub const KINDS: &[&str] = &["set-snapshot", "patch-snapshot", "set-fmt", "set-data", "patch-data", "set-other-chunks"];

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (the diff is the single
/// semantics source — never apply-and-capture).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_wav_mutation(snapshot: &mut WavSnapshot, mutation: &WavMutation) -> protocol::MutationOutcome<WavDiff> {
    let outcome = <WavMutation as Mutation<WavSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion 🔖️Mutation

//#region OpCodecs





//#endregion OpCodecs

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &WavMutation, base: &WavSnapshot) -> protocol::MutationOutcome<WavDiff> {
    let outcome = match this {
        WavMutation::PatchSnapshot(payload) => protocol::MutationKind::diff(payload, base),
        WavMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot)),
        WavMutation::SetFmt(set_fmt::SetFmt { fmt }) => protocol::MutationOutcome::new(diff_set_fmt(fmt.clone())),
        WavMutation::SetData(set_data::SetData { data }) => protocol::MutationOutcome::new(diff_set_data(data.clone())),
        WavMutation::PatchData(payload) => patch_data::diff(payload, base),
        WavMutation::SetOtherChunks(set_other_chunks::SetOtherChunks { chunks }) => protocol::MutationOutcome::new(diff_set_other_chunks(base, chunks.clone())),
    };
    let Ok(candidate) = protocol::MutationDiff::apply(outcome.diff(), base) else { return outcome };
    if let Err(issue) = validate_wav_serialization(&candidate) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("{}: {}", issue.code, issue.message), issue.target).absorb_messages(outcome.messages().to_vec());
    }
    outcome
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &WavMutation, base: &WavSnapshot) -> Result<Vec<WavMutation>, semio_framework_value::ValueError> {
    Ok({
    vec![match this {
        WavMutation::PatchSnapshot(payload) => return Ok(protocol::MutationKind::inverse(payload, base)?),
        WavMutation::SetSnapshot(_) => WavMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        WavMutation::SetFmt(_) => WavMutation::SetFmt(set_fmt::SetFmt { fmt: base.fmt.clone() }),
        WavMutation::SetData(_) => WavMutation::SetData(set_data::SetData { data: base.data.clone() }),
        WavMutation::PatchData(payload) => return Ok(patch_data::inverse(payload, base)?),
        WavMutation::SetOtherChunks(_) => WavMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
    }]

    })
}
//#endregion 🔖️MutationTrait

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
#[path = "📸️set-snapshot/🧪️tests/🔊️resamples/🦀️.rs"]
mod set_snapshot_resamples_to_16_khz_and_doubles_the_pcm16_amplitude;
//#endregion 🧪️FixtureCases

#[cfg(test)]
use protocol::{OpBinary,OpText};
