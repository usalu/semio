use serde_json::Value;
use crate::apply_puzzle3d_mutation;

use super::*;
use semio_s_artifact_puzzle_3d_mutations_object::delete_object::mutation::delete_object;
use semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::move_object;
use semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object;
use semio_s_artifact_puzzle_3d_mutations_pose::rotate_object::mutation::rotate_object;
use semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::scale_object;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_mesh::mutation::change_object_mesh;
use semio_s_artifact_puzzle_3d_mutations_object::edit_object_label::mutation::edit_object_label;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_kind::mutation::change_object_kind;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_anchor::mutation::change_object_anchor;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_hidden::mutation::change_object_hidden;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_locked::mutation::change_object_locked;
use semio_s_artifact_puzzle_3d_mutations_vortex::add_object_vortex::mutation::add_object_vortex;
use semio_s_artifact_puzzle_3d_mutations_vortex::remove_object_vortex::mutation::remove_object_vortex;
use semio_s_artifact_puzzle_3d_mutations_vortex::replace_object_vortex::mutation::replace_object_vortex;
use semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices;
use semio_s_artifact_puzzle_3d_mutations_vortex::disconnect_vortices::mutation::disconnect_vortices;
use semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::replace_attraction_geometry;
use semio_s_artifact_puzzle_3d_mutations_volume::create_target_volume::mutation::create_target_volume;
use semio_s_artifact_puzzle_3d_mutations_volume::move_target_volume::mutation::move_target_volume;
use semio_s_artifact_puzzle_3d_mutations_volume::rotate_target_volume::mutation::rotate_target_volume;
use semio_s_artifact_puzzle_3d_mutations_volume::scale_target_volume::mutation::scale_target_volume;
use semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_hidden::mutation::change_target_volume_hidden;
use semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_locked::mutation::change_target_volume_locked;
use semio_s_artifact_puzzle_3d_mutations_volume::delete_target_volume::mutation::delete_target_volume;
use semio_s_artifact_puzzle_3d_mutations_reference::create_reference::mutation::create_reference;
use semio_s_artifact_puzzle_3d_mutations_reference::move_reference::mutation::move_reference;
use semio_s_artifact_puzzle_3d_mutations_reference::resize_reference::mutation::resize_reference;
use semio_s_artifact_puzzle_3d_mutations_reference::replace_reference_source::mutation::replace_reference_source;
use semio_s_artifact_puzzle_3d_mutations_reference::change_reference_hidden::mutation::change_reference_hidden;
use semio_s_artifact_puzzle_3d_mutations_reference::change_reference_locked::mutation::change_reference_locked;
use semio_s_artifact_puzzle_3d_mutations_reference::delete_reference::mutation::delete_reference;
use semio_s_artifact_puzzle_3d_mutations_catalog::change_domain::mutation::change_domain;
use semio_s_artifact_puzzle_3d_mutations_catalog::connect_kind_compatibility::mutation::connect_kind_compatibility;
use semio_s_artifact_puzzle_3d_mutations_catalog::disconnect_kind_compatibility::mutation::disconnect_kind_compatibility;
use semio_s_artifact_puzzle_3d_mutations_catalog::replace_kind_catalogs::mutation::replace_kind_catalogs;
use semio_s_artifact_puzzle_3d_mutations_selection::drag_selection::mutation::drag_selection;

#[test]
fn puzzle3d_delta_ops_round_trip_and_stay_granular() {
    let before = serde_json::json!({
        "schema": semio_s_artifact_puzzle_3d_model::PUZZLE_3D_SCHEMA, "domain": "architecture",
        "meta": {},
        "objects": [
            { "id": "o1", "anchor": "fixed", "origin": [0.0,0.0,0.0], "vortices": [] },
            { "id": "o2", "anchor": "fixed", "origin": [1.0,0.0,0.0], "vortices": [] },
        ],
        "attractions": [], "targetVolumes": [], "references": [],
    });
    let after = serde_json::json!({
        "schema": semio_s_artifact_puzzle_3d_model::PUZZLE_3D_SCHEMA, "domain": "architecture",
        "meta": {},
        "objects": [
            { "id": "o2", "anchor": "fixed", "origin": [9.0,0.0,0.0], "vortices": [] },
            { "id": "o3", "anchor": "fixed", "origin": [2.0,0.0,0.0], "vortices": [] },
        ],
        "attractions": [], "targetVolumes": [], "references": [],
    });
    let canonical = |value: &Value| serde_json::to_value(serde_json::from_value::<Puzzle3dSnapshot>(value.clone()).expect("typed puzzle3d scene_snapshot")).expect("canonical puzzle3d JSON");
    let before_snapshot: Puzzle3dSnapshot=semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&before)).expect("base admits");
    let after_snapshot: Puzzle3dSnapshot=semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&after)).expect("result admits");
    let created = semio_s_artifact_puzzle_3d_model::Puzzle3dObject { id: "o3".into(), label: None, object_kind: None, anchor: Default::default(), origin: [2.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    let operations = vec![delete_object("o1".into()), move_object("o2".into(), [9.0, 0.0, 0.0]), create_object(created, None)];
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle3dMutation::MoveObject(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle3dMutation::CreateObject(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle3dMutation::DeleteObject(_))));
    let mut forward = before_snapshot.clone();
    let mut inverses = Vec::new();
    for operation in &operations {
        inverses.extend(Mutation::<Puzzle3dSnapshot>::inverse(operation, &forward).expect("valid retained mutation inverse scene_snapshot"));
        forward = protocol::apply_diff_cold(Mutation::<Puzzle3dSnapshot>::diff(operation, &forward).diff(), &forward).expect("valid mutation diff");
    }
    assert_eq!(serde_json::to_value(&forward).unwrap(),canonical(&after));
    for inverse in inverses.iter().rev() {
        forward = protocol::apply_diff_cold(Mutation::<Puzzle3dSnapshot>::diff(inverse, &forward).diff(), &forward).expect("valid mutation diff");
    }
    assert_eq!(serde_json::to_value(&forward).unwrap(),canonical(&before), "backwards operations must restore the pre-edit document");
}

//#region 🔖️MutationLaws
use protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_law;

#[semio_framework_async_macros::async_test]
async fn move_object_diff_absorb_law() {
    use semio_s_artifact_puzzle_3d_model::Puzzle3dObject;
    let base = empty();
    let object = Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    let with_object = protocol::apply_diff_cold(create_object(object, None).diff(&base).diff(), &base).expect("valid mutation diff");
    let d1 = move_object("o1".into(), [10.0, 10.0, 10.0]).diff(&with_object).into_parts().0;
    let mid = protocol::apply_diff_cold(&d1, &with_object).expect("valid mutation diff");
    let d2 = move_object("o1".into(), [20.0, 30.0, 40.0]).diff(&mid).into_parts().0;
    (assert_mutation_diff_absorb_law(&with_object, d1, d2)).await;
}

fn empty() -> Puzzle3dSnapshot {
    Puzzle3dSnapshot::default()
}

#[semio_framework_async_macros::async_test]
async fn create_delete_object_inverse_law() {
    use semio_s_artifact_puzzle_3d_model::Puzzle3dObject;
    let base = empty();
    let object = Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    (assert_mutation_inverse_law(&base, &create_object(object.clone(), None))).await;
    let with_object = protocol::apply_diff_cold(create_object(object, None).diff(&base).diff(), &base).expect("valid mutation diff");
    (assert_mutation_inverse_law(&with_object, &delete_object("o1".into()))).await;
}

#[semio_framework_async_macros::async_test]
async fn object_field_mutations_inverse_law() {
    use {semio_s_artifact_puzzle_3d_model::Puzzle3dObject, semio_s_artifact_puzzle_3d_model::Puzzle3dObjectAnchor, semio_s_artifact_puzzle_3d_model::Puzzle3dScale, semio_s_artifact_puzzle_3d_model::Puzzle3dVortex};
    let base = empty();
    let object = Puzzle3dObject {
        id: "o1".into(),
        label: None,
        object_kind: None,
        anchor: Default::default(),
        origin: [0.0, 0.0, 0.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: vec![Puzzle3dVortex { id: "v1".into(), vortex_kind: None, label: None, position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false }],
        hidden: false,
        locked: false,
    };
    let with_object = protocol::apply_diff_cold(create_object(object, None).diff(&base).diff(), &base).expect("valid mutation diff");
    (assert_mutation_inverse_law(&with_object, &move_object("o1".into(), [1.0, 2.0, 3.0]))).await;
    (assert_mutation_inverse_law(&with_object, &rotate_object("o1".into(), Some([0.0, 0.0, 0.0, 1.0])))).await;
    (assert_mutation_inverse_law(&with_object, &scale_object("o1".into(), Some(Puzzle3dScale::Uniform(2.0))))).await;
    (assert_mutation_inverse_law(&with_object, &change_object_mesh("o1".into(), Some("mesh://a".into())))).await;
    (assert_mutation_inverse_law(&with_object, &edit_object_label("o1".into(), Some("Label".into())))).await;
    (assert_mutation_inverse_law(&with_object, &change_object_kind("o1".into(), Some("core.capsule".into())))).await;
    (assert_mutation_inverse_law(&with_object, &change_object_anchor("o1".into(), Puzzle3dObjectAnchor::Derived))).await;
    (assert_mutation_inverse_law(&with_object, &change_object_hidden("o1".into(), true))).await;
    (assert_mutation_inverse_law(&with_object, &change_object_locked("o1".into(), true))).await;
    (assert_mutation_inverse_law(
        &with_object,
        &add_object_vortex("o1".into(), Puzzle3dVortex { id: "v2".into(), vortex_kind: None, label: None, position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false }, None),
    )).await;
    (assert_mutation_inverse_law(&with_object, &remove_object_vortex("o1".into(), "v1".into()))).await;
    (assert_mutation_inverse_law(
        &with_object,
        &replace_object_vortex("o1".into(), "v1".into(), Puzzle3dVortex { id: "v1".into(), vortex_kind: Some("k".into()), label: None, position: [1.0, 1.0, 1.0], direction: None, radius: None, hidden: false, locked: false }),
    )).await;
}

#[semio_framework_async_macros::async_test]
async fn connect_disconnect_vortices_inverse_law_and_cascade() {
    use {semio_s_artifact_puzzle_3d_model::Puzzle3dObject, semio_s_artifact_puzzle_3d_model::Puzzle3dVortex};
    let base = empty();
    let object_a = Puzzle3dObject {
        id: "a".into(),
        label: None,
        object_kind: None,
        anchor: Default::default(),
        origin: [0.0, 0.0, 0.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: vec![Puzzle3dVortex { id: "va".into(), vortex_kind: None, label: None, position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false }],
        hidden: false,
        locked: false,
    };
    let object_b = Puzzle3dObject {
        id: "b".into(),
        label: None,
        object_kind: None,
        anchor: Default::default(),
        origin: [0.0, 0.0, 0.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: vec![Puzzle3dVortex { id: "vb".into(), vortex_kind: None, label: None, position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false }],
        hidden: false,
        locked: false,
    };
    let mut projection = base;
    projection = protocol::apply_diff_cold(create_object(object_a, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    projection = protocol::apply_diff_cold(create_object(object_b, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    (assert_mutation_inverse_law(&projection, &connect_vortices("t1".into(), "a:va".into(), "b:vb".into(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None))).await;
    let connected = protocol::apply_diff_cold(connect_vortices("t1".into(), "a:va".into(), "b:vb".into(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    (assert_mutation_inverse_law(&connected, &disconnect_vortices("t1".into()))).await;
    (assert_mutation_inverse_law(
        &connected,
        &replace_attraction_geometry(ReplaceAttractionGeometry { id: "t1".into(), new_gap: 1.0, new_shift: 2.0, new_rise: 3.0, new_rotation: 4.0, new_turn: 5.0, new_tilt: 6.0, new_x: 7.0, new_y: 8.0 }),
    )).await;
    let deleted = delete_object("a".into());
    let after_delete = protocol::apply_diff_cold(deleted.diff(&connected).diff(), &connected).expect("valid mutation diff");
    assert!(!after_delete.attractions.iter().any(|attraction| attraction.id == "t1"), "delete-object must sever attractions touching its vortices");
    (assert_mutation_inverse_law(&connected, &deleted)).await;
}

#[semio_framework_async_macros::async_test]
async fn target_volume_and_reference_inverse_law() {
    use {semio_s_artifact_puzzle_3d_model::Puzzle3dReference, semio_s_artifact_puzzle_3d_model::Puzzle3dReferenceSource, semio_s_artifact_puzzle_3d_model::Puzzle3dTargetVolume};
    let base = empty();
    let volume = Puzzle3dTargetVolume { id: "tv1".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, hidden: false, locked: false };
    (assert_mutation_inverse_law(&base, &create_target_volume(volume.clone(), None))).await;
    let with_volume = protocol::apply_diff_cold(create_target_volume(volume, None).diff(&base).diff(), &base).expect("valid mutation diff");
    (assert_mutation_inverse_law(&with_volume, &move_target_volume("tv1".into(), [1.0, 2.0, 3.0]))).await;
    (assert_mutation_inverse_law(&with_volume, &rotate_target_volume("tv1".into(), Some([0.0, 0.0, 0.0, 1.0])))).await;
    (assert_mutation_inverse_law(&with_volume, &scale_target_volume("tv1".into(), None))).await;
    (assert_mutation_inverse_law(&with_volume, &change_target_volume_hidden("tv1".into(), true))).await;
    (assert_mutation_inverse_law(&with_volume, &change_target_volume_locked("tv1".into(), true))).await;
    (assert_mutation_inverse_law(&with_volume, &delete_target_volume("tv1".into()))).await;

    let reference = Puzzle3dReference { id: "r1".into(), source: Puzzle3dReferenceSource::default(), origin: [0.0, 0.0, 0.0], width_world: 1.0, locked: false, hidden: false };
    (assert_mutation_inverse_law(&base, &create_reference(reference.clone(), None))).await;
    let with_reference = protocol::apply_diff_cold(create_reference(reference, None).diff(&base).diff(), &base).expect("valid mutation diff");
    (assert_mutation_inverse_law(&with_reference, &move_reference("r1".into(), [1.0, 2.0, 3.0]))).await;
    (assert_mutation_inverse_law(&with_reference, &resize_reference("r1".into(), 4.0))).await;
    (assert_mutation_inverse_law(&with_reference, &replace_reference_source("r1".into(), Puzzle3dReferenceSource { url: "/x.png".into(), media_kind: Some("image".into()) }))).await;
    (assert_mutation_inverse_law(&with_reference, &change_reference_hidden("r1".into(), true))).await;
    (assert_mutation_inverse_law(&with_reference, &change_reference_locked("r1".into(), true))).await;
    (assert_mutation_inverse_law(&with_reference, &delete_reference("r1".into()))).await;
}

#[semio_framework_async_macros::async_test]
async fn document_scalar_mutations_inverse_law() {
    use {semio_s_artifact_puzzle_3d_model::Puzzle3dCompatSpecificity, semio_s_artifact_puzzle_3d_model::Puzzle3dKindCatalogs};
    let base = empty();
    (assert_mutation_inverse_law(&base, &change_domain("mechanical".into()))).await;
    (assert_mutation_inverse_law(&base, &connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle3dCompatSpecificity::Vortex, None))).await;
    let connected = protocol::apply_diff_cold(connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle3dCompatSpecificity::Vortex, None).diff(&base).diff(), &base).expect("valid mutation diff");
    (assert_mutation_inverse_law(&connected, &disconnect_kind_compatibility("a".into(), "b".into()))).await;
    (assert_mutation_inverse_law(&base, &replace_kind_catalogs(Some(Puzzle3dKindCatalogs::default())))).await;
}

#[test]
fn dispatch_registers_semantic_descriptors() {
    register_puzzle3d_mutation_descriptors(::semio_framework_schema_state::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in <Puzzle3dMutation as protocol::SemanticMutation<Puzzle3dSnapshot>>::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(<Puzzle3dMutation as protocol::SemanticMutation<Puzzle3dSnapshot>>::kinds().len(), 38);
}
//#endregion 🔖️MutationLaws

//#region 🔖️OutcomeLaws
// 🎫️ 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — see
// `📓️w3-f-block-puzzle-report.md` for the `assert_outcome_policy_matrix` pending-helper note.
use protocol::os_spr::protocol_laws::assert_fatal_never_applies;
use protocol::os_spr::protocol_laws::assert_missing_target_is_error;

#[semio_framework_async_macros::async_test]
async fn missing_target_is_error_per_verb_family() {
    let base = empty();
    (assert_missing_target_is_error(&base, &delete_object("missing".into()))).await; // delete
    (assert_missing_target_is_error(&base, &remove_object_vortex("missing".into(), "v0".into()))).await; // remove
    (assert_missing_target_is_error(&base, &change_object_hidden("missing".into(), true))).await; // change/set/update
    (assert_missing_target_is_error(&base, &move_object("missing".into(), [1.0, 1.0, 1.0]))).await; // move/drag/rotate/scale/resize
    (assert_missing_target_is_error(&base, &edit_object_label("missing".into(), Some("x".into())))).await; // edit/replace
    (assert_missing_target_is_error(&base, &disconnect_vortices("missing".into()))).await;
    // disconnect/unbind
}

#[semio_framework_async_macros::async_test]
async fn create_duplicate_id_is_fatal_and_never_applies() {
    use semio_s_artifact_puzzle_3d_model::Puzzle3dObject;
    let mut base = empty();
    let object = Puzzle3dObject { id: "o0".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    base.objects.push(object.clone());
    let outcome = create_object(object, None).diff(&base);
    (assert_fatal_never_applies(&outcome)).await;
    assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.duplicate-id"));
}
//#endregion 🔖️OutcomeLaws

//#region 🔖️SelectionLaws
/// 🧱️ A scene of one object and one target volume for the selection laws.
fn selection_scene(origin: [f64; 3]) -> Puzzle3dSnapshot {
    use {semio_s_artifact_puzzle_3d_model::Puzzle3dObject, semio_s_artifact_puzzle_3d_model::Puzzle3dTargetVolume};
    let mut scene = empty();
    scene.objects.push(Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin, orientation: None, scale: Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Uniform(2.0)), mesh_url: None, vortices: Vec::new(), hidden: false, locked: false });
    scene.target_volumes.push(Puzzle3dTargetVolume { id: "v1".into(), origin, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, hidden: false, locked: false });
    scene
}

/// 🔁️ A selection leaf states intent, not a final state: the same drag replayed on a base where the
/// object already moved lands exactly offset-away from THAT base, and a turn and a scaling compose onto
/// whatever orientation and scale the base carries.
#[test]
fn selection_leaves_replay_on_a_moved_base() {
    for origin in [[0.0, 0.0, 0.0], [10.0, -3.0, 2.5]] {
        let mut scene = selection_scene(origin);
        apply_puzzle3d_mutation(&mut scene, &drag_selection(vec!["o1".into(), "v1".into()], [1.0, 2.0, 3.0])).expect("drag applies");
        assert_eq!(scene.objects[0].origin, [origin[0] + 1.0, origin[1] + 2.0, origin[2] + 3.0]);
        assert_eq!(scene.target_volumes[0].origin, [origin[0] + 1.0, origin[1] + 2.0, origin[2] + 3.0]);
    }
    let mut scene = selection_scene([0.0; 3]);
    apply_puzzle3d_mutation(&mut scene, &rotate_selection(vec!["o1".into()], [0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2)).expect("turn applies");
    apply_puzzle3d_mutation(&mut scene, &rotate_selection(vec!["o1".into()], [0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2)).expect("second turn applies");
    let orientation = scene.objects[0].orientation.expect("a turned object carries an orientation");
    assert!((orientation[2] - 1.0).abs() < 1e-12 && orientation[3].abs() < 1e-12, "two quarter turns about z compose into a half turn, got {orientation:?}");
    apply_puzzle3d_mutation(&mut scene, &scale_selection(vec!["o1".into()], [0.5, 1.0, 3.0])).expect("scaling applies");
    assert_eq!(scene.objects[0].scale, Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Vec3([1.0, 2.0, 6.0])), "a uniform base scale broadcasts before the per-axis factors multiply it");
}

/// 🗣️ Every selection leaf's history label is a real sentence in both locales.
#[test]
fn selection_labels_are_localized() {
    let label = |mutation: Puzzle3dMutation| serde_json::to_string(&<Puzzle3dMutation as protocol::SemanticMutation<Puzzle3dSnapshot>>::label(&mutation)).expect("label serializes");
    let drag = label(drag_selection(vec!["o1".into(), "v1".into()], [1.5, -2.0, 0.0]));
    assert!(drag.contains("Drag 2 items by (1.5, -2, 0)") && drag.contains("2 Elemente um (1,5; -2; 0) ziehen"), "{drag}");
    let turn = label(rotate_selection(vec!["o1".into()], [0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2));
    assert!(turn.contains("Rotate 1 item by 90°") && turn.contains("1 Element um 90° drehen"), "{turn}");
    let scaling = label(scale_selection(vec!["o1".into()], [2.0, 1.0, 0.25]));
    assert!(scaling.contains("Scale 1 item by (2, 1, 0.25)") && scaling.contains("1 Element um (2; 1; 0,25) skalieren"), "{scaling}");
}

/// 🚨️ A selection leaf whose every target is absent is the Error-level `mutation.target-missing`.
#[semio_framework_async_macros::async_test]
async fn selection_missing_targets_are_errors() {
    let base = empty();
    (assert_missing_target_is_error(&base, &drag_selection(vec!["missing".into()], [1.0, 0.0, 0.0]))).await;
    (assert_missing_target_is_error(&base, &rotate_selection(vec!["missing".into()], [0.0, 0.0, 1.0], 1.0))).await;
    (assert_missing_target_is_error(&base, &scale_selection(vec!["missing".into()], [2.0, 2.0, 2.0]))).await;
}

/// ✏️ Each selection leaf is editable through its payload value: the input schema is declared, and
/// the payload round-trips through `payload_value`/`with_payload_value` with an edited offset.
#[test]
fn selection_leaves_are_editable_through_their_payload_value() {
    let drag = drag_selection(vec!["o1".into()], [1.0, 0.0, 0.0]);
    assert!(Mutation::<Puzzle3dSnapshot>::input_schema(&drag).is_some_and(|schema| schema.contains("\"offset\"")));
    let edited = Mutation::<Puzzle3dSnapshot>::with_payload_value(&drag, semio_framework_value::DslValue::from(&serde_json::json!({ "targets": ["o1"], "offset": [4.0, 0.0, 0.0] }))).expect("an edited payload value decodes");
    assert_eq!(edited, drag_selection(vec!["o1".into()], [4.0, 0.0, 0.0]));
}

//#endregion 🔖️SelectionLaws

//#region 🧪️KindsCatalog
/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed oracle
/// manifest's catalog — the framework never parses Rust, so this is the only thing that keeps the
/// declared vocabulary and the measured one from drifting apart.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <Puzzle3dMutation as protocol::SemanticMutation<Puzzle3dSnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared Puzzle3dMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🔮️oracles/🔣️.json")).expect("committed test data reads");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog

/// 📍️ LAW (design wave-2 ruling): deleting or disconnecting a MIDDLE row restores it at its original position, and the
/// inverse diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn middle_row_removals_restore_their_position() {
    use {semio_s_artifact_puzzle_3d_model::Puzzle3dCompatSpecificity, semio_s_artifact_puzzle_3d_model::Puzzle3dObject, semio_s_artifact_puzzle_3d_model::Puzzle3dReference, semio_s_artifact_puzzle_3d_model::Puzzle3dReferenceSource, semio_s_artifact_puzzle_3d_model::Puzzle3dTargetVolume, semio_s_artifact_puzzle_3d_model::Puzzle3dVortex};
    use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
    let step = |base: Puzzle3dSnapshot, mutation: Puzzle3dMutation| protocol::apply_diff_cold(mutation.diff(&base).diff(), &base).expect("valid mutation diff");
    let vortex = |id: &str| Puzzle3dVortex { id: id.into(), vortex_kind: None, label: None, position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false };
    let object = |id: &str| Puzzle3dObject { id: id.into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: vec![vortex("v1"), vortex("v2"), vortex("v3")], hidden: false, locked: false };
    let mut base = empty();
    for id in ["a", "b", "c"] {
        base = step(base, create_object(object(id), None));
        base = step(base, create_target_volume(Puzzle3dTargetVolume { id: id.into(), origin: [0.0; 3], orientation: None, scale: None, hidden: false, locked: false }, None));
        base = step(base, create_reference(Puzzle3dReference { id: id.into(), source: Puzzle3dReferenceSource { url: "u".into(), media_kind: None }, origin: [0.0; 3], width_world: 1.0, locked: false, hidden: false }, None));
        base = step(base, connect_kind_compatibility(id.into(), "z".into(), false, false, Puzzle3dCompatSpecificity::General, None));
    }
    for (id, attracting, attracted) in [("t1", "a:v1", "b:v1"), ("t2", "b:v2", "c:v2"), ("t3", "a:v3", "c:v3")] {
        base = step(base, connect_vortices(id.into(), attracting.into(), attracted.into(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None));
    }
    let removals = [delete_object("b".into()), remove_object_vortex("b".into(), "v2".into()), disconnect_vortices("t2".into()), disconnect_kind_compatibility("b".into(), "z".into()), delete_target_volume("b".into()), delete_reference("b".into())];
    for mutation in &removals {
        assert_mutation_inverse_law(&base, mutation).await;
        assert_mutation_inverse_sum_law(mutation, &base).await;
    }
}

/// 🧫️ Language-neutral roots retain typed mutation authority and match Serde's admitted values.
#[test]
fn puzzle3d_typed_mutation_fixture_and_refusal_law() {
    let fixture: serde_json::Value=serde_json::from_str(std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/📸️typed-mutation/🔣️.json")).expect("committed test data reads").as_str()).expect("neutral fixture");
    for case in fixture["cases"].as_array().expect("cases") {
        let before: Puzzle3dSnapshot=semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&case["before"])).expect("base admits");
        let after: Puzzle3dSnapshot=semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&case["after"])).expect("result admits");
        let oracle_before: Puzzle3dSnapshot=serde_json::from_value(case["before"].clone()).expect("third-party base");
        let oracle_after: Puzzle3dSnapshot=serde_json::from_value(case["after"].clone()).expect("third-party result");
        assert_eq!(before,oracle_before);
        assert_eq!(after,oracle_after);
        let operations: Vec<Puzzle3dMutation>=serde_json::from_value(case["operations"].clone()).expect("authored concrete mutations");
        assert!(operations.len()>=case["minimumOperations"].as_u64().expect("count") as usize);
        let mut current=before.clone();
        let mut inverse=Vec::new();
        for operation in &operations {
            inverse.extend(Mutation::<Puzzle3dSnapshot>::inverse(operation,&current).expect("inverse"));
            current=protocol::apply_diff_cold(Mutation::<Puzzle3dSnapshot>::diff(operation,&current).diff(),&current).expect("typed apply");
        }
        assert_eq!(current,oracle_after);
        for operation in inverse.iter().rev() {
            current=protocol::apply_diff_cold(Mutation::<Puzzle3dSnapshot>::diff(operation,&current).diff(),&current).expect("typed inverse apply");
        }
        assert_eq!(current,oracle_before);
    }
    for value in fixture["invalid"].as_array().expect("invalid") {
        let actual= <Puzzle3dSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(value));
        let oracle=serde_json::from_value::<Puzzle3dSnapshot>(value.clone());
        assert!(actual.is_err());
        assert!(oracle.is_err());
    }
}

/// 🔢️ The shared native field corpus preserves the complete scalar words through typed patches.
#[test]
fn puzzle3d_native_float_typed_mutation_fixture() {
    use semio_s_artifact_puzzle_3d_diff::Puzzle3dObjectPatch;
    let fixture:serde_json::Value=serde_json::from_str(std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🔺️diff/🔢️native-fields/🔣️.json")).expect("committed test data reads").as_str()).expect("neutral words");
    for case in fixture["cases"].as_array().expect("cases") {
        let words:Vec<u64>=case["words"].as_array().expect("words").iter().map(|word|word.as_str().expect("decimal word").parse().expect("unsigned word")).collect();
        let value=semio_framework_value::DslValue::object([("origin".into(),semio_framework_value::DslValue::Array(words.iter().map(|word|semio_framework_value::DslValue::float(f64::from_bits(*word))).collect()))]);
        let patch=<Puzzle3dObjectPatch as semio_framework_value::FromValue>::from_value(value).expect("typed patch");
        let actual:Vec<String>=patch.origin.expect("native origin").iter().map(|number|number.to_bits().to_string()).collect();
        assert_eq!(serde_json::to_value(actual).expect("third-party output"),case["words"]);
    }
}
