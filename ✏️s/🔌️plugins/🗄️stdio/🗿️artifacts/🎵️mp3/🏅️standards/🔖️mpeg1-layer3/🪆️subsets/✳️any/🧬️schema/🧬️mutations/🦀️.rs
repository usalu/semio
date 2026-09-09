//! 🧬️ Mp3Mutation — the real per-field mutation vocabulary over `Mp3Snapshot`'s three
//! top-level fields (`id3v2`/`frames`/`id3v1`), plus `SetSnapshot` for full replace.

//#region 🔖️Leaves
#[path = "🎼️set-frames/🦀️.rs"]
pub mod set_frames;
#[path = "🔖️set-id3v1/🦀️.rs"]
pub mod set_id3v1;
#[path = "🏷️set-id3v2/🦀️.rs"]
pub mod set_id3v2;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
//#endregion 🔖️Leaves

use crate::standards::mpeg1_layer3::subsets::any::schema::diff::{diff_set_frames, diff_set_id3v1, diff_set_id3v2, diff_set_snapshot, Mp3Diff};
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3Snapshot};
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
    /// 🏷️ Kebab-case kind spelling — the exact vocabulary `../../🔣️oracle.json`'s
    /// `mutationCatalogs[].kinds` declares and `🎛️mutate-mp3-mpeg1-layer3`'s Scenario Outline row ids
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests
