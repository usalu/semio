//! 🧬️ WavMutation — the real per-field mutation vocabulary over `WavSnapshot`'s three
//! top-level fields (`fmt`/`data`/`other_chunks`) and the sample lane's splices.

use crate::standards::riff_pcm::subsets::any::schema::diff::{data_len, data_slice, diff_set_data, diff_set_fmt, diff_set_other_chunks, sparse_against, WavDiff, WavSplice};
use crate::standards::riff_pcm::subsets::any::schema::snapshot::{validate_wav_serialization, RiffChunk, WavChunkRef, WavData, WavFmt, WavSnapshot};
use protocol::command::DiffAlgebra;
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
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none — and `no` is not an
/// approved semantic verb.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = WavSnapshot, diff = WavDiff, schema = "WavMutation")]
pub enum WavMutation {
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
pub const KINDS: &[&str] = &["set-fmt", "set-data", "patch-data", "set-other-chunks"];

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (the diff is the single
/// semantics source — never apply-and-capture).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_wav_mutation(snapshot: &mut WavSnapshot, mutation: &WavMutation) -> protocol::MutationOutcome<WavDiff> {
    let outcome = <WavMutation as Mutation<WavSnapshot>>::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#region 🔖️Net
/// 🧮️ The leaves that carry `base` to exactly `next`: the format, then the sample lane (one splice per changed span, or the whole lane when its kind changed), then the
/// chunk list with its exact order. The pad bytes have no leaf, so a `next` that changes them answers a list the exact replay refuses.
pub fn net_mutations(base: &WavSnapshot, next: &WavSnapshot) -> Vec<WavMutation> {
    let diff = WavDiff::between(base, next);
    let mut leaves = Vec::new();
    if let Some(fmt) = diff.fmt {
        leaves.push(WavMutation::SetFmt(set_fmt::SetFmt { fmt }));
    }
    match diff.data {
        Some(data) => leaves.push(WavMutation::SetData(set_data::SetData { data })),
        None => leaves.extend(diff.data_splices.into_iter().map(|splice| WavMutation::PatchData(patch_data::PatchData { index: splice.index, remove_count: splice.remove, data: splice.insert, move_to: None }))),
    }
    if diff.other_chunks.is_some() || diff.chunk_order.is_some() {
        leaves.push(WavMutation::SetOtherChunks(set_other_chunks::SetOtherChunks { chunks: next.other_chunks.clone(), chunk_order: Some(next.chunk_order.clone()) }));
    }
    leaves
}
//#endregion 🔖️Net

//#endregion 🔖️Mutation

//#region OpCodecs





//#endregion OpCodecs

//#endregion 🔖️MutationTrait

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

//#region 🧪️FixtureCases
//#endregion 🧪️FixtureCases

#[cfg(test)]
use protocol::{OpBinary,OpText};
