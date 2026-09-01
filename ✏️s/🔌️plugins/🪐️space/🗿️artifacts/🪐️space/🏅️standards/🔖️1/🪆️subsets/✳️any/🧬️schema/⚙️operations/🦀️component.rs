//! ⚙️ S Space mutation bridge, laws, and behavior tests.

use crate::artifacts::space::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::{create_artifact, delete_artifact, register_s_space_mutation_descriptors, rename_artifact, touch_artifact, SSpaceMutation};
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

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
        register_s_space_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
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

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `SSpaceMutation` nor
/// `SSpaceSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🧪️oracle/🔣️.json — the catalog and the recorded no-oracle decision.
pub fn s_space_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<SSpaceSnapshot, String> {
        let decoded: SSpaceSnapshot = pack::from_json_str(text).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: SSpaceMutation = pack::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = pack::json!({
        "base": pack::json_from_dsl_value(&dsl::ToValue::to_value(&base)),
        "expectedSnapshot": pack::json_from_dsl_value(&dsl::ToValue::to_value(&expected)),
        "snapshot": pack::json_from_dsl_value(&dsl::ToValue::to_value(&applied)),
        "diff": pack::json_from_dsl_value(&dsl::ToValue::to_value(forward.diff())),
        "messages": pack::json_from_dsl_value(&dsl::ToValue::to_value(&forward.messages().to_vec())),
        "inverseSteps": pack::json_from_dsl_value(&dsl::ToValue::to_value(&inverse)),
        "inverseSnapshot": pack::json_from_dsl_value(&dsl::ToValue::to_value(&undone)),
        "inverseMessages": pack::json_from_dsl_value(&dsl::ToValue::to_value(&inverse_messages)),
    });
    Ok(report.to_string())
}
//#endregion 🌉️TestBridge
