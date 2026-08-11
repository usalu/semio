//! 🧬️ SemioModelMutation — 🚧 scaffolded by W1b: a single `SetSnapshot` full-replace mutation
//! (genuinely implements `protocol::Mutation`). W2 replaces this with the full named-variant
//! vocabulary (per-field mutations, sparse `diff()`/`inverse()`), following the gif 89a / docx
//! precedent.

use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::artifacts::semio::standards::v1::subsets::model::schema::diff::SemioModelDiff;
use protocol::Mutation;
#[cfg(test)]
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioModelMutation {
    #[default]
    NoMutation,
    /// 🚧 Full-snapshot replace — the only variant until W2's per-field vocabulary lands.
    SetSnapshot { snapshot: SemioModelSnapshot },
}

impl Mutation<SemioModelSnapshot> for SemioModelMutation {
    type Diff = SemioModelDiff;

    fn diff(&self, _base: &SemioModelSnapshot) -> Self::Diff {
        match self {
            SemioModelMutation::NoMutation => SemioModelDiff::default(),
            SemioModelMutation::SetSnapshot { snapshot } => SemioModelDiff { replacement: Some(snapshot.clone()) },
        }
    }

    fn inverse(&self, base: &SemioModelSnapshot) -> Vec<Self> {
        match self {
            SemioModelMutation::NoMutation => vec![SemioModelMutation::NoMutation],
            SemioModelMutation::SetSnapshot { .. } => vec![SemioModelMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and the set-snapshot leaf).
pub fn apply_semio_model_mutation(snapshot: &mut SemioModelSnapshot, mutation: &SemioModelMutation) -> SemioModelDiff {
    let diff = <SemioModelMutation as Mutation<SemioModelSnapshot>>::diff(mutation, snapshot);
    *snapshot = <SemioModelDiff as protocol::MutationDiff<SemioModelSnapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Mutation

//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary` — 🚧 scaffolded by W1b: plain `serde_json` round-trip of
/// the whole enum (one line of compact JSON per op), the same "JSON-pack passthrough" honesty
/// boundary the subset's own `ArtifactPack` impl already uses (see that file's doc comment).
/// Deliberately NOT `#[derive(dsl::DslOps)]` + `#[dsl(block)]` (the grammar/hand-rolled-op-triple
/// path every OTHER artifact's real mutation vocabulary uses) — that path requires the embedded
/// snapshot type to itself implement `dsl::DslField` (via `dsl::DslRecord`), which is real work
/// spanning every nested type in the snapshot tree and squarely W2's job, not a wiring fix. W2
/// replaces this whole region when it replaces `SetSnapshot` with the real per-field vocabulary.
impl protocol::OpText for SemioModelMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl protocol::OpBinary for SemioModelMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
}
//#endregion OpCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ mutation_diff_law + inverse_law: `mutation.diff(base).apply(base) == target`, and
    /// applying the inverse mutation restores `base`.
    #[test]
    fn mutation_diff_law_set_snapshot_matches_diff() {
        let base = SemioModelSnapshot::default();
        let mut next = SemioModelSnapshot::default();
        next.schema = format!("{}-mutated", base.schema);
        let mutation = SemioModelMutation::SetSnapshot { snapshot: next.clone() };
        let diff = <SemioModelMutation as Mutation<SemioModelSnapshot>>::diff(&mutation, &base);
        assert_eq!(<SemioModelDiff as protocol::MutationDiff<SemioModelSnapshot>>::apply(&diff, &base), next);
        let inv = <SemioModelMutation as Mutation<SemioModelSnapshot>>::inverse(&mutation, &base);
        assert_eq!(inv.len(), 1);
        let mut round = next.clone();
        let _ = apply_semio_model_mutation(&mut round, &inv[0]);
        assert_eq!(round, base);
    }
    /// 🧪️ op_text_binary_roundtrip_law: handcrafted `OpText`/`OpBinary` JSON round-trip.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = SemioModelSnapshot::default();
        for m in [SemioModelMutation::NoMutation, SemioModelMutation::SetSnapshot { snapshot: base.clone() }] {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioModelMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioModelMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
}
//#endregion 🔖️Tests
