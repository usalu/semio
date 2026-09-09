use super::*;
use crate::schema::{create_drawing_path_layer, create_drawing_shape_layer_rect, default_drawing_document};
use protocol::os_spr::testkit::{assert_fatal_never_applies, assert_missing_target_is_error, assert_mutation_diff_absorb_law, assert_mutation_inverse_law, assert_outcome_policy_matrix};
use protocol::{Mutation, MutationDiff, SemanticMutation};

fn base_document() -> DrawingSnapshot {
    let mut doc = default_drawing_document("mutations-test", None);
    doc.layers.push(create_drawing_shape_layer_rect("Rect"));
    doc
}

#[semio_framework_async_macros::async_test]
async fn set_layer_visible_inverse_law() {
    let base = base_document();
    let layer_id = crate::schema::layer_id(&base.layers[0]).to_string();
    let mutation = set_layer_visible(layer_id, false);
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_layer_inverse_law() {
    let base = base_document();
    let layer_id = crate::schema::layer_id(&base.layers[0]).to_string();
    let mutation = rename_layer(layer_id, "Renamed".into());
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn create_layer_inverse_law() {
    let base = base_document();
    let mutation = create_layer(None, None, create_drawing_path_layer("New", Vec::new()));
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_layer_inverse_law() {
    let base = base_document();
    let layer_id = crate::schema::layer_id(&base.layers[0]).to_string();
    let mutation = delete_layer(layer_id);
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn duplicate_layer_inverse_law() {
    let base = base_document();
    let layer_id = crate::schema::layer_id(&base.layers[0]).to_string();
    let mutation = duplicate_layer(layer_id);
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_layer_inverse_law() {
    let mut base = base_document();
    base.layers.push(create_drawing_path_layer("Second", Vec::new()));
    let layer_id = crate::schema::layer_id(&base.layers[0]).to_string();
    let mutation = reorder_layer(layer_id, None, 1);
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn set_layer_opacity_diff_absorb_law() {
    let base = base_document();
    let layer_id = crate::schema::layer_id(&base.layers[0]).to_string();
    let d1 = set_layer_opacity(layer_id.clone(), 0.5).diff(&base).diff().clone();
    let mid = d1.apply(&base).expect("valid mutation diff");
    let d2 = set_layer_opacity(layer_id, 0.25).diff(&mid).diff().clone();
    assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

//#region 🧪️OutcomeLaws
/// ⚖️ `📋️contract-freeze.md` §C2 laws, per verb family: `assert_missing_target_is_error`/
/// `assert_fatal_never_applies` below, `assert_outcome_policy_matrix` cases further down (delete,
/// rename, set, create).
#[semio_framework_async_macros::async_test]
async fn delete_missing_layer_is_a_target_missing_error() {
    let base = base_document();
    assert_missing_target_is_error(&base, &delete_layer("does-not-exist".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_missing_layer_is_a_target_missing_error() {
    let base = base_document();
    assert_missing_target_is_error(&base, &rename_layer("does-not-exist".into(), "New Name".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn set_layer_opacity_missing_layer_is_a_target_missing_error() {
    let base = base_document();
    assert_missing_target_is_error(&base, &set_layer_opacity("does-not-exist".into(), 0.5)).await;
}

#[semio_framework_async_macros::async_test]
async fn create_layer_duplicate_id_never_applies() {
    let base = base_document();
    // Re-creating the exact existing node collides on id for real (ids are content-addressed).
    let duplicate = create_layer(None, None, base.layers[0].clone());
    assert_fatal_never_applies(&duplicate.diff(&base)).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_layer_outcome_obeys_the_policy_matrix() {
    let base = base_document();
    let layer_id = crate::schema::layer_id(&base.layers[0]).to_string();
    assert_outcome_policy_matrix(&base, &delete_layer(layer_id)).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_layer_outcome_obeys_the_policy_matrix() {
    let base = base_document();
    let layer_id = crate::schema::layer_id(&base.layers[0]).to_string();
    assert_outcome_policy_matrix(&base, &rename_layer(layer_id, "Renamed".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn set_layer_opacity_outcome_obeys_the_policy_matrix() {
    let base = base_document();
    let layer_id = crate::schema::layer_id(&base.layers[0]).to_string();
    assert_outcome_policy_matrix(&base, &set_layer_opacity(layer_id, 0.5)).await;
}

#[semio_framework_async_macros::async_test]
async fn create_layer_outcome_obeys_the_policy_matrix() {
    let base = base_document();
    assert_outcome_policy_matrix(&base, &create_layer(None, None, create_drawing_path_layer("New", Vec::new()))).await;
}
//#endregion 🧪️OutcomeLaws

#[semio_framework_async_macros::async_test]
async fn dispatch_registers_semantic_descriptors() {
    register_drawing_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in DrawingMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        assert_eq!(kind.entity, "layer");
    }
    assert_eq!(DrawingMutation::kinds().len(), 14);
}
