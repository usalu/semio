//! 🧬️ WavMutation — the real per-field mutation vocabulary over `WavSnapshot`'s three
//! top-level fields (`fmt`/`data`/`other_chunks`), plus `SetSnapshot` for full replace.

use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::diff::{diff_set_data, diff_set_fmt, diff_set_other_chunks, diff_set_snapshot, WavDiff};
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::{RiffChunk, WavData, WavFmt, WavSnapshot};
use protocol::Mutation;
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum WavMutation {
    #[default]
    NoMutation,
    /// 🔁️ Full-snapshot replace.
    SetSnapshot { snapshot: WavSnapshot },
    /// 🎚️ Replaces the `fmt ` chunk's typed fields wholesale.
    SetFmt { fmt: WavFmt },
    /// 🔊️ Replaces the typed sample data wholesale (may also change `WavData`'s variant, e.g.
    /// `Pcm16` → `Float32`, mirroring a real re-encode).
    SetData { data: WavData },
    /// 📎️ Replaces the verbatim-retained non-`fmt `/`data` chunk list wholesale.
    SetOtherChunks { chunks: Vec<RiffChunk> },
}

impl Mutation<WavSnapshot> for WavMutation {
    type Diff = WavDiff;

    async fn diff(&self, base: &WavSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            WavMutation::NoMutation => WavDiff::default(),
            WavMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot).await,
            WavMutation::SetFmt { fmt } => diff_set_fmt(fmt.clone()).await,
            WavMutation::SetData { data } => diff_set_data(data.clone()).await,
            WavMutation::SetOtherChunks { chunks } => diff_set_other_chunks(chunks.clone()).await,
        }).await
    }

    async fn inverse(&self, base: &WavSnapshot) -> Vec<Self> {
        match self {
            WavMutation::NoMutation => vec![WavMutation::NoMutation],
            WavMutation::SetSnapshot { .. } => vec![WavMutation::SetSnapshot { snapshot: base.clone() }],
            WavMutation::SetFmt { .. } => vec![WavMutation::SetFmt { fmt: base.fmt.clone() }],
            WavMutation::SetData { .. } => vec![WavMutation::SetData { data: base.data.clone() }],
            WavMutation::SetOtherChunks { .. } => vec![WavMutation::SetOtherChunks { chunks: base.other_chunks.clone() }],
        }
    }
}

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (the diff is the single
/// semantics source — never apply-and-capture).
pub async fn apply_wav_mutation(snapshot: &mut WavSnapshot, mutation: &WavMutation) -> protocol::MutationOutcome<WavDiff> {
    let outcome = <WavMutation as Mutation<WavSnapshot>>::diff(mutation, snapshot).await;
    match protocol::MutationDiff::apply(outcome.diff().await, snapshot).await {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).await.absorb_messages(outcome.messages().await.to_vec()).await,
    }
}
//#endregion 🔖️Mutation

//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary` via plain `serde_json` (one line of compact JSON per op) —
/// deliberately NOT `#[derive(dsl::DslOps)]`: `WavData` is a data-carrying enum embedded in
/// `SetData`'s payload, the same shape `f6-final-summary.md` §4.4 documents as structurally
/// unbindable by the derive machinery today (no generic/enum-payload `DslField` bridge). This is
/// a SEPARATE wire format from the subset's own `ArtifactDsl`/`ArtifactPack` envelope (which
/// wraps real RIFF/WAVE bytes, see that file's doc comment) — an op is always plain JSON here.
impl OpText for WavMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    async fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl OpBinary for WavMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
}
//#endregion OpCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    async fn base_snapshot() -> WavSnapshot {
        WavSnapshot { data: WavData::Pcm16(vec![10, -10, 5]), ..WavSnapshot::default() }
    }

    async fn variants(base: &WavSnapshot) -> Vec<WavMutation> {
        vec![
            WavMutation::NoMutation,
            WavMutation::SetSnapshot { snapshot: WavSnapshot { fmt: WavFmt { sample_rate: 48000, ..base.fmt.clone() }, ..base.clone() } },
            WavMutation::SetFmt { fmt: WavFmt { channels: 2, ..WavFmt::default() } },
            WavMutation::SetData { data: WavData::Float32(vec![0.25, -0.25]) },
            WavMutation::SetOtherChunks { chunks: vec![RiffChunk { fourcc: "fact".into(), data: vec![1, 2] }] },
        ]
    }

    //#region mutation_diff_law
    /// 🧪️ `mutation.diff(base).diff().apply(base) == apply_wav_mutation(base, mutation)`.
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law_every_variant() {
        let base = base_snapshot();
        for m in variants(&base) {
            let mut via_apply = base.clone();
            let returned = apply_wav_mutation(&mut via_apply, &m);
            let direct = m.diff(&base);
            assert_eq!(direct, returned, "diff mismatch for {m:?}");
            assert_eq!(direct.diff().apply(&base).unwrap(), via_apply, "apply mismatch for {m:?}");
        }
    }
    //#endregion mutation_diff_law

    //#region inverse_law
    /// 🧪️ Applying the inverse mutation restores base, at both the mutation and diff levels.
    #[semio_framework_async_macros::async_test]
    async fn inverse_law_mutation_and_diff_level() {
        let base = base_snapshot();
        for m in variants(&base) {
            let mut round = base.clone();
            apply_wav_mutation(&mut round, &m);
            for inv in m.inverse(&base) {
                apply_wav_mutation(&mut round, &inv);
            }
            assert_eq!(round, base, "mutation-level inverse failed for {m:?}");

            let d = m.diff(&base);
            let applied = d.diff().apply(&base).unwrap();
            let undone = d.diff().inverse(&base).apply(&applied).unwrap();
            assert_eq!(undone, base, "diff-level inverse failed for {m:?}");
        }
    }
    //#endregion inverse_law

    //#region op_text_binary_roundtrip_law
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        for m in variants(&base) {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = WavMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = WavMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
    //#endregion op_text_binary_roundtrip_law
}
//#endregion 🔖️Tests
