
use super::*;

#[test]
fn puzzle5d_delta_ops_round_trip_and_stay_granular() {
    let before = serde_json::json!({
        "schema": crate::PUZZLE_5D_SCHEMA, "domain": "architecture",
        "meta": { "description": "" },
        "parts": [
            { "id": "p1", "2d": { "x": 0.0, "y": 0.0 }, "3d": { "origin": [0.0,0.0,0.0] }, "grips": [] },
            { "id": "p2", "2d": { "x": 1.0, "y": 0.0 }, "3d": { "origin": [1.0,0.0,0.0] }, "grips": [] },
        ],
        "fasteners": [],
    });
    let after = serde_json::json!({
        "schema": crate::PUZZLE_5D_SCHEMA, "domain": "architecture",
        "meta": { "description": "" },
        "parts": [
            { "id": "p2", "2d": { "x": 9.0, "y": 0.0 }, "3d": { "origin": [9.0,0.0,0.0] }, "grips": [] },
            { "id": "p3", "2d": { "x": 2.0, "y": 0.0 }, "3d": { "origin": [2.0,0.0,0.0] }, "grips": [] },
        ],
        "fasteners": [],
    });
    let canonical = |value: &Value| {
        let snapshot: Puzzle5dSnapshot = semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("typed puzzle5d fixture");
        serde_json::from_str::<Value>(&semio_framework_pack_json::to_json_string(&snapshot)).expect("canonical puzzle5d JSON")
    };
    let operations = puzzle5d_document_delta_operations(&before, &after);
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::MovePart2d(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::CreatePart(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::DeletePart(_))));
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
async fn move_part_2d_diff_absorb_law() {
    use crate::Puzzle5dPart;
    let base = empty();
    let part = Puzzle5dPart { id: "p1".into(), ..Default::default() };
    let with_part = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part, None).diff(&base).diff(), &base).expect("valid mutation diff");
    let d1 = move_part_2d("p1".into(), 10.0, 10.0).diff(&with_part).into_parts().0;
    let mid = MutationDiff::<Puzzle5dSnapshot>::apply(&d1, &with_part).expect("valid mutation diff");
    let d2 = move_part_2d("p1".into(), 20.0, 30.0).diff(&mid).into_parts().0;
    (assert_mutation_diff_absorb_law(&with_part, d1, d2)).await;
}

fn empty() -> Puzzle5dSnapshot {
    Puzzle5dSnapshot::default()
}

#[semio_framework_async_macros::async_test]
async fn create_delete_part_inverse_law() {
    use crate::Puzzle5dPart;
    let base = empty();
    let part = Puzzle5dPart { id: "p1".into(), ..Default::default() };
    (assert_mutation_inverse_law(&base, &create_part(part.clone(), None))).await;
    let with_part = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part, None).diff(&base).diff(), &base).expect("valid mutation diff");
    (assert_mutation_inverse_law(&with_part, &delete_part("p1".into()))).await;
}

#[semio_framework_async_macros::async_test]
async fn part_field_mutations_inverse_law() {
    use crate::{Puzzle5dGrip, Puzzle5dPart, Puzzle5dPartAnchor, Puzzle5dScale};
    let base = empty();
    let part = Puzzle5dPart { id: "p1".into(), grips: vec![Puzzle5dGrip { id: "g1".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Default::default() }], ..Default::default() };
    let with_part = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part, None).diff(&base).diff(), &base).expect("valid mutation diff");
    (assert_mutation_inverse_law(&with_part, &move_part_2d("p1".into(), 5.0, 6.0))).await;
    (assert_mutation_inverse_law(&with_part, &replace_part_2d_geometry("p1".into(), Some("rectangle".into()), None, Some(4.0), Some(2.0)))).await;
    (assert_mutation_inverse_law(&with_part, &edit_part_2d_text("p1".into(), Some("hi".into())))).await;
    (assert_mutation_inverse_law(&with_part, &change_part_2d_icon("p1".into(), Some("star".into())))).await;
    (assert_mutation_inverse_law(&with_part, &change_part_2d_hidden("p1".into(), Some(true)))).await;
    (assert_mutation_inverse_law(&with_part, &change_part_2d_locked("p1".into(), Some(true)))).await;
    (assert_mutation_inverse_law(&with_part, &move_part_3d("p1".into(), [1.0, 2.0, 3.0]))).await;
    (assert_mutation_inverse_law(&with_part, &rotate_part_3d("p1".into(), Some([0.0, 0.0, 0.0, 1.0])))).await;
    (assert_mutation_inverse_law(&with_part, &scale_part_3d("p1".into(), Some(Puzzle5dScale::Uniform(2.0))))).await;
    (assert_mutation_inverse_law(&with_part, &change_part_3d_mesh("p1".into(), Some("mesh://a".into())))).await;
    (assert_mutation_inverse_law(&with_part, &edit_part_3d_label("p1".into(), Some("Label".into())))).await;
    (assert_mutation_inverse_law(&with_part, &change_part_kind("p1".into(), Some("core.capsule".into())))).await;
    (assert_mutation_inverse_law(&with_part, &change_part_anchor("p1".into(), Puzzle5dPartAnchor::Derived))).await;
    (assert_mutation_inverse_law(&with_part, &add_part_grip("p1".into(), Puzzle5dGrip { id: "g2".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Default::default() }, None))).await;
    (assert_mutation_inverse_law(&with_part, &remove_part_grip("p1".into(), "g1".into()))).await;
    (assert_mutation_inverse_law(&with_part, &replace_part_grip("p1".into(), "g1".into(), Puzzle5dGrip { id: "g1".into(), grip_kind: Some("k".into()), grip_2d: Default::default(), grip_3d: Default::default() }))).await;
}

#[semio_framework_async_macros::async_test]
async fn connect_disconnect_grips_inverse_law_and_cascade() {
    use crate::{Puzzle5dGrip, Puzzle5dPart};
    let base = empty();
    let part_a = Puzzle5dPart { id: "a".into(), grips: vec![Puzzle5dGrip { id: "ga".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Default::default() }], ..Default::default() };
    let part_b = Puzzle5dPart { id: "b".into(), grips: vec![Puzzle5dGrip { id: "gb".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Default::default() }], ..Default::default() };
    let mut projection = base;
    projection = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part_a, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    projection = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part_b, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    (assert_mutation_inverse_law(&projection, &connect_grips("f1".into(), "a:ga".into(), "b:gb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0))).await;
    let connected = MutationDiff::<Puzzle5dSnapshot>::apply(connect_grips("f1".into(), "a:ga".into(), "b:gb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0).diff(&projection).diff(), &projection).expect("valid mutation diff");
    (assert_mutation_inverse_law(&connected, &disconnect_grips("f1".into()))).await;
    (assert_mutation_inverse_law(
        &connected,
        &replace_fastener_geometry(ReplaceFastenerGeometry { id: "f1".into(), new_gap: 1.0, new_shift: 2.0, new_rise: 3.0, new_rotation: 4.0, new_turn: 5.0, new_tilt: 6.0, new_x: 7.0, new_y: 8.0 }),
    )).await;
    (assert_mutation_inverse_law(&connected, &change_fastener_kind("f1".into(), Some("core.link".into())))).await;
    let deleted = delete_part("a".into());
    let after_delete = MutationDiff::<Puzzle5dSnapshot>::apply(deleted.diff(&connected).diff(), &connected).expect("valid mutation diff");
    assert!(!after_delete.fasteners.iter().any(|fastener| fastener.id == "f1"), "delete-part must sever fasteners touching its grips");
    (assert_mutation_inverse_law(&connected, &deleted)).await;
}

#[semio_framework_async_macros::async_test]
async fn document_scalar_mutations_inverse_law() {
    use crate::{Puzzle5dCompatSpecificity, Puzzle5dKindCatalogs};
    let base = empty();
    (assert_mutation_inverse_law(&base, &rename_puzzle5d(Some("Nakagin".into())))).await;
    (assert_mutation_inverse_law(&base, &change_domain("mechanical".into()))).await;
    (assert_mutation_inverse_law(&base, &change_description("a scene".into()))).await;
    (assert_mutation_inverse_law(&base, &connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle5dCompatSpecificity::Grip))).await;
    let connected = MutationDiff::<Puzzle5dSnapshot>::apply(connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle5dCompatSpecificity::Grip).diff(&base).diff(), &base).expect("valid mutation diff");
    (assert_mutation_inverse_law(&connected, &disconnect_kind_compatibility("a".into(), "b".into()))).await;
    (assert_mutation_inverse_law(&base, &replace_kind_catalogs(Some(Puzzle5dKindCatalogs::default())))).await;
}

#[test]
fn dispatch_registers_semantic_descriptors() {
    register_puzzle5d_mutation_descriptors(::semio_framework_schema_state::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in <Puzzle5dMutation as protocol::SemanticMutation<Puzzle5dSnapshot>>::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(<Puzzle5dMutation as protocol::SemanticMutation<Puzzle5dSnapshot>>::kinds().len(), 39);
}
//#endregion 🔖️MutationLaws

//#region 🔖️OutcomeLaws
// 🎫️ 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — see
// `📓️w3-f-block-puzzle-report.md` for the `assert_outcome_policy_matrix` pending-helper note.
use protocol::os_spr::protocol_laws::{assert_fatal_never_applies, assert_missing_target_is_error};

#[semio_framework_async_macros::async_test]
async fn missing_target_is_error_per_verb_family() {
    let base = empty();
    (assert_missing_target_is_error(&base, &delete_part("missing".into()))).await; // delete
    (assert_missing_target_is_error(&base, &remove_part_grip("missing".into(), "g0".into()))).await; // remove
    (assert_missing_target_is_error(&base, &change_part_2d_icon("missing".into(), Some("star".into())))).await; // change/set/update
    (assert_missing_target_is_error(&base, &move_part_2d("missing".into(), 1.0, 1.0))).await; // move/drag/rotate/scale/resize
    (assert_missing_target_is_error(&base, &edit_part_3d_label("missing".into(), Some("x".into())))).await; // edit/replace
    (assert_missing_target_is_error(&base, &disconnect_grips("missing".into()))).await;
    // disconnect/unbind
}

#[semio_framework_async_macros::async_test]
async fn create_duplicate_id_is_fatal_and_never_applies() {
    use crate::Puzzle5dPart;
    let mut base = empty();
    let part = Puzzle5dPart { id: "p0".into(), ..Default::default() };
    base.parts.push(part.clone());
    let outcome = create_part(part, None).diff(&base);
    (assert_fatal_never_applies(&outcome)).await;
    assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.duplicate-id"));
}
//#endregion 🔖️OutcomeLaws

//#region 🔖️SelectionLaws
/// 🧱️ One part placed on the board at `(x, 0)` and in the world at `origin`, plus one target volume at `origin`.
fn selection_scene(x: f64, origin: [f64; 3]) -> Puzzle5dSnapshot {
    let mut scene = empty();
    scene.parts.push(crate::Puzzle5dPart { id: "p1".into(), part_2d: crate::Puzzle5dPart2d { x, ..Default::default() }, part_3d: crate::Puzzle5dPart3d { origin, scale: Some(crate::Puzzle5dScale::Uniform(2.0)), ..Default::default() }, ..Default::default() });
    scene.target_volumes.push(crate::Puzzle5dTargetVolume { id: "v1".into(), origin, ..Default::default() });
    scene
}

/// 🔁️ A selection leaf states intent, not a final state: the same drag replayed on a moved base lands exactly
/// offset-away from THAT base — on the board for the 2d leaf, in the world for the 3d one. A board drag never touches
/// the world pose; a world drag carries the board pin along its ground-plane motion.
#[test]
fn selection_leaves_replay_on_a_moved_base() {
    for (x, origin) in [(0.0, [0.0; 3]), (40.0, [10.0, -3.0, 2.5])] {
        let mut scene = selection_scene(x, origin);
        apply_puzzle5d_mutation(&mut scene, &drag_selection_2d(vec!["p1".into()], 5.0, -2.5)).expect("board drag applies");
        apply_puzzle5d_mutation(&mut scene, &drag_selection_3d(vec!["p1".into(), "v1".into()], [1.0, 2.0, 3.0])).expect("world drag applies");
        assert_eq!((scene.parts[0].part_2d.x, scene.parts[0].part_2d.y), (x + 5.0 + 1.0 / PUZZLE5D_FLAT_TO_WORLD, -2.5 - 2.0 / PUZZLE5D_FLAT_TO_WORLD), "the board drag moves the pin, and the world drag carries it along its ground-plane motion");
        assert_eq!(scene.parts[0].part_3d.origin, [origin[0] + 1.0, origin[1] + 2.0, origin[2] + 3.0]);
        assert_eq!(scene.target_volumes[0].origin, [origin[0] + 1.0, origin[1] + 2.0, origin[2] + 3.0]);
    }
    let mut scene = selection_scene(0.0, [0.0; 3]);
    apply_puzzle5d_mutation(&mut scene, &scale_selection_3d(vec!["p1".into()], [0.5, 1.0, 3.0])).expect("scaling applies");
    assert_eq!(scene.parts[0].part_3d.scale, Some(crate::Puzzle5dScale::Vec3([1.0, 2.0, 6.0])), "a uniform base scale broadcasts before the per-axis factors multiply it");
    let outcome = drag_selection_2d(vec!["v1".into()], 1.0, 1.0).diff(&scene);
    assert_eq!(outcome.messages().iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>(), vec!["mutation.target-missing"], "the board paints no target volume, so a board drag of one reaches nothing");
}

/// 🗣️ Every selection leaf's history label is a real sentence in both locales.
#[test]
fn selection_labels_are_localized() {
    let label = |mutation: Puzzle5dMutation| serde_json::to_string(&<Puzzle5dMutation as protocol::SemanticMutation<Puzzle5dSnapshot>>::label(&mutation)).expect("label serializes");
    let board = label(drag_selection_2d(vec!["p1".into(), "p2".into()], 5.0, -2.5));
    assert!(board.contains("Drag 2 items by (5, -2.5) on the board") && board.contains("2 Elemente auf dem Brett um (5; -2,5) ziehen"), "{board}");
    let world = label(drag_selection_3d(vec!["p1".into()], [1.5, 0.0, -1.0]));
    assert!(world.contains("Drag 1 item by (1.5, 0, -1)") && world.contains("1 Element um (1,5; 0; -1) ziehen"), "{world}");
    let turn = label(rotate_selection_3d(vec!["p1".into()], [0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2));
    assert!(turn.contains("Rotate 1 item by 90°") && turn.contains("1 Element um 90° drehen"), "{turn}");
    let scaling = label(scale_selection_3d(vec!["p1".into()], [2.0, 1.0, 0.25]));
    assert!(scaling.contains("Scale 1 item by (2, 1, 0.25)") && scaling.contains("1 Element um (2; 1; 0,25) skalieren"), "{scaling}");
}

/// 🚨️ A selection leaf whose every target is absent is the Error-level `mutation.target-missing`.
#[semio_framework_async_macros::async_test]
async fn selection_missing_targets_are_errors() {
    let base = empty();
    (assert_missing_target_is_error(&base, &drag_selection_2d(vec!["missing".into()], 1.0, 0.0))).await;
    (assert_missing_target_is_error(&base, &drag_selection_3d(vec!["missing".into()], [1.0, 0.0, 0.0]))).await;
    (assert_missing_target_is_error(&base, &rotate_selection_3d(vec!["missing".into()], [0.0, 0.0, 1.0], 1.0))).await;
    (assert_missing_target_is_error(&base, &scale_selection_3d(vec!["missing".into()], [2.0, 2.0, 2.0]))).await;
}

/// ⏪️ Time travel edits a selection leaf's inputs, never the gesture: a board drag superseded with a new offset
/// previews as the state before it plus the draft, and its Report replay re-applies every downstream
/// transform onto the edited pose — exactly the fresh fold of the edited log.
#[semio_framework_async_macros::async_test]
async fn a_board_drag_edited_in_history_replays_its_downstream() {
    use protocol::OpBinary;
    let mut store = crate::host::owned::puzzle5d_store(store::create_document_envelope::<Puzzle5dSnapshot, Puzzle5dMutation>(crate::PUZZLE_5D_SCHEMA, "selection-time-travel", selection_scene(0.0, [0.0; 3]), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("the store opens");
    let log = [drag_selection_2d(vec!["p1".into()], 1.0, 0.0), drag_selection_3d(vec!["p1".into()], [0.0, 0.0, 4.0]), drag_selection_2d(vec!["p1".into()], 0.0, 2.0)];
    for mutation in &log {
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation.clone()], transaction: None }).await.expect("a selection gesture applies");
    }
    let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
    let edited = drag_selection_2d(vec!["p1".into()], 7.0, 0.0);
    let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(ids[0].clone(), protocol::InputReplacement::Input { schema: crate::PUZZLE_5D_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
    let mut preview = store.state_before(&ids[0], &drafts).expect("the preview base folds").as_ref().clone();
    assert_eq!(preview.parts[0].part_2d.x, 0.0, "the preview base is the state right before the edited drag");
    apply_puzzle5d_mutation(&mut preview, &edited).expect("the draft applies to its base");
    assert_eq!((preview.parts[0].part_2d.x, preview.parts[0].part_3d.origin), (7.0, [0.0; 3]), "the preview shows the draft and nothing downstream");
    let mut replay = store.begin_report_replay(&drafts, Some(&ids[0])).expect("the replay begins at the edited drag");
    assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)));
    let result = replay.finish().expect("a finished replay yields its result");
    assert!(!store.replay_report(&result).expect("report").blocks_finalize(), "a clean edit never blocks finalizing");
    let mut fresh = selection_scene(0.0, [0.0; 3]);
    for mutation in [edited, log[1].clone(), log[2].clone()] {
        apply_puzzle5d_mutation(&mut fresh, &mutation).expect("the edited log folds");
    }
    assert_eq!(result.state().expect("the replay reached a state").as_ref(), &fresh, "the replay equals the fresh fold of the edited log");
    assert_eq!((fresh.parts[0].part_2d.x, fresh.parts[0].part_2d.y, fresh.parts[0].part_3d.origin), (7.0, 2.0, [0.0, 0.0, 4.0]));
    store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
    assert_eq!(store.snapshot_ref(), &fresh, "the overwritten history folds to the edited state");
    crate::host::owned::close_puzzle5d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
}
//#endregion 🔖️SelectionLaws

//#region 🧪️KindsCatalog
/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed oracle
/// manifest's catalog — the framework never parses Rust, so this is the only thing that keeps the
/// declared vocabulary and the measured one from drifting apart.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <Puzzle5dMutation as protocol::SemanticMutation<Puzzle5dSnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared Puzzle5dMutation variant");
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
    let spec = <Puzzle5dPlaySnapshot as store::ArtifactPack>::record_spec().expect("play snapshot declares its record spec");
    assert_eq!(store::os_pack::schema_hash(&spec), store::os_pack::schema_hash(&Puzzle5dSnapshot::__dsl_spec()));
    assert_ne!(store::os_pack::schema_hash(&spec), [0u8; 32]);
    let play = Puzzle5dPlaySnapshot::from_typed(Puzzle5dSnapshot::default());
    let bytes = store::ArtifactPack::encode_pack(&play);
    assert_eq!(bytes, store::ArtifactPack::encode_pack(play.typed()));
    assert_eq!(<Puzzle5dPlaySnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode play pack"), play);
}
