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

    async fn sample_row(id: &str) -> SpaceArtifactRow {
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

    async fn seeded_snapshot() -> SSpaceSnapshot {
        let mut snapshot = empty_space_index_snapshot("space-1");
        snapshot.artifacts.push(sample_row("artifact-1"));
        snapshot
    }

    #[semio_framework_async_macros::async_test]
    async fn home_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&create_artifact(sample_row("artifact-9")));
        store::os_store::test_support::assert_op_line_round_trip(&delete_artifact("artifact-1".into()));
        store::os_store::test_support::assert_op_line_round_trip(&rename_artifact("artifact-1".into(), "New Name".into()));
        store::os_store::test_support::assert_op_line_round_trip(&touch_artifact("artifact-1".into(), 42, "user:2".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors() {
        register_s_space_mutation_descriptors();
        for kind in <SSpaceMutation as protocol::SemanticMutation<SSpaceSnapshot>>::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(<SSpaceMutation as protocol::SemanticMutation<SSpaceSnapshot>>::kinds().len(), 4);
    }

    //#region 🔖️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn create_artifact_inverse_law() {
        let base = empty_space_index_snapshot("space-1");
        assert_mutation_inverse_law(&base, &create_artifact(sample_row("artifact-1")));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_artifact_inverse_law() {
        let base = seeded_snapshot();
        assert_mutation_inverse_law(&base, &delete_artifact("artifact-1".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_artifact_inverse_law() {
        let base = seeded_snapshot();
        assert_mutation_inverse_law(&base, &rename_artifact("artifact-1".into(), "Renamed".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn touch_artifact_inverse_law() {
        let base = seeded_snapshot();
        assert_mutation_inverse_law(&base, &touch_artifact("artifact-1".into(), 99, "user:3".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_artifact_diff_absorb_law() {
        let base = empty_space_index_snapshot("space-1");
        let d1 = create_artifact(sample_row("artifact-1")).diff(&base).diff().clone();
        let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
        let d2 = touch_artifact("artifact-1".into(), 55, "user:4".into()).diff(&mid).diff().clone();
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🔖️MutationLaws

    //#region 🔖️OutcomeLaws
    /// ✅️ §C2/fan-out-recipe laws — one per verb family this facet implements.
    #[semio_framework_async_macros::async_test]
    async fn create_artifact_duplicate_id_is_fatal() {
        let base = seeded_snapshot();
        let outcome = create_artifact(sample_row("artifact-1")).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_artifact_missing_target_is_error() {
        let base = seeded_snapshot();
        assert_missing_target_is_error(&base, &delete_artifact("ghost".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_artifact_missing_target_is_error() {
        let base = seeded_snapshot();
        assert_missing_target_is_error(&base, &rename_artifact("ghost".into(), "x".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_artifact_same_name_is_no_op() {
        let base = seeded_snapshot();
        let outcome = rename_artifact("artifact-1".into(), "Artifact artifact-1".into()).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert_eq!(outcome.diff(), &SSpaceDiff::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_artifact_name_collision_is_fatal() {
        let mut base = seeded_snapshot();
        base.artifacts.push(sample_row("artifact-2"));
        base.artifacts[1].name = "Taken".into();
        let outcome = rename_artifact("artifact-1".into(), "Taken".into()).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn touch_artifact_missing_target_is_error() {
        let base = seeded_snapshot();
        assert_missing_target_is_error(&base, &touch_artifact("ghost".into(), 1, "user:1".into()));
    }
    //#endregion 🔖️OutcomeLaws
}
//#endregion 🧪️Tests

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `SSpaceMutation` variant, in declaration order — the vocabulary the `s-space-1-any` mutation catalog
/// (`../../🧪️oracle/🔣️component.json`) declares and the `mutate-s-space-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] = &[
    "create-artifact",
    "delete-artifact",
    "rename-artifact",
    "touch-artifact",
];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// there is no `serde`, no `serde_json` and no `protocol` reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `SSpaceMutation` nor `SSpaceSnapshot`
/// can be named there and hand-transcribing either into a Rust literal would be a second copy of the
/// committed specification vector, free to drift away from it. This bridge is the whole surface an
/// adapter needs, and every type in its signature is a `str`.
///
/// The report carries the forward half (`snapshot`, `diff`, `messages`) and the inverse half
/// (`inverseSteps`, `inverseSnapshot`, `inverseMessages`), so the inverse law is checked against the
/// mutation's OWN computed inverse rather than against a hand-written undo.
///
/// @see ../../🧪️oracle/🔣️component.json — the catalog and the recorded no-oracle decision.
pub fn s_space_mutation_report_json(base_json: &str, mutation_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<SSpaceSnapshot, String> { Ok(serde_json::from_str(text).map_err(|error| error.to_string())?) };
    let base = decode_snapshot(base_json)?;
    let mutation: SSpaceMutation = serde_json::from_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = serde_json::json!({
        "snapshot": serde_json::to_value(&applied).map_err(|error| error.to_string())?,
        "diff": serde_json::to_value(forward.diff()).map_err(|error| error.to_string())?,
        "messages": serde_json::to_value(forward.messages()).map_err(|error| error.to_string())?,
        "inverseSteps": serde_json::to_value(&inverse).map_err(|error| error.to_string())?,
        "inverseSnapshot": serde_json::to_value(&undone).map_err(|error| error.to_string())?,
        "inverseMessages": serde_json::to_value(&inverse_messages).map_err(|error| error.to_string())?,
    });
    Ok(report.to_string())
}
//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
mod kinds_conformance {
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of them must appear in the committed oracle
    /// manifest's catalog. The framework never parses Rust, so this is what keeps the declaration
    /// honest in both directions at once.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <SSpaceMutation as protocol::SemanticMutation<SSpaceSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsConformance
