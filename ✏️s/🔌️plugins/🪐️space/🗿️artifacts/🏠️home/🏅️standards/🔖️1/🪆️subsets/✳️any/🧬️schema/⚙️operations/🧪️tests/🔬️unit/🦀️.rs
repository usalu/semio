
use super::*;
use protocol::os_spr::protocol_laws::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law, assert_outcome_policy_matrix};

#[semio_framework_async_macros::async_test]
async fn home_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&change_catalog_generation(7));
}

#[semio_framework_async_macros::async_test]
async fn dispatch_registers_semantic_descriptors() {
    register_s_home_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in <SHomeMutation as protocol::SemanticMutation<SHomeSnapshot>>::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(<SHomeMutation as protocol::SemanticMutation<SHomeSnapshot>>::kinds().len(), 1);
}

//#region 🔖️MutationLaws
#[semio_framework_async_macros::async_test]
async fn change_catalog_generation_inverse_law() {
    let base = SHomeSnapshot::default();
    assert_mutation_inverse_law(&base, &change_catalog_generation(7)).await;
}

#[semio_framework_async_macros::async_test]
async fn change_catalog_generation_diff_absorb_law() {
    use protocol::Mutation;
    let base = SHomeSnapshot::default();
    let d1 = change_catalog_generation(3).diff(&base).diff().clone();
    let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
    let d2 = change_catalog_generation(9).diff(&mid).diff().clone();
    assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🔖️MutationLaws

// 🧪️OutcomeLaws — no `assert_missing_target_is_error`/`assert_fatal_never_applies` case applies:
// this facet's one mutation kind (`change-catalog-generation`) is a root scalar counter setter
// with no addressable target and no domain invariant to violate — it can only succeed (no
// messages) or raise a `mutation.no-op` Warning (see the leaf's own `🔺️diff`), never Error/Fatal.
// `assert_outcome_policy_matrix` DOES apply to that Warning-or-nothing outcome (Vigilant rejects
// a `mutation.no-op`, LaissezFaire/Normal accept it) — both cases are covered below.
//#region 🔖️OutcomeLaws
#[semio_framework_async_macros::async_test]
async fn change_catalog_generation_success_outcome_obeys_the_policy_matrix() {
    let base = SHomeSnapshot::default();
    assert_outcome_policy_matrix(&base, &change_catalog_generation(7)).await;
}

#[semio_framework_async_macros::async_test]
async fn change_catalog_generation_no_op_outcome_obeys_the_policy_matrix() {
    let base = SHomeSnapshot::default();
    assert_outcome_policy_matrix(&base, &change_catalog_generation(base.catalog_generation)).await;
}
//#endregion 🔖️OutcomeLaws
