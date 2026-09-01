//! 🧬️ WavMutation — the real per-field mutation vocabulary over `WavSnapshot`'s three
//! top-level fields (`fmt`/`data`/`other_chunks`), plus `SetSnapshot` for full replace.

use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::diff::{diff_set_data, diff_set_fmt, diff_set_other_chunks, diff_set_snapshot, WavDiff};
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::{RiffChunk, WavData, WavFmt, WavSnapshot};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

//#region 🔖️Mutation
//#region 🔖️Leaves
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🎚set-fmt/🦀️.rs"]
pub mod set_fmt;
#[path = "🔊set-data/🦀️.rs"]
pub mod set_data;
#[path = "📎set-other-chunks/🦀️.rs"]
pub mod set_other_chunks;
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
/// oracle catalog (`../../🧪️oracle/🔣️.json`) is measured against. Order matches the enum.
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
mod tests {
    use super::*;
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> WavSnapshot {
        WavSnapshot { data: WavData::Pcm16(vec![10, -10, 5]), ..WavSnapshot::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn variants(base: &WavSnapshot) -> Vec<WavMutation> {
        vec![
            WavMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: WavSnapshot { fmt: WavFmt { sample_rate: 48000, ..base.fmt.clone() }, ..base.clone() } }),
            WavMutation::SetFmt(set_fmt::SetFmt { fmt: WavFmt { channels: 2, ..WavFmt::default() } }),
            WavMutation::SetData(set_data::SetData { data: WavData::Float32(vec![0.25, -0.25]) }),
            WavMutation::SetOtherChunks(set_other_chunks::SetOtherChunks { chunks: vec![RiffChunk { fourcc: "fact".into(), data: vec![1, 2] }] }),
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

    //#region kinds_matches_enum_and_manifest
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn kind_of(m: &WavMutation) -> &'static str {
        match m {
            WavMutation::SetSnapshot(_) => "set-snapshot",
            WavMutation::SetFmt(_) => "set-fmt",
            WavMutation::SetData(_) => "set-data",
            WavMutation::SetOtherChunks(_) => "set-other-chunks",
        }
    }

    /// 🧪️ `KINDS` must name exactly the enum's own variants (the match above is exhaustive, so a
    /// variant added without updating `kind_of` fails to compile) AND every one of them must appear
    /// in the mutation catalog's declared `kinds` — the framework never parses Rust, so this test is
    /// what keeps the manifest honest. The check is containment, not equality: the manifest also
    /// declares `no-mutation`, the identity scenario the `🥒️.feature` still exercises against the
    /// independent oracle even though `NoMutation` is no longer an enum variant (`no` is not an
    /// approved semantic verb for `#[derive(dsl::Mutations)]`) — the same split
    /// `mutate-mp3-mpeg1-layer3`'s own `kinds_match_the_committed_catalog` makes for its sibling
    /// vocabulary.
    #[semio_framework_async_macros::async_test]
    async fn kinds_matches_enum_variants_and_manifest() {
        let base = base_snapshot();
        let mut from_enum: Vec<&str> = variants(&base).iter().map(kind_of).collect();
        from_enum.sort_unstable();
        from_enum.dedup();
        let mut from_const = KINDS.to_vec();
        from_const.sort_unstable();
        assert_eq!(from_const, from_enum, "KINDS must name exactly the enum's variants");

        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "the oracle catalog manifest must declare kind {kind:?}");
        }
    }
    //#endregion kinds_matches_enum_and_manifest

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

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📄set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `📦️glue.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📄set-snapshot/🧪️tests/resamples-to-16-khz-and-doubles-the-pcm16-amplitude/🦀️component.rs"]
mod set_snapshot_resamples_to_16_khz_and_doubles_the_pcm16_amplitude;
//#endregion 🧪️FixtureCases
