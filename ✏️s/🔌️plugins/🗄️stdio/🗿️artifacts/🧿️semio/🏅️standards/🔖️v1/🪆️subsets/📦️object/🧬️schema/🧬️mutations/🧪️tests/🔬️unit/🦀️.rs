
use super::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
use crate::standards::v1::subsets::object::schema::snapshot::demo_object_snapshot;
use protocol::{Mutation, MutationDiff, SemanticMutation};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> SemioObjectSnapshot {
    demo_object_snapshot()
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn ref_of(subset: &str, id: &str) -> store::os_io::ArtifactRef {
    store::os_io::ArtifactRef { artifact_id: id.into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() } }
}

/// 🔧️ Each inverse's diff must be computed against the CURRENT (`restored`) state, not the
/// stale pre-operation `base` — same fix `🔤️text`'s corrected `round_trip` helper established
/// (📌️important.md Trap #1: the `din4108`-derived helper got this wrong).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn round_trip(base: &SemioObjectSnapshot, operation: &SemioObjectMutation) -> SemioObjectSnapshot {
    let forward = operation.diff(base).diff().apply(base).expect("apply must succeed for a well-formed fixture");
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(&restored).diff().apply(&restored).expect("apply must succeed for a well-formed fixture");
    }
    assert_eq!(&restored, base, "inverse must exactly restore the pre-operation fixture");
    forward
}

#[semio_framework_async_macros::async_test]
async fn move_rotate_scale_round_trip() {
    let base = fixture();

    let mv = SemioObjectMutation::MoveObject(move_object::MoveObject { translation: SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 } });
    let after = round_trip(&base, &mv);
    assert_eq!(after.transform.translation, SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 });
    assert_eq!(after.transform.rotation, base.transform.rotation);

    let rot = SemioObjectMutation::RotateObject(rotate_object::RotateObject { rotation: SemioQuaternion { x: 0.0, y: 0.0, z: 1.0, w: 0.0 } });
    let after = round_trip(&base, &rot);
    assert_eq!(after.transform.rotation, SemioQuaternion { x: 0.0, y: 0.0, z: 1.0, w: 0.0 });

    let sc = SemioObjectMutation::ScaleObject(scale_object::ScaleObject { scale: SemioPoint3 { x: 2.0, y: 2.0, z: 2.0 } });
    let after = round_trip(&base, &sc);
    assert_eq!(after.transform.scale, SemioPoint3 { x: 2.0, y: 2.0, z: 2.0 });
}

#[semio_framework_async_macros::async_test]
async fn create_delete_brep_round_trips() {
    let base = fixture();
    let target = ref_of("brep", "new-brep");

    let create = SemioObjectMutation::CreateBrep(create_brep::CreateBrep { child_id: "brep-99".into(), target: target.clone() });
    let after = round_trip(&base, &create);
    assert_eq!(after.brep.as_ref().unwrap().child_id, "brep-99");
    assert_eq!(after.brep.as_ref().unwrap().target, target);

    let delete = SemioObjectMutation::DeleteBrep(delete_brep::DeleteBrep {});
    let after = round_trip(&base, &delete);
    assert!(after.brep.is_none());
}

#[semio_framework_async_macros::async_test]
async fn delete_brep_of_an_absent_slot_has_an_empty_inverse() {
    let mut base = fixture();
    base.brep = None;
    let delete = SemioObjectMutation::DeleteBrep(delete_brep::DeleteBrep {});
    assert!(delete.inverse(&base).is_empty(), "deleting an already-absent slot has nothing to undo");
    assert_eq!(delete.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base, "deleting an absent slot is a no-op");
}

#[semio_framework_async_macros::async_test]
async fn create_brep_overwriting_an_existing_handle_restores_the_prior_one_on_undo() {
    let base = fixture();
    assert!(base.brep.is_some(), "fixture must start with a brep handle to exercise overwrite");
    let create = SemioObjectMutation::CreateBrep(create_brep::CreateBrep { child_id: "brand-new".into(), target: ref_of("brep", "brand-new-target") });
    let after = round_trip(&base, &create);
    assert_eq!(after.brep.as_ref().unwrap().child_id, "brand-new");
}

#[semio_framework_async_macros::async_test]
async fn create_delete_mesh_round_trips() {
    let base = fixture();
    let create = SemioObjectMutation::CreateMesh(create_mesh::CreateMesh { child_id: "mesh-99".into(), target: ref_of("mesh", "new-mesh") });
    let after = round_trip(&base, &create);
    assert_eq!(after.mesh.as_ref().unwrap().child_id, "mesh-99");

    let delete = SemioObjectMutation::DeleteMesh(delete_mesh::DeleteMesh {});
    let after = round_trip(&base, &delete);
    assert!(after.mesh.is_none());
}

#[semio_framework_async_macros::async_test]
async fn create_delete_properties_round_trips() {
    let base = fixture();
    let create = SemioObjectMutation::CreateProperties(create_properties::CreateProperties { child_id: "props-99".into(), target: ref_of("value", "new-props") });
    let after = round_trip(&base, &create);
    assert_eq!(after.properties.as_ref().unwrap().child_id, "props-99");

    let delete = SemioObjectMutation::DeleteProperties(delete_properties::DeleteProperties {});
    let after = round_trip(&base, &delete);
    assert!(after.properties.is_none());
}

#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(SemioObjectMutation::kinds().len(), 9);
    let mutation = SemioObjectMutation::DeleteMesh(delete_mesh::DeleteMesh {});
    assert_eq!(mutation.semantics().kind, "delete-mesh");
    assert_eq!(mutation.semantics().record, "DeletedMesh");
}

/// 🏷️ `KINDS` (this facet's own const, consumed by `mutate-semio-object`'s adapter) must name
/// every declared variant, in the exact order and spelling `#[derive(dsl::Mutations)]` assigns —
/// the framework never parses Rust, so this is what keeps the catalog honest.
#[semio_framework_async_macros::async_test]
async fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = SemioObjectMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
