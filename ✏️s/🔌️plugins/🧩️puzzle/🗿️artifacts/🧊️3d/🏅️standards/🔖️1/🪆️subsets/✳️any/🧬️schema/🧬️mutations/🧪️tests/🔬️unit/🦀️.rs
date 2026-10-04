
use super::*;

#[test]
fn puzzle3d_delta_ops_round_trip_and_stay_granular() {
    let before = serde_json::json!({
        "schema": crate::PUZZLE_3D_SCHEMA, "domain": "architecture",
        "meta": {},
        "objects": [
            { "id": "o1", "anchor": "fixed", "origin": [0.0,0.0,0.0], "vortices": [] },
            { "id": "o2", "anchor": "fixed", "origin": [1.0,0.0,0.0], "vortices": [] },
        ],
        "attractions": [], "targetVolumes": [], "references": [],
    });
    let after = serde_json::json!({
        "schema": crate::PUZZLE_3D_SCHEMA, "domain": "architecture",
        "meta": {},
        "objects": [
            { "id": "o2", "anchor": "fixed", "origin": [9.0,0.0,0.0], "vortices": [] },
            { "id": "o3", "anchor": "fixed", "origin": [2.0,0.0,0.0], "vortices": [] },
        ],
        "attractions": [], "targetVolumes": [], "references": [],
    });
    let canonical = |value: &Value| serde_json::to_value(serde_json::from_value::<Puzzle3dSnapshot>(value.clone()).expect("typed puzzle3d fixture")).expect("canonical puzzle3d JSON");
    let operations = puzzle3d_document_delta_operations(&before, &after);
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle3dMutation::MoveObject(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle3dMutation::CreateObject(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle3dMutation::DeleteObject(_))));
    let mut forward = before.clone();
    let mut inverses = Vec::new();
    for operation in &operations {
        inverses.extend(Mutation::<Value>::inverse(operation, &forward).expect("valid retained mutation inverse fixture"));
        forward = Mutation::<Value>::diff(operation, &forward).diff().apply(&forward).expect("valid mutation diff");
    }
    assert_eq!(forward, canonical(&after));
    for inverse in inverses.iter().rev() {
        forward = Mutation::<Value>::diff(inverse, &forward).diff().apply(&forward).expect("valid mutation diff");
    }
    assert_eq!(forward, canonical(&before), "backwards operations must restore the pre-edit document");
}

//#region 🔖️MutationLaws
use protocol::os_spr::protocol_laws::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};

#[semio_framework_async_macros::async_test]
async fn move_object_diff_absorb_law() {
    use crate::Puzzle3dObject;
    let base = empty();
    let object = Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    let with_object = MutationDiff::<Puzzle3dSnapshot>::apply(create_object(object, None).diff(&base).diff(), &base).expect("valid mutation diff");
    let d1 = move_object("o1".into(), [10.0, 10.0, 10.0]).diff(&with_object).into_parts().0;
    let mid = MutationDiff::<Puzzle3dSnapshot>::apply(&d1, &with_object).expect("valid mutation diff");
    let d2 = move_object("o1".into(), [20.0, 30.0, 40.0]).diff(&mid).into_parts().0;
    (assert_mutation_diff_absorb_law(&with_object, d1, d2)).await;
}

fn empty() -> Puzzle3dSnapshot {
    Puzzle3dSnapshot::default()
}

#[semio_framework_async_macros::async_test]
async fn create_delete_object_inverse_law() {
    use crate::Puzzle3dObject;
    let base = empty();
    let object = Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    (assert_mutation_inverse_law(&base, &create_object(object.clone(), None))).await;
    let with_object = MutationDiff::<Puzzle3dSnapshot>::apply(create_object(object, None).diff(&base).diff(), &base).expect("valid mutation diff");
    (assert_mutation_inverse_law(&with_object, &delete_object("o1".into()))).await;
}

#[semio_framework_async_macros::async_test]
async fn object_field_mutations_inverse_law() {
    use crate::{Puzzle3dObject, Puzzle3dObjectAnchor, Puzzle3dScale, Puzzle3dVortex};
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
    let with_object = MutationDiff::<Puzzle3dSnapshot>::apply(create_object(object, None).diff(&base).diff(), &base).expect("valid mutation diff");
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
    use crate::{Puzzle3dObject, Puzzle3dVortex};
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
    projection = MutationDiff::<Puzzle3dSnapshot>::apply(create_object(object_a, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    projection = MutationDiff::<Puzzle3dSnapshot>::apply(create_object(object_b, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    (assert_mutation_inverse_law(&projection, &connect_vortices("t1".into(), "a:va".into(), "b:vb".into(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0))).await;
    let connected = MutationDiff::<Puzzle3dSnapshot>::apply(connect_vortices("t1".into(), "a:va".into(), "b:vb".into(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0).diff(&projection).diff(), &projection).expect("valid mutation diff");
    (assert_mutation_inverse_law(&connected, &disconnect_vortices("t1".into()))).await;
    (assert_mutation_inverse_law(
        &connected,
        &replace_attraction_geometry(ReplaceAttractionGeometry { id: "t1".into(), new_gap: 1.0, new_shift: 2.0, new_rise: 3.0, new_rotation: 4.0, new_turn: 5.0, new_tilt: 6.0, new_x: 7.0, new_y: 8.0 }),
    )).await;
    let deleted = delete_object("a".into());
    let after_delete = MutationDiff::<Puzzle3dSnapshot>::apply(deleted.diff(&connected).diff(), &connected).expect("valid mutation diff");
    assert!(!after_delete.attractions.iter().any(|attraction| attraction.id == "t1"), "delete-object must sever attractions touching its vortices");
    (assert_mutation_inverse_law(&connected, &deleted)).await;
}

#[semio_framework_async_macros::async_test]
async fn target_volume_and_reference_inverse_law() {
    use crate::{Puzzle3dReference, Puzzle3dReferenceSource, Puzzle3dTargetVolume};
    let base = empty();
    let volume = Puzzle3dTargetVolume { id: "tv1".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, hidden: false, locked: false };
    (assert_mutation_inverse_law(&base, &create_target_volume(volume.clone(), None))).await;
    let with_volume = MutationDiff::<Puzzle3dSnapshot>::apply(create_target_volume(volume, None).diff(&base).diff(), &base).expect("valid mutation diff");
    (assert_mutation_inverse_law(&with_volume, &move_target_volume("tv1".into(), [1.0, 2.0, 3.0]))).await;
    (assert_mutation_inverse_law(&with_volume, &rotate_target_volume("tv1".into(), Some([0.0, 0.0, 0.0, 1.0])))).await;
    (assert_mutation_inverse_law(&with_volume, &scale_target_volume("tv1".into(), None))).await;
    (assert_mutation_inverse_law(&with_volume, &change_target_volume_hidden("tv1".into(), true))).await;
    (assert_mutation_inverse_law(&with_volume, &change_target_volume_locked("tv1".into(), true))).await;
    (assert_mutation_inverse_law(&with_volume, &delete_target_volume("tv1".into()))).await;

    let reference = Puzzle3dReference { id: "r1".into(), source: Puzzle3dReferenceSource::default(), origin: [0.0, 0.0, 0.0], width_world: 1.0, locked: false, hidden: false };
    (assert_mutation_inverse_law(&base, &create_reference(reference.clone(), None))).await;
    let with_reference = MutationDiff::<Puzzle3dSnapshot>::apply(create_reference(reference, None).diff(&base).diff(), &base).expect("valid mutation diff");
    (assert_mutation_inverse_law(&with_reference, &move_reference("r1".into(), [1.0, 2.0, 3.0]))).await;
    (assert_mutation_inverse_law(&with_reference, &resize_reference("r1".into(), 4.0))).await;
    (assert_mutation_inverse_law(&with_reference, &replace_reference_source("r1".into(), Puzzle3dReferenceSource { url: "/x.png".into(), media_kind: Some("image".into()) }))).await;
    (assert_mutation_inverse_law(&with_reference, &change_reference_hidden("r1".into(), true))).await;
    (assert_mutation_inverse_law(&with_reference, &change_reference_locked("r1".into(), true))).await;
    (assert_mutation_inverse_law(&with_reference, &delete_reference("r1".into()))).await;
}

#[semio_framework_async_macros::async_test]
async fn document_scalar_mutations_inverse_law() {
    use crate::{Puzzle3dCompatSpecificity, Puzzle3dKindCatalogs};
    let base = empty();
    (assert_mutation_inverse_law(&base, &change_domain("mechanical".into()))).await;
    (assert_mutation_inverse_law(&base, &connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle3dCompatSpecificity::Vortex))).await;
    let connected = MutationDiff::<Puzzle3dSnapshot>::apply(connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle3dCompatSpecificity::Vortex).diff(&base).diff(), &base).expect("valid mutation diff");
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
use protocol::os_spr::protocol_laws::{assert_fatal_never_applies, assert_missing_target_is_error};

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
    use crate::Puzzle3dObject;
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
    use crate::{Puzzle3dObject, Puzzle3dTargetVolume};
    let mut scene = empty();
    scene.objects.push(Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin, orientation: None, scale: Some(crate::Puzzle3dScale::Uniform(2.0)), mesh_url: None, vortices: Vec::new(), hidden: false, locked: false });
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
    assert_eq!(scene.objects[0].scale, Some(crate::Puzzle3dScale::Vec3([1.0, 2.0, 6.0])), "a uniform base scale broadcasts before the per-axis factors multiply it");
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

/// ⏪️ Time travel edits a selection leaf's inputs, never the gesture: a drag superseded with a new offset
/// previews as the state before it plus the draft, and its Report replay re-applies every
/// downstream transform onto the edited pose — exactly the fresh fold of the edited log.
#[semio_framework_async_macros::async_test]
async fn a_drag_edited_in_history_replays_its_downstream() {
    use protocol::OpBinary;
    let mut store = crate::standards::v1::subsets::any::schema::mutations::binary::puzzle3d_store(store::create_document_envelope::<Puzzle3dSnapshot, Puzzle3dMutation>(crate::PUZZLE_3D_SCHEMA, "selection-time-travel", selection_scene([0.0; 3]), None)).await.expect("the store opens");
    let log = [drag_selection(vec!["o1".into()], [1.0, 0.0, 0.0]), rotate_selection(vec!["o1".into()], [0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2), drag_selection(vec!["o1".into(), "v1".into()], [0.0, 2.0, 0.0])];
    for mutation in &log {
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation.clone()], description: None, transaction: None }).await.expect("a selection gesture applies");
    }
    let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
    let edited = drag_selection(vec!["o1".into()], [5.0, 0.0, 0.0]);
    let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(ids[0].clone(), protocol::InputReplacement::Input { schema: crate::PUZZLE_3D_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
    let mut preview = store.state_before(&ids[0], &drafts).expect("the preview base folds").as_ref().clone();
    assert_eq!(preview.objects[0].origin, [0.0; 3], "the preview base is the state right before the edited drag");
    apply_puzzle3d_mutation(&mut preview, &edited).expect("the draft applies to its base");
    assert_eq!((preview.objects[0].origin, preview.objects[0].orientation), ([5.0, 0.0, 0.0], None), "the preview shows the draft and nothing downstream");
    let mut replay = store.begin_report_replay(&drafts, Some(&ids[0])).expect("the replay begins at the edited drag");
    assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)));
    let result = replay.finish().expect("a finished replay yields its result");
    assert!(!store.replay_report(&result).expect("report").blocks_finalize(), "a clean edit never blocks finalizing");
    let mut fresh = selection_scene([0.0; 3]);
    for mutation in [edited, log[1].clone(), log[2].clone()] {
        apply_puzzle3d_mutation(&mut fresh, &mutation).expect("the edited log folds");
    }
    assert_eq!(result.state().expect("the replay reached a state").as_ref(), &fresh, "the replay equals the fresh fold of the edited log");
    assert_eq!(fresh.objects[0].origin, [5.0, 2.0, 0.0], "the downstream drag lands on the edited pose");
    store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
    assert_eq!(store.snapshot_ref(), &fresh, "the overwritten history folds to the edited state");
    crate::standards::v1::subsets::any::schema::mutations::binary::close_puzzle3d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
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
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog

#[test]
fn play_snapshot_pack_shares_the_typed_record_identity_and_round_trips() {
    let spec = <Puzzle3dPlaySnapshot as store::ArtifactPack>::record_spec().expect("play snapshot declares its record spec");
    assert_eq!(store::os_pack::schema_hash(&spec), store::os_pack::schema_hash(&Puzzle3dSnapshot::__dsl_spec()));
    assert_ne!(store::os_pack::schema_hash(&spec), [0u8; 32]);
    let play = Puzzle3dPlaySnapshot::from_typed(Puzzle3dSnapshot::default());
    let bytes = store::ArtifactPack::encode_pack(&play);
    assert_eq!(bytes, store::ArtifactPack::encode_pack(play.typed()));
    assert_eq!(<Puzzle3dPlaySnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode play pack"), play);
}
