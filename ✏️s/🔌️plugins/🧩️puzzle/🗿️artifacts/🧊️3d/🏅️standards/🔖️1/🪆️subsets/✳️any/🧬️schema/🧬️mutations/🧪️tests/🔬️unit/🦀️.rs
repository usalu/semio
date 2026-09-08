
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
fn move_object_diff_absorb_law() {
    use crate::Puzzle3dObject;
    let base = empty();
    let object = Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    let with_object = MutationDiff::<Puzzle3dSnapshot>::apply(create_object(object, None).diff(&base).diff(), &base).expect("valid mutation diff");
    let d1 = move_object("o1".into(), [10.0, 10.0, 10.0]).diff(&with_object).into_parts().0;
    let mid = MutationDiff::<Puzzle3dSnapshot>::apply(&d1, &with_object).expect("valid mutation diff");
    let d2 = move_object("o1".into(), [20.0, 30.0, 40.0]).diff(&mid).into_parts().0;
    semio_framework::io::resolve_ready(assert_mutation_diff_absorb_law(&with_object, d1, d2));
}

fn empty() -> Puzzle3dSnapshot {
    Puzzle3dSnapshot::default()
}

#[test]
fn create_delete_object_inverse_law() {
    use crate::Puzzle3dObject;
    let base = empty();
    let object = Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &create_object(object.clone(), None)));
    let with_object = MutationDiff::<Puzzle3dSnapshot>::apply(create_object(object, None).diff(&base).diff(), &base).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &delete_object("o1".into())));
}

#[test]
fn object_field_mutations_inverse_law() {
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
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &move_object("o1".into(), [1.0, 2.0, 3.0])));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &rotate_object("o1".into(), Some([0.0, 0.0, 0.0, 1.0]))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &scale_object("o1".into(), Some(Puzzle3dScale::Uniform(2.0)))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &change_object_mesh("o1".into(), Some("mesh://a".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &edit_object_label("o1".into(), Some("Label".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &change_object_kind("o1".into(), Some("core.capsule".into()))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &change_object_anchor("o1".into(), Puzzle3dObjectAnchor::Derived)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &change_object_hidden("o1".into(), true)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &change_object_locked("o1".into(), true)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(
        &with_object,
        &add_object_vortex("o1".into(), Puzzle3dVortex { id: "v2".into(), vortex_kind: None, label: None, position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false }, None),
    ));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_object, &remove_object_vortex("o1".into(), "v1".into())));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(
        &with_object,
        &replace_object_vortex("o1".into(), "v1".into(), Puzzle3dVortex { id: "v1".into(), vortex_kind: Some("k".into()), label: None, position: [1.0, 1.0, 1.0], direction: None, radius: None, hidden: false, locked: false }),
    ));
}

#[test]
fn connect_disconnect_vortices_inverse_law_and_cascade() {
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
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&projection, &connect_vortices("t1".into(), "a:va".into(), "b:vb".into(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)));
    let connected = MutationDiff::<Puzzle3dSnapshot>::apply(connect_vortices("t1".into(), "a:va".into(), "b:vb".into(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0).diff(&projection).diff(), &projection).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &disconnect_vortices("t1".into())));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(
        &connected,
        &replace_attraction_geometry(crate::standards::v1::subsets::any::schema::mutations::ReplaceAttractionGeometry { id: "t1".into(), new_gap: 1.0, new_shift: 2.0, new_rise: 3.0, new_rotation: 4.0, new_turn: 5.0, new_tilt: 6.0, new_x: 7.0, new_y: 8.0 }),
    ));
    let deleted = delete_object("a".into());
    let after_delete = MutationDiff::<Puzzle3dSnapshot>::apply(deleted.diff(&connected).diff(), &connected).expect("valid mutation diff");
    assert!(!after_delete.attractions.iter().any(|attraction| attraction.id == "t1"), "delete-object must sever attractions touching its vortices");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &deleted));
}

#[test]
fn target_volume_and_reference_inverse_law() {
    use crate::{Puzzle3dReference, Puzzle3dReferenceSource, Puzzle3dTargetVolume};
    let base = empty();
    let volume = Puzzle3dTargetVolume { id: "tv1".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, hidden: false, locked: false };
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &create_target_volume(volume.clone(), None)));
    let with_volume = MutationDiff::<Puzzle3dSnapshot>::apply(create_target_volume(volume, None).diff(&base).diff(), &base).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_volume, &move_target_volume("tv1".into(), [1.0, 2.0, 3.0])));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_volume, &rotate_target_volume("tv1".into(), Some([0.0, 0.0, 0.0, 1.0]))));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_volume, &scale_target_volume("tv1".into(), None)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_volume, &change_target_volume_hidden("tv1".into(), true)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_volume, &change_target_volume_locked("tv1".into(), true)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_volume, &delete_target_volume("tv1".into())));

    let reference = Puzzle3dReference { id: "r1".into(), source: Puzzle3dReferenceSource::default(), origin: [0.0, 0.0, 0.0], width_world: 1.0, locked: false, hidden: false };
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &create_reference(reference.clone(), None)));
    let with_reference = MutationDiff::<Puzzle3dSnapshot>::apply(create_reference(reference, None).diff(&base).diff(), &base).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_reference, &move_reference("r1".into(), [1.0, 2.0, 3.0])));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_reference, &resize_reference("r1".into(), 4.0)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_reference, &replace_reference_source("r1".into(), Puzzle3dReferenceSource { url: "/x.png".into(), media_kind: Some("image".into()) })));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_reference, &change_reference_hidden("r1".into(), true)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_reference, &change_reference_locked("r1".into(), true)));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_reference, &delete_reference("r1".into())));
}

#[test]
fn document_scalar_mutations_inverse_law() {
    use crate::{Puzzle3dCompatSpecificity, Puzzle3dKindCatalogs};
    let base = empty();
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &change_domain("mechanical".into())));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle3dCompatSpecificity::Vortex)));
    let connected = MutationDiff::<Puzzle3dSnapshot>::apply(connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle3dCompatSpecificity::Vortex).diff(&base).diff(), &base).expect("valid mutation diff");
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &disconnect_kind_compatibility("a".into(), "b".into())));
    semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &replace_kind_catalogs(Some(Puzzle3dKindCatalogs::default()))));
}

#[test]
fn dispatch_registers_semantic_descriptors() {
    register_puzzle3d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in <Puzzle3dMutation as protocol::SemanticMutation<Puzzle3dSnapshot>>::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(<Puzzle3dMutation as protocol::SemanticMutation<Puzzle3dSnapshot>>::kinds().len(), 35);
}
//#endregion 🔖️MutationLaws

//#region 🔖️OutcomeLaws
// 🎫️ 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — see
// `📓️w3-f-block-puzzle-report.md` for the `assert_outcome_policy_matrix` pending-helper note.
use protocol::os_spr::testkit::{assert_fatal_never_applies, assert_missing_target_is_error};

#[test]
fn missing_target_is_error_per_verb_family() {
    let base = empty();
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &delete_object("missing".into()))); // delete
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &remove_object_vortex("missing".into(), "v0".into()))); // remove
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &change_object_hidden("missing".into(), true))); // change/set/update
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &move_object("missing".into(), [1.0, 1.0, 1.0]))); // move/drag/rotate/scale/resize
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &edit_object_label("missing".into(), Some("x".into())))); // edit/replace
    semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &disconnect_vortices("missing".into())));
    // disconnect/unbind
}

#[test]
fn create_duplicate_id_is_fatal_and_never_applies() {
    use crate::Puzzle3dObject;
    let mut base = empty();
    let object = Puzzle3dObject { id: "o0".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    base.objects.push(object.clone());
    let outcome = create_object(object, None).diff(&base);
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
    let descriptors = <Puzzle3dMutation as protocol::SemanticMutation<Puzzle3dSnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared Puzzle3dMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog
