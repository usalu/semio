//! 🧬️ Puzzle 3d artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<Puzzle3dSnapshot>` and `impl protocol::SemanticMutation<Puzzle3dSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here. `dsl::DslEnum` supplies
//! `DslVariants`, consumed by `OpText`/`OpBinary` in the sibling `📝️text`/`💾️binary` modules.
//!
//! The native editor owns its immutable play root; semantic mutation authority remains the
//! canonical typed snapshot and its typed diff algebra.

use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::Puzzle3dSnapshot;
use protocol::{DiffAlgebra, Mutation};

//#region 🔖️Mutations
/// 🧮️ Semantic puzzle-3d document mutation vocabulary: id-keyed object/target-volume/reference
/// create-delete plus per-field edits, vortex membership, a vortex-to-vortex attraction connect/
/// disconnect relationship, and document-level edits (domain change, kind-compatibility connect/
/// disconnect, kind-catalog replace), and the three parametric selection transforms (`drag-`,
/// `rotate-`, `scale-selection`) that record a gesture's own inputs. There is deliberately no camera
/// mutation: camera pose is session-only app runtime state (`ActionKind::View`), never a document
/// operation. There is deliberately no whole-document mutation: import/reset/example-load goes
/// through `store::ArtifactStore::reset` (non-history), never through this enum.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = Puzzle3dSnapshot, diff = Puzzle3dDiff, schema = "puzzle.puzzle3d")]
pub enum Puzzle3dMutation {
    CreateObject(CreateObject),
    DeleteObject(DeleteObject),
    MoveObject(MoveObject),
    RotateObject(RotateObject),
    ScaleObject(ScaleObject),
    ChangeObjectMesh(ChangeObjectMesh),
    EditObjectLabel(EditObjectLabel),
    ChangeObjectKind(ChangeObjectKind),
    ChangeObjectAnchor(ChangeObjectAnchor),
    ChangeObjectHidden(ChangeObjectHidden),
    ChangeObjectLocked(ChangeObjectLocked),
    AddObjectVortex(AddObjectVortex),
    RemoveObjectVortex(RemoveObjectVortex),
    ReplaceObjectVortex(ReplaceObjectVortex),
    ConnectVortices(ConnectVortices),
    DisconnectVortices(DisconnectVortices),
    ReplaceAttractionGeometry(ReplaceAttractionGeometry),
    CreateTargetVolume(CreateTargetVolume),
    DeleteTargetVolume(DeleteTargetVolume),
    MoveTargetVolume(MoveTargetVolume),
    RotateTargetVolume(RotateTargetVolume),
    ScaleTargetVolume(ScaleTargetVolume),
    ChangeTargetVolumeHidden(ChangeTargetVolumeHidden),
    ChangeTargetVolumeLocked(ChangeTargetVolumeLocked),
    CreateReference(CreateReference),
    DeleteReference(DeleteReference),
    MoveReference(MoveReference),
    ResizeReference(ResizeReference),
    ReplaceReferenceSource(ReplaceReferenceSource),
    ChangeReferenceHidden(ChangeReferenceHidden),
    ChangeReferenceLocked(ChangeReferenceLocked),
    ChangeDomain(ChangeDomain),
    ConnectKindCompatibility(ConnectKindCompatibility),
    DisconnectKindCompatibility(DisconnectKindCompatibility),
    ReplaceKindCatalogs(ReplaceKindCatalogs),
    DragSelection(DragSelection),
    RotateSelection(RotateSelection),
    ScaleSelection(ScaleSelection),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Puzzle3dMutation`] variant, in declaration order — the exact
/// vocabulary the `puzzle-3d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🧊️mutate-puzzle-3d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-object",
    "delete-object",
    "move-object",
    "rotate-object",
    "scale-object",
    "change-object-mesh",
    "edit-object-label",
    "change-object-kind",
    "change-object-anchor",
    "change-object-hidden",
    "change-object-locked",
    "add-object-vortex",
    "remove-object-vortex",
    "replace-object-vortex",
    "connect-vortices",
    "disconnect-vortices",
    "replace-attraction-geometry",
    "create-target-volume",
    "delete-target-volume",
    "move-target-volume",
    "rotate-target-volume",
    "scale-target-volume",
    "change-target-volume-hidden",
    "change-target-volume-locked",
    "create-reference",
    "delete-reference",
    "move-reference",
    "resize-reference",
    "replace-reference-source",
    "change-reference-hidden",
    "change-reference-locked",
    "change-domain",
    "connect-kind-compatibility",
    "disconnect-kind-compatibility",
    "replace-kind-catalogs",
    "drag-selection",
    "rotate-selection",
    "scale-selection",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

pub use super::add_object_vortex::mutation::{add_object_vortex, AddObjectVortex};
pub use super::change_domain::mutation::{change_domain, ChangeDomain};
pub use super::change_object_anchor::mutation::{change_object_anchor, ChangeObjectAnchor};
pub use super::change_object_hidden::mutation::{change_object_hidden, ChangeObjectHidden};
pub use super::change_object_kind::mutation::{change_object_kind, ChangeObjectKind};
pub use super::change_object_locked::mutation::{change_object_locked, ChangeObjectLocked};
pub use super::change_object_mesh::mutation::{change_object_mesh, ChangeObjectMesh};
pub use super::change_reference_hidden::mutation::{change_reference_hidden, ChangeReferenceHidden};
pub use super::change_reference_locked::mutation::{change_reference_locked, ChangeReferenceLocked};
pub use super::change_target_volume_hidden::mutation::{change_target_volume_hidden, ChangeTargetVolumeHidden};
pub use super::change_target_volume_locked::mutation::{change_target_volume_locked, ChangeTargetVolumeLocked};
pub use super::connect_kind_compatibility::mutation::{connect_kind_compatibility, ConnectKindCompatibility};
pub use super::connect_vortices::mutation::{connect_vortices, ConnectVortices};
pub use super::create_object::mutation::{create_object, CreateObject};
pub use super::create_reference::mutation::{create_reference, CreateReference};
pub use super::create_target_volume::mutation::{create_target_volume, CreateTargetVolume};
pub use super::delete_object::mutation::{delete_object, DeleteObject};
pub use super::delete_reference::mutation::{delete_reference, DeleteReference};
pub use super::delete_target_volume::mutation::{delete_target_volume, DeleteTargetVolume};
pub use super::disconnect_kind_compatibility::mutation::{disconnect_kind_compatibility, DisconnectKindCompatibility};
pub use super::disconnect_vortices::mutation::{disconnect_vortices, DisconnectVortices};
pub use super::drag_selection::mutation::{drag_selection, DragSelection};
pub use super::edit_object_label::mutation::{edit_object_label, EditObjectLabel};
pub use super::move_object::mutation::{move_object, MoveObject};
pub use super::move_reference::mutation::{move_reference, MoveReference};
pub use super::move_target_volume::mutation::{move_target_volume, MoveTargetVolume};
pub use super::remove_object_vortex::mutation::{remove_object_vortex, RemoveObjectVortex};
pub use super::replace_attraction_geometry::mutation::{replace_attraction_geometry, ReplaceAttractionGeometry};
pub use super::replace_kind_catalogs::mutation::{replace_kind_catalogs, ReplaceKindCatalogs};
pub use super::replace_object_vortex::mutation::{replace_object_vortex, ReplaceObjectVortex};
pub use super::replace_reference_source::mutation::{replace_reference_source, ReplaceReferenceSource};
pub use super::resize_reference::mutation::{resize_reference, ResizeReference};
pub use super::rotate_object::mutation::{rotate_object, RotateObject};
pub use super::rotate_selection::mutation::{rotate_selection, RotateSelection};
pub use super::rotate_target_volume::mutation::{rotate_target_volume, RotateTargetVolume};
pub use super::scale_object::mutation::{scale_object, ScaleObject};
pub use super::scale_selection::mutation::{scale_selection, ScaleSelection};
pub use super::scale_target_volume::mutation::{scale_target_volume, ScaleTargetVolume};

//#region 🔖️SelectionTransform

//#endregion 🔖️SelectionTransform

//#region 🔖️AttractionPose

//#endregion 🔖️AttractionPose

pub fn inverse_puzzle3d_mutation(projection: &Puzzle3dSnapshot, mutation: &Puzzle3dMutation) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🧪️selection-time-travel/🦀️.rs"]
mod selection_time_travel;
//#endregion 🧪️Tests

/// ▶️ Applies one mutation to a projection through the central applier (`vcs::apply_mutation`): the step of the editor tools' charts and of the mutation tests.
pub fn apply_puzzle3d_mutation(projection: &mut Puzzle3dSnapshot, mutation: &Puzzle3dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;
    *projection = next;
    Ok(())
}

#[path = "✋️selection/🦀️.rs"]
pub mod selection;
