
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
        let snapshot: Puzzle5dSnapshot = dsl::json::from_json_str(&value.to_string()).expect("typed puzzle5d fixture");
        serde_json::from_str::<Value>(&dsl::json::to_json_string(&snapshot)).expect("canonical puzzle5d JSON")
    };
    let operations = puzzle5d_document_delta_operations(&before, &after);
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::MovePart2d(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::CreatePart(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::DeletePart(_))));
    let mut forward = before.clone();
    let mut inverses = Vec::new();
    for operation in &operations {
        inverses.extend(Mutation::<Value>::inverse(operation, &forward));
        forward = Mutation::<Value>::diff(operation, &forward).diff().apply(&forward).expect("valid mutation diff");
    }
    assert_eq!(forward, canonical(&after));
    for inverse in inverses.iter().rev() {
        forward = Mutation::<Value>::diff(inverse, &forward).diff().apply(&forward).expect("valid mutation diff");
    }
    assert_eq!(forward, canonical(&before), "backwards operations must restore the pre-edit document");
}

//#region 🔖️MutationLaws
use protocol::os_spr::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};

#[test]
fn move_part_2d_diff_absorb_law() {
    use crate::Puzzle5dPart;
    let base = empty();
    let part = Puzzle5dPart { id: "p1".into(), ..Default::default() };
    let with_part = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part, None).diff(&base).diff(), &base).expect("valid mutation diff");
    let d1 = move_part_2d("p1".into(), 10.0, 10.0).diff(&with_part).into_parts().0;
    let mid = MutationDiff::<Puzzle5dSnapshot>::apply(&d1, &with_part).expect("valid mutation diff");
    let d2 = move_part_2d("p1".into(), 20.0, 30.0).diff(&mid).into_parts().0;
    semio_framework::io::resolve_ready(assert_mutation_diff_absorb_law(&with_part, d1, d2));
}

fn empty() -> Puzzle5dSnapshot {
    Puzzle5dSnapshot::default()
}

#[test]
fn create_delete_part_inverse_law() {
    use crate::Puzzle5dPart;
    let base = empty();
    let part = Puzzle5dPart { id: "p1".into(), ..Default::default() };
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &create_part(part.clone(), None)));
    let with_part = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part, None).diff(&base).diff(), &base).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &delete_part("p1".into())));
}

#[test]
fn part_field_mutations_inverse_law() {
    use crate::{Puzzle5dGrip, Puzzle5dPart, Puzzle5dPartAnchor, Puzzle5dScale};
    let base = empty();
    let part = Puzzle5dPart { id: "p1".into(), grips: vec![Puzzle5dGrip { id: "g1".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Default::default() }], ..Default::default() };
    let with_part = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part, None).diff(&base).diff(), &base).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &move_part_2d("p1".into(), 5.0, 6.0)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &replace_part_2d_geometry("p1".into(), Some("rectangle".into()), None, Some(4.0), Some(2.0))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &edit_part_2d_text("p1".into(), Some("hi".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &change_part_2d_icon("p1".into(), Some("star".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &change_part_2d_hidden("p1".into(), Some(true))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &change_part_2d_locked("p1".into(), Some(true))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &move_part_3d("p1".into(), [1.0, 2.0, 3.0])));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &rotate_part_3d("p1".into(), Some([0.0, 0.0, 0.0, 1.0]))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &scale_part_3d("p1".into(), Some(Puzzle5dScale::Uniform(2.0)))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &change_part_3d_mesh("p1".into(), Some("mesh://a".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &edit_part_3d_label("p1".into(), Some("Label".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &change_part_kind("p1".into(), Some("core.capsule".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &change_part_anchor("p1".into(), Puzzle5dPartAnchor::Derived)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &add_part_grip("p1".into(), Puzzle5dGrip { id: "g2".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Default::default() }, None)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &remove_part_grip("p1".into(), "g1".into())));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_part, &replace_part_grip("p1".into(), "g1".into(), Puzzle5dGrip { id: "g1".into(), grip_kind: Some("k".into()), grip_2d: Default::default(), grip_3d: Default::default() })));
}

#[test]
fn connect_disconnect_grips_inverse_law_and_cascade() {
    use crate::{Puzzle5dGrip, Puzzle5dPart};
    let base = empty();
    let part_a = Puzzle5dPart { id: "a".into(), grips: vec![Puzzle5dGrip { id: "ga".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Default::default() }], ..Default::default() };
    let part_b = Puzzle5dPart { id: "b".into(), grips: vec![Puzzle5dGrip { id: "gb".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Default::default() }], ..Default::default() };
    let mut projection = base;
    projection = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part_a, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    projection = MutationDiff::<Puzzle5dSnapshot>::apply(create_part(part_b, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&projection, &connect_grips("f1".into(), "a:ga".into(), "b:gb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)));
    let connected = MutationDiff::<Puzzle5dSnapshot>::apply(connect_grips("f1".into(), "a:ga".into(), "b:gb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0).diff(&projection).diff(), &projection).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &disconnect_grips("f1".into())));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(
        &connected,
        &replace_fastener_geometry(crate::standards::v1::subsets::any::schema::mutations::ReplaceFastenerGeometry { id: "f1".into(), new_gap: 1.0, new_shift: 2.0, new_rise: 3.0, new_rotation: 4.0, new_turn: 5.0, new_tilt: 6.0, new_x: 7.0, new_y: 8.0 }),
    ));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &change_fastener_kind("f1".into(), Some("core.link".into()))));
    let deleted = delete_part("a".into());
    let after_delete = MutationDiff::<Puzzle5dSnapshot>::apply(deleted.diff(&connected).diff(), &connected).expect("valid mutation diff");
    assert!(!after_delete.fasteners.iter().any(|fastener| fastener.id == "f1"), "delete-part must sever fasteners touching its grips");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &deleted));
}

#[test]
fn document_scalar_mutations_inverse_law() {
    use crate::{Puzzle5dCompatSpecificity, Puzzle5dKindCatalogs};
    let base = empty();
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &rename_puzzle5d(Some("Nakagin".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &change_domain("mechanical".into())));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &change_description("a scene".into())));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle5dCompatSpecificity::Grip)));
    let connected = MutationDiff::<Puzzle5dSnapshot>::apply(connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle5dCompatSpecificity::Grip).diff(&base).diff(), &base).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &disconnect_kind_compatibility("a".into(), "b".into())));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &replace_kind_catalogs(Some(Puzzle5dKindCatalogs::default()))));
}

#[test]
fn dispatch_registers_semantic_descriptors() {
    register_puzzle5d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in <Puzzle5dMutation as protocol::SemanticMutation<Puzzle5dSnapshot>>::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(<Puzzle5dMutation as protocol::SemanticMutation<Puzzle5dSnapshot>>::kinds().len(), 28);
}
//#endregion 🔖️MutationLaws

//#region 🔖️OutcomeLaws
// 🎫️ 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — see
// `📓️w3-f-block-puzzle-report.md` for the `assert_outcome_policy_matrix` pending-helper note.
use protocol::os_spr::testkit::{assert_fatal_never_applies, assert_missing_target_is_error};

#[test]
fn missing_target_is_error_per_verb_family() {
    let base = empty();
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &delete_part("missing".into()))); // delete
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &remove_part_grip("missing".into(), "g0".into()))); // remove
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &change_part_2d_icon("missing".into(), Some("star".into())))); // change/set/update
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &move_part_2d("missing".into(), 1.0, 1.0))); // move/drag/rotate/scale/resize
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &edit_part_3d_label("missing".into(), Some("x".into())))); // edit/replace
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &disconnect_grips("missing".into())));
    // disconnect/unbind
}

#[test]
fn create_duplicate_id_is_fatal_and_never_applies() {
    use crate::Puzzle5dPart;
    let mut base = empty();
    let part = Puzzle5dPart { id: "p0".into(), ..Default::default() };
    base.parts.push(part.clone());
    let outcome = create_part(part, None).diff(&base);
    semio_framework::io::resolve_ready(assert_fatal_never_applies(&outcome));
    assert_eq!(outcome.worst_level(), Some(dsl::Severity::Fatal));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.duplicate-id"));
}
//#endregion 🔖️OutcomeLaws

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
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog
