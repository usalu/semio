//! 🧬️ Mp3Mutation — the real per-field mutation vocabulary over `Mp3Snapshot`'s three
//! top-level fields (`id3v2`/`frames`/`id3v1`), plus `SetSnapshot` for full replace.

//#region 🔖️Leaves
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🏷set-id3v2/🦀️.rs"]
pub mod set_id3v2;
#[path = "🎼set-frames/🦀️.rs"]
pub mod set_frames;
#[path = "🔖set-id3v1/🦀️.rs"]
pub mod set_id3v1;
//#endregion 🔖️Leaves

use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::diff::{diff_set_frames, diff_set_id3v1, diff_set_id3v2, diff_set_snapshot, Mp3Diff};
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3Snapshot};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

//#region 🔖️Mutation
/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = Mp3Snapshot, diff = Mp3Diff, schema = "Mp3Mutation")]
pub enum Mp3Mutation {
    /// 🔁️ Full-snapshot replace.
    SetSnapshot(set_snapshot::SetSnapshot),
    /// 🏷️ Sets (`Some`) or clears (`None`) the ID3v2 tag wholesale.
    SetId3v2(set_id3v2::SetId3v2),
    /// 🎼️ Replaces the MPEG frame sequence wholesale.
    SetFrames(set_frames::SetFrames),
    /// 🏷️ Sets (`Some`) or clears (`None`) the ID3v1 trailer wholesale.
    SetId3v1(set_id3v1::SetId3v1),
}

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (the diff is the single
/// semantics source — never apply-and-capture).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_mp3_mutation(snapshot: &mut Mp3Snapshot, mutation: &Mp3Mutation) -> protocol::MutationOutcome<Mp3Diff> {
    let outcome = <Mp3Mutation as Mutation<Mp3Snapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Mutation

//#region 🔖️Kinds
impl Mp3Mutation {
    /// 🏷️ Kebab-case kind spelling — the exact vocabulary `../../🧪️oracle/🔣️.json`'s
    /// `mutationCatalogs[].kinds` declares and `mutate-mp3-mpeg1-layer3`'s Scenario Outline row ids
    /// equal. Hand-matched rather than derived, so [`KINDS`] is checked against something with its
    /// own reason to be right; and exhaustive, so a variant added to the enum is a COMPILE error
    /// here rather than a silently uncatalogued kind.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn kind(&self) -> &'static str {
        match self {
            Mp3Mutation::SetSnapshot(_) => "set-snapshot",
            Mp3Mutation::SetId3v2(_) => "set-id3v2",
            Mp3Mutation::SetFrames(_) => "set-frames",
            Mp3Mutation::SetId3v1(_) => "set-id3v1",
        }
    }
}

/// 🏷️ Every declared kind, kebab-case, in the enum's own declaration order — mirrors the catalog's
/// `mutationCatalogs[].kinds` exactly.
pub const KINDS: &[&str] = &["set-snapshot", "set-id3v2", "set-frames", "set-id3v1"];
//#endregion 🔖️Kinds

//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary` via `pack::json` (one line of compact JSON per op) —
/// deliberately NOT `#[derive(dsl::DslOps)]`: `Mp3Frame`/`Id3v2Tag` embed nested collections of
/// named structs, the same generic-collection-diff shape `f6-final-summary.md` §4.4 documents as
/// needing a hand-rolled bridge. This is a SEPARATE wire format from the subset's own
/// `ArtifactDsl`/`ArtifactPack` envelope (which wraps real MP3 bytes, see that file's doc
/// comment) — an op is always plain JSON here.
impl OpText for Mp3Mutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parsed = pack::parse_json(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
        <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(self)))
    }
}

impl OpBinary for Mp3Mutation {
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
pub(crate) fn agg_diff(this: &Mp3Mutation, base: &Mp3Snapshot) -> protocol::MutationOutcome<Mp3Diff> {
    protocol::MutationOutcome::new(match this {
        Mp3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        Mp3Mutation::SetId3v2(set_id3v2::SetId3v2 { id3v2 }) => diff_set_id3v2(id3v2.clone()),
        Mp3Mutation::SetFrames(set_frames::SetFrames { frames }) => diff_set_frames(frames.clone()),
        Mp3Mutation::SetId3v1(set_id3v1::SetId3v1 { id3v1 }) => diff_set_id3v1(id3v1.clone()),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &Mp3Mutation, base: &Mp3Snapshot) -> Vec<Mp3Mutation> {
    vec![match this {
        Mp3Mutation::SetSnapshot(_) => Mp3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        Mp3Mutation::SetId3v2(_) => Mp3Mutation::SetId3v2(set_id3v2::SetId3v2 { id3v2: base.id3v2.clone() }),
        Mp3Mutation::SetFrames(_) => Mp3Mutation::SetFrames(set_frames::SetFrames { frames: base.frames.clone() }),
        Mp3Mutation::SetId3v1(_) => Mp3Mutation::SetId3v1(set_id3v1::SetId3v1 { id3v1: base.id3v1.clone() }),
    }]
}
//#endregion 🔖️MutationTrait

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Frame, Mp3FrameHeader};
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn frame() -> Mp3Frame {
        Mp3Frame {
            header: Mp3FrameHeader { mpeg_version_id: 3, layer: 1, protection_bit: true, bitrate_index: 9, sample_rate_index: 0, padding: false, private_bit: false, channel_mode: 3, mode_extension: 0, copyright: false, original: true, emphasis: 0 },
            payload: vec![0u8; 4],
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> Mp3Snapshot {
        Mp3Snapshot { frames: vec![frame()], ..Mp3Snapshot::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn variants(base: &Mp3Snapshot) -> Vec<Mp3Mutation> {
        vec![
            Mp3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Mp3Snapshot { frames: vec![frame(), frame()], ..base.clone() } }),
            Mp3Mutation::SetId3v2(set_id3v2::SetId3v2 { id3v2: Some(Id3v2Tag { major_version: 3, minor_version: 0, flags: 0, frames: vec![Id3Frame { id: "TIT2".into(), flags: 0, data: vec![0] }] }) }),
            Mp3Mutation::SetId3v2(set_id3v2::SetId3v2 { id3v2: None }),
            Mp3Mutation::SetFrames(set_frames::SetFrames { frames: vec![frame(), frame(), frame()] }),
            Mp3Mutation::SetId3v1(set_id3v1::SetId3v1 { id3v1: Some(Id3v1Tag { raw: vec![b'T', b'A', b'G'] }) }),
            Mp3Mutation::SetId3v1(set_id3v1::SetId3v1 { id3v1: None }),
        ]
    }

    /// 🧪️ Keeps the declaration honest, which nothing else can: the framework never parses Rust, so
    /// the CATALOG is what the contract gate counts against, and this is the only check that ties it
    /// to the enum. `variants()` already carries every declared variant, `kind()` is an exhaustive
    /// match, and the manifest is read as committed text — so a kind added to one of the three and
    /// not the others fails here. The sibling `KINDS` in `../../🧪️oracle/🦀️component.rs` mirrors
    /// this one from the oracle crate, which must never link this crate; it can only compare
    /// strings, whereas this test compares against real values.
    #[test]
    fn kinds_matches_every_variant_and_the_catalog() {
        let from_variants: std::collections::BTreeSet<&str> = variants(&base_snapshot()).iter().map(Mp3Mutation::kind).collect();
        let from_kinds: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
        assert_eq!(from_variants, from_kinds, "KINDS must equal every Mp3Mutation variant's kind()");
        assert_eq!(KINDS.len(), 4, "KINDS must list exactly the declared 4 kinds");
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "the oracle catalog manifest must declare kind {kind:?}");
        }
    }

    //#region mutation_diff_law
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law_every_variant() {
        let base = base_snapshot();
        for m in variants(&base) {
            let mut via_apply = base.clone();
            let returned = apply_mp3_mutation(&mut via_apply, &m);
            let direct = m.diff(&base);
            assert_eq!(direct, returned, "diff mismatch for {m:?}");
            assert_eq!(direct.diff().apply(&base).unwrap(), via_apply, "apply mismatch for {m:?}");
        }
    }
    //#endregion mutation_diff_law

    //#region inverse_law
    #[semio_framework_async_macros::async_test]
    async fn inverse_law_mutation_and_diff_level() {
        let base = base_snapshot();
        for m in variants(&base) {
            let mut round = base.clone();
            apply_mp3_mutation(&mut round, &m);
            for inv in m.inverse(&base) {
                apply_mp3_mutation(&mut round, &inv);
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
            let parsed = Mp3Mutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = Mp3Mutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
    //#endregion op_text_binary_roundtrip_law
}
//#endregion 🔖️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `📦️glue.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/retitles-the-id3v2-tit2-frame/🦀️component.rs"]
    mod tests_set_snapshot_retitles_the_id3v2_tit2_frame;
}
//#endregion 🧪️FixtureTests
