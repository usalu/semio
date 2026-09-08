
use super::*;
use crate::Dictionary;
use crate::schema::default_snapshot;
use neural_engine::{Atom, Value};
use protocol::SemanticMutation;
use protocol::os_spr::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};

use std::collections::BTreeMap;

fn step(id: &str, kind: &str) -> Step {
    Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() }
}

//#region 🔖️MutationLaws
#[semio_framework_async_macros::async_test]
async fn create_step_inverse_law() {
    let base = default_snapshot();
    assert_mutation_inverse_law(&base, &create_step(PathRef::default(), step("step-99", "log.print"))).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_step_inverse_law() {
    let base = default_snapshot();
    assert_mutation_inverse_law(&base, &delete_step(PathRef::default(), "step-1".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_step_missing_target_is_error() {
    let base = default_snapshot();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &delete_step(PathRef::default(), "step-missing".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_steps_inverse_law() {
    let base = default_snapshot();
    assert_mutation_inverse_law(&base, &reorder_steps(PathRef::default(), "step-2".into(), 0)).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_steps_missing_target_is_error() {
    let base = default_snapshot();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &reorder_steps(PathRef::default(), "step-missing".into(), 0)).await;
}

#[semio_framework_async_macros::async_test]
async fn edit_step_params_inverse_law() {
    let base = default_snapshot();
    let params = Dictionary::new().insert("message", Value::Atom(Atom::String("hi".into())));
    assert_mutation_inverse_law(&base, &edit_step_params(PathRef::default(), "step-2".into(), params)).await;
}

#[semio_framework_async_macros::async_test]
async fn edit_step_params_missing_target_is_error() {
    let base = default_snapshot();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &edit_step_params(PathRef::default(), "step-missing".into(), Dictionary::new())).await;
}

#[semio_framework_async_macros::async_test]
async fn create_step_duplicate_id_fatal_never_applies() {
    let base = default_snapshot();
    let mutation = create_step(PathRef::default(), step("step-1", "log.print"));
    protocol::os_spr::testkit::assert_fatal_never_applies(&protocol::Mutation::diff(&mutation, &base)).await;
}

#[semio_framework_async_macros::async_test]
async fn create_step_diff_absorb_law() {
    use protocol::Mutation;
    let base = default_snapshot();
    let d1 = create_step(PathRef::default(), step("step-97", "log.print")).diff(&base).into_parts().0;
    let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
    let d2 = create_step(PathRef::default(), step("step-98", "log.print")).diff(&mid).into_parts().0;
    assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn dispatch_registers_semantic_descriptors() {
    register_procedure_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in ProcedureMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(ProcedureMutation::kinds().len(), 4);
}
//#endregion 🔖️MutationLaws
