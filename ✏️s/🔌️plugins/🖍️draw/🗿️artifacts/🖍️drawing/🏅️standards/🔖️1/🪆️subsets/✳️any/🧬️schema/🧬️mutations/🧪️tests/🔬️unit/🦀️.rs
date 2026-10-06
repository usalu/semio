use crate::standards::v1::subsets::any::io::text::mutations::parse_layer_field_input;
use super::*;
use crate::schema::{create_drawing_shape_layer_rect};
use crate::standards::v1::subsets::any::io::text::snapshot::{default_drawing_document};
use crate::standards::v1::subsets::any::io::text::snapshot::{create_drawing_path_layer};
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
    register_drawing_mutation_descriptors(::semio_framework_schema_state::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in DrawingMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        assert_eq!(kind.entity, match kind.kind { "update-path-geometry" => "path", "update-text" => "text", "set-group-isolation" => "group", "drag-layers" | "rotate-layers" | "scale-layers" => "layers", "drag-path-points" => "path-points", _ => "layer" });
    }
    assert_eq!(DrawingMutation::kinds().len(), KINDS.len());
}

#[test]
fn field_patch_validation_fixtures() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎛️field-patch/🔣️.json")).unwrap();
    let document = base_document();
    let id = crate::schema::layer_id(&document.layers[0]);
    for case in cases.as_array().unwrap() {
        let patch = &case["patch"];
        let value = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&patch["value"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
        let group_document = DrawingSnapshot { layers: vec![crate::schema::create_drawing_group_layer("Group")], ..Default::default() };
        let (target, target_id) = if patch["field"] == "isolation" { (&group_document, crate::schema::layer_id(&group_document.layers[0])) } else { (&document, id) };
        let operation = drawing_op_for_layer_field(target, target_id, patch["field"].as_str().unwrap(), &value);
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
        let inverse = operation.inverse(&document).expect("valid retained mutation inverse fixture");
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
    assert_eq!(parse_layer_field_input("name", "123"), semio_framework_value::DslValue::String("123".into()));
}

#[test]
fn text_field_edits_preserve_numeric_strings_and_other_facets() {
    use protocol::Mutation;
    let layer = crate::schema::create_drawing_text_layer("Text");
    let id = crate::schema::layer_id(&layer).to_string();
    let mut document = DrawingSnapshot { layers: vec![layer], ..Default::default() };
    for (field, input) in [("textContent", "123"), ("textContent", "Grüße 🌍\nHello"), ("textSize", "36")] {
        let before = document.clone();
        let mutation = drawing_op_for_layer_field(&document, &id, field, &parse_layer_field_input(field, input)).unwrap();
        let inverse = mutation.inverse(&document).expect("valid retained mutation inverse fixture");
        apply_drawing_mutation(&mut document, &mutation).unwrap();
        let DrawingLayerNode::Text(text) = &document.layers[0] else { panic!("Expected text") };
        if field == "textContent" { assert_eq!(text.content, input); assert_eq!(text.size, 24.0); }
        else { assert_eq!(text.size, 36.0); assert_eq!(text.content, "Text"); }
        for undo in inverse { apply_drawing_mutation(&mut document, &undo).unwrap(); }
        assert_eq!(document, before);
    }
    assert!(drawing_op_for_layer_field(&document, &id, "textSize", &parse_layer_field_input("textSize", "0")).is_none());
    let shape = base_document();
    assert!(drawing_op_for_layer_field(&shape, crate::schema::layer_id(&shape.layers[0]), "textContent", &parse_layer_field_input("textContent", "123")).is_none());
}

#[test]
fn all_inspector_blend_modes_preserve_other_fields_and_undo() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎛️field-patch/🔣️.json")).unwrap();
    let original = base_document();
    let id = crate::schema::layer_id(&original.layers[0]);
    for case in cases.as_array().unwrap().iter().filter(|case| case["patch"]["field"] == "blendMode" && case["accepted"] == true) {
        let mode = case["patch"]["value"].as_str().unwrap();
        let operation = drawing_op_for_layer_field(&original, id, "blendMode", &parse_layer_field_input("blendMode", mode)).unwrap();
        let inverse = operation.inverse(&original).expect("valid retained mutation inverse fixture");
        let mut document = original.clone();
        apply_drawing_mutation(&mut document, &operation).unwrap();
        let mut expected = original.clone();
        crate::schema::layer_base_mut(&mut expected.layers[0]).blend_mode = mode.into();
        assert_eq!(document, expected, "{mode}");
        for undo in inverse { apply_drawing_mutation(&mut document, &undo).unwrap(); }
        assert_eq!(document, original, "{mode}");
    }
}

//#region ⏪️TimeTravel
/// 🗺️ A drawing with two rects and a two-anchor path, plus the selection-transform log the canvas tool would have
/// committed over it — one leaf of every relative kind, then a downstream drag that depends on all of them.
fn selection_history() -> (DrawingSnapshot, Vec<DrawingMutation>) {
    let mut base = base_document();
    base.layers.push(create_drawing_shape_layer_rect("Other"));
    base.layers.push(create_drawing_path_layer("Spine", vec![crate::PathSegment::Move { to: [0.0, 0.0] }, crate::PathSegment::Line { to: [10.0, 0.0] }]));
    let [_, rect, other, spine] = [0, 1, 2, 3].map(|index| crate::schema::layer_id(&base.layers[index]).to_string());
    let anchor = DrawingPathPointTarget { layer_id: spine.clone(), index: 1, point: crate::schema::geometry::editing::PathPoint::Anchor };
    let log = vec![
        drag_layers(vec![rect.clone()], 10.0, 0.0),
        rotate_layers(vec![rect.clone(), other.clone()], 0.0, 0.0, std::f64::consts::FRAC_PI_2),
        scale_layers(vec![other], 0.0, 0.0, 2.0, 2.0),
        drag_path_points(vec![anchor], 0.0, 5.0),
        drag_layers(vec![rect, spine], 1.0, 1.0),
    ];
    (base, log)
}

/// ⏪️ Opens a standalone store over `base`, commits `log` one edit per leaf, supersedes the leaf at `index` with
/// `edited` and drives the Report replay to its end: the preview base is the fold of the prefix, the replay is the
/// fresh fold of the edited log, and an overwrite finalizes exactly that state. Returns the report.
async fn replay_history_edit(base: &DrawingSnapshot, log: &[DrawingMutation], index: usize, edited: &DrawingMutation) -> protocol::ReplayReport {
    use protocol::OpBinary;
    let mut store = store::ArtifactStore::new(store::create_document_envelope::<DrawingSnapshot, DrawingMutation>(crate::DRAWING_DOCUMENT_SCHEMA, "selection-time-travel", base.clone(), None)).await.expect("the store opens");
    store.install_document_store_owners_exact(crate::spr::drawing_document_store_owners());
    for mutation in log {
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation.clone()], transaction: None }).await.expect("a selection transform applies");
    }
    let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
    assert_eq!(ids.len(), log.len(), "one applied operation per committed leaf");
    let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(ids[index].clone(), protocol::InputReplacement::Input { schema: crate::DRAWING_DOCUMENT_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
    let mut prefix = base.clone();
    for mutation in &log[..index] {
        apply_drawing_mutation(&mut prefix, mutation).expect("the prefix folds");
    }
    let preview = store.state_before(&ids[index], &drafts).expect("the preview base folds");
    assert_eq!(preview.as_ref(), &prefix, "the preview base is the state right before the edited leaf, nothing downstream");
    let mut replay = store.begin_report_replay(&drafts, Some(&ids[index])).expect("the replay begins at the edited leaf");
    assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)));
    let result = replay.finish().expect("a finished replay yields its result");
    let report = store.replay_report(&result).expect("report");
    let mut fresh = base.clone();
    for (position, mutation) in log.iter().enumerate() {
        let mutation = if position == index { edited } else { mutation };
        if !mutation.diff(&fresh).messages().iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
            apply_drawing_mutation(&mut fresh, mutation).expect("the edited log folds");
        }
    }
    assert_eq!(result.state().expect("the replay reached a state").as_ref(), &fresh, "the replay equals the fresh fold of the edited log");
    if !report.blocks_finalize() {
        store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
        assert_eq!(store.snapshot_ref(), &fresh, "the overwritten history folds to the edited state");
    }
    store::os_store::test_support::close_plain_test_store(&mut store);
    report
}

/// ⏪️ Time travel edits each relative selection leaf's inputs, never the canvas tool: re-offsetting the drag,
/// re-pivoting the rotation, re-factoring the scale and re-offsetting the path-point drag each replay every downstream
/// transform onto the edited state — exactly the fresh fold of the edited log — and never block finalizing.
#[semio_framework_async_macros::async_test]
async fn every_selection_leaf_edited_in_history_replays_its_downstream() {
    let (base, log) = selection_history();
    let DrawingMutation::DragPathPoints(points) = &log[3] else { panic!("the fourth leaf drags path points") };
    let edits = [
        (0, drag_layers(vec![crate::schema::layer_id(&base.layers[1]).into()], -4.0, 3.0)),
        (1, rotate_layers(vec![crate::schema::layer_id(&base.layers[1]).into(), crate::schema::layer_id(&base.layers[2]).into()], 5.0, 5.0, std::f64::consts::PI)),
        (2, scale_layers(vec![crate::schema::layer_id(&base.layers[2]).into()], 1.0, 1.0, 0.5, 3.0)),
        (3, drag_path_points(points.targets.clone(), 7.0, -2.0)),
    ];
    for (index, edited) in &edits {
        let report = replay_history_edit(&base, &log, *index, edited).await;
        assert!(!report.blocks_finalize(), "a re-parametrised {} never blocks finalizing: {report:?}", edited.label().resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En));
        assert_eq!(report.outcomes.len(), log.len() - index, "the replay reports the edited leaf and every downstream one");
    }
}

/// 🚨️ An edit that retargets a drag onto a layer that does not exist reports the edited leaf's own
/// `mutation.target-missing` Error, which blocks finalizing until it is edited again or withdrawn.
#[semio_framework_async_macros::async_test]
async fn a_drag_retargeted_onto_a_missing_layer_blocks_finalizing() {
    let (base, log) = selection_history();
    let report = replay_history_edit(&base, &log, 0, &drag_layers(vec!["ghost".into()], 10.0, 0.0)).await;
    assert!(report.blocks_finalize(), "{report:?}");
    assert!(report.outcomes[0].messages.iter().any(|message| message.code.0 == "mutation.target-missing"), "{report:?}");
}

/// 🪧️ A history-edit reference chip reads a layer by its own name in the shown document (the generic default, gap N3), and
/// every layer reference of the four selection leaves takes "Use selection" from the canvas selection domain.
#[test]
fn layer_references_read_their_name_and_take_the_canvas_selection() {
    let mut base = base_document();
    crate::schema::layer_base_mut(&mut base.layers[0]).name="Rectangle <Name> & Ü".into();
    let id = crate::schema::layer_id(&base.layers[0]).to_string();
    let names = semio_framework_plugin::app::time_travel::time_travel_entity_names(&semio_framework_value::ToValue::to_value(&base), &[id.as_str()].into_iter().collect());
    assert_eq!(names.get(&id).map(|label| label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De).to_owned()).as_deref(), Some("Rectangle <Name> & Ü"));
    fn references(value: &serde_json::Value, into: &mut Vec<serde_json::Value>) {
        match value {
            serde_json::Value::Object(fields) => {
                into.extend(fields.get("x-semio-ui").and_then(|ui| ui.get("ref")).cloned());
                fields.values().for_each(|field| references(field, into));
            }
            serde_json::Value::Array(items) => items.iter().for_each(|item| references(item, into)),
            _ => {}
        }
    }
    for schema in [<DragLayers as protocol::MutationLeaf>::PAYLOAD_SCHEMA, <RotateLayers as protocol::MutationLeaf>::PAYLOAD_SCHEMA, <ScaleLayers as protocol::MutationLeaf>::PAYLOAD_SCHEMA, <DragPathPoints as protocol::MutationLeaf>::PAYLOAD_SCHEMA] {
        let mut found = Vec::new();
        references(&serde_json::from_str(schema).expect("leaf schema"), &mut found);
        assert!(!found.is_empty(), "{schema}");
        for reference in found {
            assert_eq!((reference["domain"].as_str(), reference["granularity"].as_str()), (Some(crate::editor::drawing::DRAWING_INTERACTION_DOMAIN), Some(crate::editor::drawing::DRAWING_INTERACTION_GRANULARITY)), "{reference}");
        }
    }
}
//#endregion ⏪️TimeTravel
