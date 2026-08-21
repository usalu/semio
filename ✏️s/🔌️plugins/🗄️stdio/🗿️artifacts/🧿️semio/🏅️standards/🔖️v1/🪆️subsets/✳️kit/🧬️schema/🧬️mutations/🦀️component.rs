//! 🧬️ SemioKitMutation — document mutation dispatch. Vocabulary derived from
//! `📸️snapshot/🦀️component.rs`'s `SemioKitSnapshot` shape: two owned-CHILD collections
//! (`objects`/`models`, `create`/`delete` pairs), one optional owned-CHILD slot (`properties`,
//! `create`/`delete`), one LINK collection (`representations`, `bind`/`unbind` attach/detach plus
//! `change` to re-pin), and two id-keyed value collections (`types`: `add`/`remove`/`rename`;
//! `designs`: `add`/`remove`/`edit` — a design's pieces/connections are one authored unit, `edit`
//! replaces them wholesale per `📓️taxonomy.md`'s "replace an authored content body" rule, same
//! shape `✳️text`'s `edit-run` uses one level down).
//!
//! `object` has no LINK slots (`✳️object`'s own doc comment), so this is the FIRST facet in the
//! ticket to exercise `bind`/`unbind`/`change-link-pin` for real.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Leaves
use super::add_design;
use super::add_type;
use super::bind_representation;
use super::change_representation_pin;
use super::create_model;
use super::create_object;
use super::create_properties;
use super::delete_model;
use super::delete_object;
use super::delete_properties;
use super::edit_design;
use super::remove_design;
use super::remove_type;
use super::rename_type;
use super::unbind_representation;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[mutations(snapshot = SemioKitSnapshot, diff = SemioKitDiff, schema = "s.stdio.semio.kit")]
pub enum SemioKitMutation {
    CreateObject(create_object::mutation::CreateObject),
    DeleteObject(delete_object::mutation::DeleteObject),
    CreateModel(create_model::mutation::CreateModel),
    DeleteModel(delete_model::mutation::DeleteModel),
    CreateProperties(create_properties::mutation::CreateProperties),
    DeleteProperties(delete_properties::mutation::DeleteProperties),
    BindRepresentation(bind_representation::mutation::BindRepresentation),
    UnbindRepresentation(unbind_representation::mutation::UnbindRepresentation),
    ChangeRepresentationPin(change_representation_pin::mutation::ChangeRepresentationPin),
    AddType(add_type::mutation::AddType),
    RemoveType(remove_type::mutation::RemoveType),
    RenameType(rename_type::mutation::RenameType),
    AddDesign(add_design::mutation::AddDesign),
    RemoveDesign(remove_design::mutation::RemoveDesign),
    EditDesign(edit_design::mutation::EditDesign),
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{demo_kit_snapshot, SemioKitDesign, SemioKitPiece};
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
    /// stale pre-operation `base` — same fix `✳️text`'s corrected `round_trip` helper established
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
        // user-meaningful display order (same precedent `✳️graph`'s W2fix established for
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
        let create = SemioKitMutation::CreateObject(create_object::mutation::CreateObject { child_id: "obj-99".into(), target: ref_of("object", "new-obj") });
        let after = round_trip(&base, &create);
        assert!(after.objects.iter().any(|c| c.child_id == "obj-99"));

        let delete = SemioKitMutation::DeleteObject(delete_object::mutation::DeleteObject { child_id: base.objects[0].child_id.clone() });
        let after = round_trip(&base, &delete);
        assert!(after.objects.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_object_of_an_absent_id_has_an_empty_inverse() {
        let base = fixture();
        let delete = SemioKitMutation::DeleteObject(delete_object::mutation::DeleteObject { child_id: "does-not-exist".into() });
        assert!(delete.inverse(&base).is_empty());
        assert_eq!(delete.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base);
    }

    #[semio_framework_async_macros::async_test]
    async fn create_delete_model_round_trips() {
        let base = fixture();
        let create = SemioKitMutation::CreateModel(create_model::mutation::CreateModel { child_id: "model-99".into(), target: ref_of("model", "new-model") });
        let after = round_trip(&base, &create);
        assert!(after.models.iter().any(|c| c.child_id == "model-99"));

        let delete = SemioKitMutation::DeleteModel(delete_model::mutation::DeleteModel { child_id: base.models[0].child_id.clone() });
        let after = round_trip(&base, &delete);
        assert!(after.models.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn create_delete_properties_round_trips() {
        let base = fixture();
        let create = SemioKitMutation::CreateProperties(create_properties::mutation::CreateProperties { child_id: "props-99".into(), target: ref_of("value", "new-props") });
        let after = round_trip(&base, &create);
        assert_eq!(after.properties.as_ref().unwrap().child_id, "props-99");

        let delete = SemioKitMutation::DeleteProperties(delete_properties::mutation::DeleteProperties {});
        let after = round_trip(&base, &delete);
        assert!(after.properties.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn bind_unbind_representation_round_trips() {
        let base = fixture();
        let bind = SemioKitMutation::BindRepresentation(bind_representation::mutation::BindRepresentation { target: ref_of("mesh", "extra-repr"), pin: store::LinkPin::Head, role: "chair".into() });
        let after = round_trip(&base, &bind);
        assert_eq!(after.representations.len(), base.representations.len() + 1);

        let unbind = SemioKitMutation::UnbindRepresentation(unbind_representation::mutation::UnbindRepresentation { index: 0 });
        let after = round_trip(&base, &unbind);
        assert!(after.representations.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn unbind_representation_of_an_out_of_range_index_has_an_empty_inverse() {
        let base = fixture();
        let unbind = SemioKitMutation::UnbindRepresentation(unbind_representation::mutation::UnbindRepresentation { index: 99 });
        assert!(unbind.inverse(&base).is_empty());
        assert_eq!(unbind.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base);
    }

    #[semio_framework_async_macros::async_test]
    async fn change_representation_pin_round_trips() {
        let base = fixture();
        let change = SemioKitMutation::ChangeRepresentationPin(change_representation_pin::mutation::ChangeRepresentationPin { index: 0, pin: store::LinkPin::Checkpoint { id: "cp-1".into() } });
        let after = round_trip(&base, &change);
        assert_eq!(after.representations[0].pin, store::LinkPin::Checkpoint { id: "cp-1".into() });
    }

    #[semio_framework_async_macros::async_test]
    async fn add_remove_rename_type_round_trips() {
        let base = fixture();
        let add = SemioKitMutation::AddType(add_type::mutation::AddType { id: "table".into(), name: "Table".into(), category: "furniture".into() });
        let after = round_trip(&base, &add);
        assert!(after.types.iter().any(|t| t.id == "table"));

        let rename = SemioKitMutation::RenameType(rename_type::mutation::RenameType { id: "chair".into(), new_name: "Armchair".into() });
        let after = round_trip(&base, &rename);
        assert_eq!(after.types.iter().find(|t| t.id == "chair").unwrap().name, "Armchair");

        let remove = SemioKitMutation::RemoveType(remove_type::mutation::RemoveType { id: "chair".into() });
        let after = round_trip(&base, &remove);
        assert!(after.types.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn add_remove_edit_design_round_trips() {
        let base = fixture();
        let add = SemioKitMutation::AddDesign(add_design::mutation::AddDesign { id: "office".into(), name: "Office".into() });
        let after = round_trip(&base, &add);
        assert!(after.designs.iter().any(|d| d.id == "office"));

        let new_pieces = vec![SemioKitPiece { id: "p9".into(), type_id: "chair".into(), transform: crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioTransform::identity() }];
        let edit = SemioKitMutation::EditDesign(edit_design::mutation::EditDesign { id: "living-room".into(), pieces: new_pieces.clone(), connections: vec![] });
        let after = round_trip(&base, &edit);
        assert_eq!(after.designs.iter().find(|d| d.id == "living-room").unwrap().pieces, new_pieces);

        let remove = SemioKitMutation::RemoveDesign(remove_design::mutation::RemoveDesign { id: "living-room".into() });
        let after = round_trip(&base, &remove);
        assert!(after.designs.is_empty());
        let _ = SemioKitDesign::default();
    }

    #[semio_framework_async_macros::async_test]
    async fn semantic_kinds_cover_every_variant() {
        assert_eq!(SemioKitMutation::kinds().len(), 15);
        let mutation = SemioKitMutation::UnbindRepresentation(unbind_representation::mutation::UnbindRepresentation { index: 1 });
        assert_eq!(mutation.semantics().kind, "unbind-representation");
        assert_eq!(mutation.semantics().record, "UnboundRepresentation");
    }
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
/// 🧪️ Handcrafted mutation fixtures (contract D1, ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`)
/// — one case per triad leaf, self-wired here rather than in `📦️glue.rs` so this subset owns its
/// own test surface. `#[path = "."]` re-roots the nested `#[path]`s at THIS file's directory (the
/// `🧬️mutations` root) instead of the implicit `🦀️component/` child directory.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "🏗️create-object/🧪️tests/attaches-a-second-object-child/🦀️component.rs"]
    mod tests_create_object_attaches_a_second_object_child;
    #[path = "🪓delete-object/🧪️tests/detaches-the-only-object-child-and-keeps-the-model-child/🦀️component.rs"]
    mod tests_delete_object_detaches_the_only_object_child_and_keeps_the_model_child;
    #[path = "🏛️create-model/🧪️tests/attaches-a-second-model-child/🦀️component.rs"]
    mod tests_create_model_attaches_a_second_model_child;
    #[path = "💣delete-model/🧪️tests/detaches-the-only-model-child-and-keeps-the-object-child/🦀️component.rs"]
    mod tests_delete_model_detaches_the_only_model_child_and_keeps_the_object_child;
    #[path = "🏷️create-properties/🧪️tests/attaches-a-properties-child-to-a-kit-that-has-none/🦀️component.rs"]
    mod tests_create_properties_attaches_a_properties_child_to_a_kit_that_has_none;
    #[path = "🚫delete-properties/🧪️tests/detaches-the-properties-child-and-leaves-every-other-collection-alone/🦀️component.rs"]
    mod tests_delete_properties_detaches_the_properties_child_and_leaves_every_other_collection_alone;
    #[path = "🔗bind-representation/🧪️tests/binds-a-second-representation-to-an-existing-type/🦀️component.rs"]
    mod tests_bind_representation_binds_a_second_representation_to_an_existing_type;
    #[path = "✂️unbind-representation/🧪️tests/unbinds-the-leading-representation-and-keeps-the-trailing-one/🦀️component.rs"]
    mod tests_unbind_representation_unbinds_the_leading_representation_and_keeps_the_trailing_one;
    #[path = "📌change-representation-pin/🧪️tests/repins-the-representation-from-head-to-a-checkpoint/🦀️component.rs"]
    mod tests_change_representation_pin_repins_the_representation_from_head_to_a_checkpoint;
    #[path = "➕add-type/🧪️tests/appends-a-slab-type-to-the-catalogue/🦀️component.rs"]
    mod tests_add_type_appends_a_slab_type_to_the_catalogue;
    #[path = "➖remove-type/🧪️tests/removes-the-column-type-and-keeps-the-beam-type/🦀️component.rs"]
    mod tests_remove_type_removes_the_column_type_and_keeps_the_beam_type;
    #[path = "✏️rename-type/🧪️tests/renames-the-beam-type-without-recategorising-it/🦀️component.rs"]
    mod tests_rename_type_renames_the_beam_type_without_recategorising_it;
    #[path = "🆕add-design/🧪️tests/adds-an-empty-roof-design/🦀️component.rs"]
    mod tests_add_design_adds_an_empty_roof_design;
    #[path = "🗑️remove-design/🧪️tests/removes-the-only-design-together-with-its-pieces/🦀️component.rs"]
    mod tests_remove_design_removes_the_only_design_together_with_its_pieces;
    #[path = "🖊️edit-design/🧪️tests/replaces-the-designs-pieces-and-connections-in-one-step/🦀️component.rs"]
    mod tests_edit_design_replaces_the_designs_pieces_and_connections_in_one_step;
}
//#endregion 🧪️FixtureTests
