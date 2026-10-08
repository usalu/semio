//! 🧬️ WavMutation — the real per-field mutation vocabulary over `WavSnapshot`'s three
//! top-level fields (`fmt`/`data`/`other_chunks`) and the sample lane's splices.

use crate::standards::riff_pcm::subsets::any::schema::diff::{data_len, data_slice, diff_set_data, diff_set_fmt, diff_set_other_chunks, sparse_against, WavDiff, WavSplice};
use crate::standards::riff_pcm::subsets::any::schema::snapshot::{validate_wav_serialization, RiffChunk, WavChunkRef, WavData, WavFmt, WavSnapshot};
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
#[path = "🧷️set-pad-bytes/🦀️.rs"]
pub mod set_pad_bytes;
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
    /// 🧷️ Sets the RIFF alignment bytes the `fmt ` and `data` chunks carry after an odd payload.
    SetPadBytes(set_pad_bytes::SetPadBytes),
}

/// 🦠️ Kebab-case spelling of every `WavMutation` variant — the exhaustive vocabulary the mutation
/// oracle catalog (`../../🔣️oracle.json`) is measured against. Order matches the enum.
pub const KINDS: &[&str] = &["set-fmt", "set-data", "patch-data", "set-other-chunks", "set-pad-bytes"];




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
