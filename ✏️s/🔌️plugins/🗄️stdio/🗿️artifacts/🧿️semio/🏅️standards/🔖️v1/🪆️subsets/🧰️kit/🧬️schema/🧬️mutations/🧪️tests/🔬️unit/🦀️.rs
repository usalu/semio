
use super::*;
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitDesign, SemioKitPiece, demo_kit_snapshot};
use protocol::{Mutation, MutationDiff, SemanticMutation};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> SemioKitSnapshot {
    demo_kit_snapshot()
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn ref_of(subset: &str, id: &str) -> store::os_io::ArtifactRef {
    store::os_io::ArtifactRef { artifact_id: id.into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() } }
}

/// 🔧️ Each inverse's diff must be computed against the CURRENT (`restored`) state, not the
/// stale pre-operation `base` — same fix `🔤️text`'s corrected `round_trip` helper established
/// (📌️important.md Trap #1).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn round_trip(base: &SemioKitSnapshot, operation: &SemioKitMutation) -> SemioKitSnapshot {
    let forward = operation.diff(base).diff().apply(base).expect("apply must succeed for a well-formed fixture");
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(&restored).diff().apply(&restored).expect("apply must succeed for a well-formed fixture");
    }
    // 🔧️ `objects`/`models`/`representations` are documented as id/role-keyed SETS with no
    // user-meaningful display order (same precedent `🕸️graph`'s W2fix established for
    // `nodes`/`edges` — a cascading create-on-undo always APPENDS, which can legitimately land
    // a restored entry at a different POSITION than it started at without the SET itself being
    // wrong). Compare order-insensitively for those three fields; everything else compares
    // exactly.
    let mut restored_objects = restored.objects.clone();
    let mut base_objects = base.objects.clone();
    restored_objects.sort_by(|a, b| a.child_id.cmp(&b.child_id));
    base_objects.sort_by(|a, b| a.child_id.cmp(&b.child_id));
    assert_eq!(restored_objects, base_objects, "inverse must exactly restore `objects` (order-insensitive)");
    let mut restored_models = restored.models.clone();
    let mut base_models = base.models.clone();
    restored_models.sort_by(|a, b| a.child_id.cmp(&b.child_id));
    base_models.sort_by(|a, b| a.child_id.cmp(&b.child_id));
    assert_eq!(restored_models, base_models, "inverse must exactly restore `models` (order-insensitive)");
    let mut restored_reprs = restored.representations.clone();
    let mut base_reprs = base.representations.clone();
    restored_reprs.sort_by(|a, b| a.role.cmp(&b.role));
    base_reprs.sort_by(|a, b| a.role.cmp(&b.role));
    assert_eq!(restored_reprs, base_reprs, "inverse must exactly restore `representations` (order-insensitive)");
    assert_eq!(restored.types, base.types);
    assert_eq!(restored.designs, base.designs);
    assert_eq!(restored.properties, base.properties);
    forward
}

#[semio_framework_async_macros::async_test]
async fn create_delete_object_round_trips() {
    let base = fixture();
    let create = SemioKitMutation::CreateObject(create_object::CreateObject { child_id: "obj-99".into(), target: ref_of("object", "new-obj") });
    let after = round_trip(&base, &create);
    assert!(after.objects.iter().any(|c| c.child_id == "obj-99"));

    let delete = SemioKitMutation::DeleteObject(delete_object::DeleteObject { child_id: base.objects[0].child_id.clone() });
    let after = round_trip(&base, &delete);
    assert!(after.objects.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn delete_object_of_an_absent_id_has_an_empty_inverse() {
    let base = fixture();
    let delete = SemioKitMutation::DeleteObject(delete_object::DeleteObject { child_id: "does-not-exist".into() });
    assert!(delete.inverse(&base).is_empty());
    assert_eq!(delete.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base);
}

#[semio_framework_async_macros::async_test]
async fn create_delete_model_round_trips() {
    let base = fixture();
    let create = SemioKitMutation::CreateModel(create_model::CreateModel { child_id: "model-99".into(), target: ref_of("model", "new-model") });
    let after = round_trip(&base, &create);
    assert!(after.models.iter().any(|c| c.child_id == "model-99"));

    let delete = SemioKitMutation::DeleteModel(delete_model::DeleteModel { child_id: base.models[0].child_id.clone() });
    let after = round_trip(&base, &delete);
    assert!(after.models.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn create_delete_properties_round_trips() {
    let base = fixture();
    let create = SemioKitMutation::CreateProperties(create_properties::CreateProperties { child_id: "props-99".into(), target: ref_of("value", "new-props") });
    let after = round_trip(&base, &create);
    assert_eq!(after.properties.as_ref().unwrap().child_id, "props-99");

    let delete = SemioKitMutation::DeleteProperties(delete_properties::DeleteProperties {});
    let after = round_trip(&base, &delete);
    assert!(after.properties.is_none());
}

#[semio_framework_async_macros::async_test]
async fn bind_unbind_representation_round_trips() {
    let base = fixture();
    let bind = SemioKitMutation::BindRepresentation(bind_representation::BindRepresentation { target: ref_of("mesh", "extra-repr"), pin: store::LinkPin::Head, role: "chair".into() });
    let after = round_trip(&base, &bind);
    assert_eq!(after.representations.len(), base.representations.len() + 1);

    let unbind = SemioKitMutation::UnbindRepresentation(unbind_representation::UnbindRepresentation { index: 0 });
    let after = round_trip(&base, &unbind);
    assert!(after.representations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn unbind_representation_of_an_out_of_range_index_has_an_empty_inverse() {
    let base = fixture();
    let unbind = SemioKitMutation::UnbindRepresentation(unbind_representation::UnbindRepresentation { index: 99 });
    assert!(unbind.inverse(&base).is_empty());
    assert_eq!(unbind.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base);
}

#[semio_framework_async_macros::async_test]
async fn change_representation_pin_round_trips() {
    let base = fixture();
    let change = SemioKitMutation::ChangeRepresentationPin(change_representation_pin::ChangeRepresentationPin { index: 0, pin: store::LinkPin::Checkpoint { id: "cp-1".into() } });
    let after = round_trip(&base, &change);
    assert_eq!(after.representations[0].pin, store::LinkPin::Checkpoint { id: "cp-1".into() });
}

#[semio_framework_async_macros::async_test]
async fn add_remove_rename_type_round_trips() {
    let base = fixture();
    let add = SemioKitMutation::AddType(add_type::AddType { id: "table".into(), name: "Table".into(), category: "furniture".into() });
    let after = round_trip(&base, &add);
    assert!(after.types.iter().any(|t| t.id == "table"));

    let rename = SemioKitMutation::RenameType(rename_type::RenameType { id: "chair".into(), new_name: "Armchair".into() });
    let after = round_trip(&base, &rename);
    assert_eq!(after.types.iter().find(|t| t.id == "chair").unwrap().name, "Armchair");

    let remove = SemioKitMutation::RemoveType(remove_type::RemoveType { id: "chair".into() });
    let after = round_trip(&base, &remove);
    assert!(after.types.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn add_remove_edit_design_round_trips() {
    let base = fixture();
    let add = SemioKitMutation::AddDesign(add_design::AddDesign { id: "office".into(), name: "Office".into() });
    let after = round_trip(&base, &add);
    assert!(after.designs.iter().any(|d| d.id == "office"));

    let new_pieces = vec![SemioKitPiece { id: "p9".into(), type_id: "chair".into(), transform: crate::standards::v1::subsets::base::schema::geometry::SemioTransform::identity() }];
    let edit = SemioKitMutation::EditDesign(edit_design::EditDesign { id: "living-room".into(), pieces: new_pieces.clone(), connections: vec![] });
    let after = round_trip(&base, &edit);
    assert_eq!(after.designs.iter().find(|d| d.id == "living-room").unwrap().pieces, new_pieces);

    let remove = SemioKitMutation::RemoveDesign(remove_design::RemoveDesign { id: "living-room".into() });
    let after = round_trip(&base, &remove);
    assert!(after.designs.is_empty());
    let _ = SemioKitDesign::default();
}

#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(SemioKitMutation::kinds().len(), 15);
    let mutation = SemioKitMutation::UnbindRepresentation(unbind_representation::UnbindRepresentation { index: 1 });
    assert_eq!(mutation.semantics().kind, "unbind-representation");
    assert_eq!(mutation.semantics().record, "UnboundRepresentation");
}

/// 🏷️ `KINDS` (this facet's own const, consumed by `mutate-semio-kit`'s adapter) must name
/// every declared variant, in the exact order and spelling `#[derive(dsl::Mutations)]` assigns —
/// the framework never parses Rust, so this is what keeps the catalog honest.
#[semio_framework_async_macros::async_test]
async fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = SemioKitMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
