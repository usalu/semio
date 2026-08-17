//! 🧬️ S Space index artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<SSpaceSnapshot>` and `impl protocol::SemanticMutation<SSpaceSnapshot>` from
//! that payload — no hand-written apply/diff/inverse dispatch here. Ticket
//! 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C4 / MUTATION-OUTCOMES contract: every
//! leaf's `diff` returns `protocol::MutationOutcome<SSpaceDiff>` with the frozen fault codes.

use crate::artifacts::space::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 🧮️ Semantic S Space index mutation vocabulary: id-keyed artifact row create/delete/rename/touch.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = SSpaceSnapshot, diff = SSpaceDiff, schema = "s.space.space")]
pub enum SSpaceMutation {
    CreateArtifact(CreateArtifact),
    DeleteArtifact(DeleteArtifact),
    RenameArtifact(RenameArtifact),
    TouchArtifact(TouchArtifact),
}
//#endregion 🔖️Mutations

pub use super::create_artifact::mutation::{create_artifact, CreateArtifact};
pub use super::delete_artifact::mutation::{delete_artifact, DeleteArtifact};
pub use super::rename_artifact::mutation::{rename_artifact, RenameArtifact};
pub use super::touch_artifact::mutation::{touch_artifact, TouchArtifact};

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::{empty_space_index_snapshot, SpaceArtifactDialect, SpaceArtifactRow};
    use protocol::testkit::{assert_fatal_never_applies, assert_missing_target_is_error, assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::Mutation;
    use protocol::SemanticMutation;

    fn sample_row(id: &str) -> SpaceArtifactRow {
        SpaceArtifactRow {
            id: id.into(),
            name: format!("Artifact {id}"),
            kind_id: "space.sdraw".into(),
            schema: "s.draw".into(),
            dialect: SpaceArtifactDialect { artifact_kind: "s.draw".into(), standard: "1".into(), subset: "*".into() },
            created_at_ms: 1,
            created_by: "user:1".into(),
            updated_at_ms: 1,
            updated_by: "user:1".into(),
        }
    }

    fn seeded_snapshot() -> SSpaceSnapshot {
        let mut snapshot = empty_space_index_snapshot("space-1");
        snapshot.artifacts.push(sample_row("artifact-1"));
        snapshot
    }

    #[test]
    fn home_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&create_artifact(sample_row("artifact-9")));
        store::os_store::test_support::assert_op_line_round_trip(&delete_artifact("artifact-1".into()));
        store::os_store::test_support::assert_op_line_round_trip(&rename_artifact("artifact-1".into(), "New Name".into()));
        store::os_store::test_support::assert_op_line_round_trip(&touch_artifact("artifact-1".into(), 42, "user:2".into()));
    }

    #[test]
    fn dispatch_registers_semantic_descriptors() {
        register_s_space_mutation_descriptors();
        for kind in <SSpaceMutation as protocol::SemanticMutation<SSpaceSnapshot>>::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(<SSpaceMutation as protocol::SemanticMutation<SSpaceSnapshot>>::kinds().len(), 4);
    }

    //#region 🔖️MutationLaws
    #[test]
    fn create_artifact_inverse_law() {
        let base = empty_space_index_snapshot("space-1");
        assert_mutation_inverse_law(&base, &create_artifact(sample_row("artifact-1")));
    }

    #[test]
    fn delete_artifact_inverse_law() {
        let base = seeded_snapshot();
        assert_mutation_inverse_law(&base, &delete_artifact("artifact-1".into()));
    }

    #[test]
    fn rename_artifact_inverse_law() {
        let base = seeded_snapshot();
        assert_mutation_inverse_law(&base, &rename_artifact("artifact-1".into(), "Renamed".into()));
    }

    #[test]
    fn touch_artifact_inverse_law() {
        let base = seeded_snapshot();
        assert_mutation_inverse_law(&base, &touch_artifact("artifact-1".into(), 99, "user:3".into()));
    }

    #[test]
    fn create_artifact_diff_absorb_law() {
        let base = empty_space_index_snapshot("space-1");
        let d1 = create_artifact(sample_row("artifact-1")).diff(&base).diff().clone();
        let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
        let d2 = touch_artifact("artifact-1".into(), 55, "user:4".into()).diff(&mid).diff().clone();
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🔖️MutationLaws

    //#region 🔖️OutcomeLaws
    /// ✅️ §C2/fan-out-recipe laws — one per verb family this facet implements.
    #[test]
    fn create_artifact_duplicate_id_is_fatal() {
        let base = seeded_snapshot();
        let outcome = create_artifact(sample_row("artifact-1")).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[test]
    fn delete_artifact_missing_target_is_error() {
        let base = seeded_snapshot();
        assert_missing_target_is_error(&base, &delete_artifact("ghost".into()));
    }

    #[test]
    fn rename_artifact_missing_target_is_error() {
        let base = seeded_snapshot();
        assert_missing_target_is_error(&base, &rename_artifact("ghost".into(), "x".into()));
    }

    #[test]
    fn rename_artifact_same_name_is_no_op() {
        let base = seeded_snapshot();
        let outcome = rename_artifact("artifact-1".into(), "Artifact artifact-1".into()).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert_eq!(outcome.diff(), &SSpaceDiff::default());
    }

    #[test]
    fn rename_artifact_name_collision_is_fatal() {
        let mut base = seeded_snapshot();
        base.artifacts.push(sample_row("artifact-2"));
        base.artifacts[1].name = "Taken".into();
        let outcome = rename_artifact("artifact-1".into(), "Taken".into()).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[test]
    fn touch_artifact_missing_target_is_error() {
        let base = seeded_snapshot();
        assert_missing_target_is_error(&base, &touch_artifact("ghost".into(), 1, "user:1".into()));
    }
    //#endregion 🔖️OutcomeLaws
}
//#endregion 🧪️Tests
