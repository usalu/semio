use super::*;
use crate::schema::{create_drawing_path_layer, create_drawing_shape_layer_rect, default_drawing_document};
use protocol::os_spr::protocol_laws::{assert_fatal_never_applies, assert_missing_target_is_error, assert_mutation_diff_absorb_law, assert_mutation_inverse_law, assert_outcome_policy_matrix};
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
        assert_eq!(kind.entity, if kind.kind == "update-path-geometry" { "path" } else { "layer" });
    }
    assert_eq!(DrawingMutation::kinds().len(), 15);
}

#[test]
fn field_patch_validation_fixtures() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎛️field-patch/🔣️.json")).unwrap();
    let document = base_document();
    let id = crate::schema::layer_id(&document.layers[0]);
    for case in cases.as_array().unwrap() {
        let patch = &case["patch"];
        let value = dsl::json::to_dsl_value(&dsl::json::parse(&patch["value"].to_string()).unwrap());
        let operation = drawing_op_for_layer_field(&document, id, patch["field"].as_str().unwrap(), &value);
        assert_eq!(operation.is_some(), case["accepted"].as_bool().unwrap(), "{case}");
        if patch["field"] == "rotationDegrees" {
            let DrawingMutation::UpdateLayerTransform(update) = operation.unwrap() else { panic!("rotation must use a semantic transform mutation") };
            assert!((update.transform.rotation - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
        }
    }
}

#[test]
fn stroke_fields_preserve_appearance_and_undo() {
    use protocol::Mutation;
    let original = DrawingSnapshot { layers: vec![create_drawing_shape_layer_rect("Stroke target")], ..Default::default() };
    let id = crate::schema::layer_id(&original.layers[0]);
    let mut document = original.clone();
    for (field, input) in [("strokeWidth", "3"), ("strokeColor", "#123456"), ("strokeCap", "round"), ("strokeJoin", "bevel"), ("strokeDash", "8")] {
        let value = parse_layer_field_input(field, input);
        let operation = drawing_op_for_layer_field(&document, id, field, &value).unwrap();
        let inverse = operation.inverse(&document);
        let before = document.clone();
        apply_drawing_mutation(&mut document, &operation).unwrap();
        let mut undone = document.clone();
        for undo in inverse { apply_drawing_mutation(&mut undone, &undo).unwrap(); }
        assert_eq!(undone, before);
    }
    let stroke = crate::schema::layer_base(&document.layers[0]).attributes.stroke.as_ref().unwrap();
    assert_eq!((stroke.width, stroke.cap.as_str(), stroke.join.as_str()), (3.0, "round", "bevel"));
    assert_eq!(stroke.dash, Some(vec![8.0]));
    let scene = crate::schema::flatten_drawing_document_to_scene_nodes(&document);
    assert_eq!(scene.iter().find_map(|node| node.stroke.as_ref()), Some(stroke));
    assert_eq!(parse_layer_field_input("name", "123"), dsl::DslValue::String("123".into()));
    eprintln!("[DEBUG] stroke edits preserve sibling attributes and undo atomically");
}
