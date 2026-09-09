//! 🧬️ WavMutation — the real per-field mutation vocabulary over `WavSnapshot`'s three
//! top-level fields (`fmt`/`data`/`other_chunks`), plus `SetSnapshot` for full replace.

use crate::standards::riff_pcm::subsets::any::schema::diff::{diff_set_data, diff_set_fmt, diff_set_other_chunks, diff_set_snapshot, WavDiff};
use crate::standards::riff_pcm::subsets::any::schema::snapshot::{RiffChunk, WavData, WavFmt, WavSnapshot};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

//#region 🔖️Mutation
//#region 🔖️Leaves
#[path = "🔊️set-data/🦀️.rs"]
pub mod set_data;
#[path = "🎚️set-fmt/🦀️.rs"]
pub mod set_fmt;
#[path = "📎️set-other-chunks/🦀️.rs"]
pub mod set_other_chunks;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
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
    /// 🎚️ Replaces the `fmt ` chunk's typed fields wholesale.
    SetFmt(set_fmt::SetFmt),
    /// 🔊️ Replaces the typed sample data wholesale (may also change `WavData`'s variant, e.g.
    /// `Pcm16` → `Float32`, mirroring a real re-encode).
    SetData(set_data::SetData),
    /// 📎️ Replaces the verbatim-retained non-`fmt `/`data` chunk list wholesale.
    SetOtherChunks(set_other_chunks::SetOtherChunks),
}

/// 🦠️ Kebab-case spelling of every `WavMutation` variant — the exhaustive vocabulary the mutation
/// oracle catalog (`../../🔣️oracle.json`) is measured against. Order matches the enum.
pub const KINDS: &[&str] = &["set-snapshot", "set-fmt", "set-data", "set-other-chunks"];

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
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Mutation

//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary` via `pack::json` (one line of compact JSON per op) —
/// deliberately NOT `#[derive(dsl::DslOps)]`: `WavData` is a data-carrying enum embedded in
/// `SetData`'s payload, the same shape `f6-final-summary.md` §4.4 documents as structurally
/// unbindable by the derive machinery today (no generic/enum-payload `DslField` bridge). This is
/// a SEPARATE wire format from the subset's own `ArtifactDsl`/`ArtifactPack` envelope (which
/// wraps real RIFF/WAVE bytes, see that file's doc comment) — an op is always plain JSON here.
impl OpText for WavMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parsed = pack::parse_json(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
        <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(self)))
    }
}

impl OpBinary for WavMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(self))).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = pack::parse_json_bytes(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))?;
        <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
}
//#endregion OpCodecs

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &WavMutation, base: &WavSnapshot) -> protocol::MutationOutcome<WavDiff> {
    protocol::MutationOutcome::new(match this {
        WavMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        WavMutation::SetFmt(set_fmt::SetFmt { fmt }) => diff_set_fmt(fmt.clone()),
        WavMutation::SetData(set_data::SetData { data }) => diff_set_data(data.clone()),
        WavMutation::SetOtherChunks(set_other_chunks::SetOtherChunks { chunks }) => diff_set_other_chunks(chunks.clone()),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &WavMutation, base: &WavSnapshot) -> Vec<WavMutation> {
    vec![match this {
        WavMutation::SetSnapshot(_) => WavMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        WavMutation::SetFmt(_) => WavMutation::SetFmt(set_fmt::SetFmt { fmt: base.fmt.clone() }),
        WavMutation::SetData(_) => WavMutation::SetData(set_data::SetData { data: base.data.clone() }),
        WavMutation::SetOtherChunks(_) => WavMutation::SetOtherChunks(set_other_chunks::SetOtherChunks { chunks: base.other_chunks.clone() }),
    }]
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
#[path = "📸️set-snapshot/🧪️tests/🔊️resamples-to-16-a155c9/🦀️.rs"]
mod set_snapshot_resamples_to_16_khz_and_doubles_the_pcm16_amplitude;
//#endregion 🧪️FixtureCases
