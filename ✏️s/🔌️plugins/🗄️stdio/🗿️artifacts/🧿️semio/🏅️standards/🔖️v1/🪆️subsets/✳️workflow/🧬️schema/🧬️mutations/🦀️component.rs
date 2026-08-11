//! 🧬️ SemioWorkflowMutation — 🚧 scaffolded by W1b: a single `SetSnapshot` full-replace mutation
//! (genuinely implements `protocol::Mutation`). W2 replaces this with the full named-variant
//! vocabulary (per-field mutations, sparse `diff()`/`inverse()`), following the gif 89a / docx
//! precedent.

use crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::SemioWorkflowSnapshot;
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::diff::SemioWorkflowDiff;
use protocol::Mutation;
#[cfg(test)]
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioWorkflowMutation {
    #[default]
    NoMutation,
    /// 🚧 Full-snapshot replace — the only variant until W2's per-field vocabulary lands.
    SetSnapshot { snapshot: SemioWorkflowSnapshot },
}

impl Mutation<SemioWorkflowSnapshot> for SemioWorkflowMutation {
    type Diff = SemioWorkflowDiff;

    fn diff(&self, _base: &SemioWorkflowSnapshot) -> Self::Diff {
        match self {
            SemioWorkflowMutation::NoMutation => SemioWorkflowDiff::default(),
            SemioWorkflowMutation::SetSnapshot { snapshot } => SemioWorkflowDiff { replacement: Some(snapshot.clone()) },
        }
    }

    fn inverse(&self, base: &SemioWorkflowSnapshot) -> Vec<Self> {
        match self {
            SemioWorkflowMutation::NoMutation => vec![SemioWorkflowMutation::NoMutation],
            SemioWorkflowMutation::SetSnapshot { .. } => vec![SemioWorkflowMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and the set-snapshot leaf).
pub fn apply_semio_workflow_mutation(snapshot: &mut SemioWorkflowSnapshot, mutation: &SemioWorkflowMutation) -> SemioWorkflowDiff {
    let diff = <SemioWorkflowMutation as Mutation<SemioWorkflowSnapshot>>::diff(mutation, snapshot);
    *snapshot = <SemioWorkflowDiff as protocol::MutationDiff<SemioWorkflowSnapshot>>::apply(&diff, snapshot);
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
impl protocol::OpText for SemioWorkflowMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl protocol::OpBinary for SemioWorkflowMutation {
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
        let base = SemioWorkflowSnapshot::default();
        let mut next = SemioWorkflowSnapshot::default();
        next.schema = format!("{}-mutated", base.schema);
        let mutation = SemioWorkflowMutation::SetSnapshot { snapshot: next.clone() };
        let diff = <SemioWorkflowMutation as Mutation<SemioWorkflowSnapshot>>::diff(&mutation, &base);
        assert_eq!(<SemioWorkflowDiff as protocol::MutationDiff<SemioWorkflowSnapshot>>::apply(&diff, &base), next);
        let inv = <SemioWorkflowMutation as Mutation<SemioWorkflowSnapshot>>::inverse(&mutation, &base);
        assert_eq!(inv.len(), 1);
        let mut round = next.clone();
        let _ = apply_semio_workflow_mutation(&mut round, &inv[0]);
        assert_eq!(round, base);
    }
    /// 🧪️ op_text_binary_roundtrip_law: handcrafted `OpText`/`OpBinary` JSON round-trip.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = SemioWorkflowSnapshot::default();
        for m in [SemioWorkflowMutation::NoMutation, SemioWorkflowMutation::SetSnapshot { snapshot: base.clone() }] {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioWorkflowMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioWorkflowMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
}
//#endregion 🔖️Tests
