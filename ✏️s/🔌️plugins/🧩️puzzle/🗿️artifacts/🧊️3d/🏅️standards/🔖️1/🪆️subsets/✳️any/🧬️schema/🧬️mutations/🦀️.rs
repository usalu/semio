//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! 🧬️ Puzzle 3d artifact — semantic document mutation dispatch enum. Every variant is a
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<Puzzle3dSnapshot>` and `impl protocol::SemanticMutation<Puzzle3dSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here. `dsl::DslEnum` supplies
//! `DslVariants`, consumed by `OpText`/`OpBinary` in the sibling `📝️text`/`💾️binary` modules.
//!
//! The native editor owns its immutable play root; semantic mutation authority remains the
//! canonical typed snapshot and its typed diff algebra.

use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use crate::Puzzle3dSnapshot;
use protocol::ColdMutation;
use protocol::ColdDiffAlgebra;
use protocol::DiffAlgebra;
use protocol::Mutation;

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
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = Puzzle3dSnapshot, diff = Puzzle3dDiff, schema = "puzzle.puzzle3d", reference = Puzzle3dSnapshot, reference_diff = Puzzle3dDiff)]
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

use semio_s_artifact_puzzle_3d_mutations_vortex::add_object_vortex::mutation::AddObjectVortex;
use semio_s_artifact_puzzle_3d_mutations_catalog::change_domain::mutation::ChangeDomain;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_anchor::mutation::ChangeObjectAnchor;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_hidden::mutation::ChangeObjectHidden;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_kind::mutation::ChangeObjectKind;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_locked::mutation::ChangeObjectLocked;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_mesh::mutation::ChangeObjectMesh;
use semio_s_artifact_puzzle_3d_mutations_reference::change_reference_hidden::mutation::ChangeReferenceHidden;
use semio_s_artifact_puzzle_3d_mutations_reference::change_reference_locked::mutation::ChangeReferenceLocked;
use semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_hidden::mutation::ChangeTargetVolumeHidden;
use semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_locked::mutation::ChangeTargetVolumeLocked;
use semio_s_artifact_puzzle_3d_mutations_catalog::connect_kind_compatibility::mutation::ConnectKindCompatibility;
use semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::ConnectVortices;
use semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::CreateObject;
use semio_s_artifact_puzzle_3d_mutations_reference::create_reference::mutation::CreateReference;
use semio_s_artifact_puzzle_3d_mutations_volume::create_target_volume::mutation::CreateTargetVolume;
use semio_s_artifact_puzzle_3d_mutations_object::delete_object::mutation::DeleteObject;
use semio_s_artifact_puzzle_3d_mutations_reference::delete_reference::mutation::DeleteReference;
use semio_s_artifact_puzzle_3d_mutations_volume::delete_target_volume::mutation::DeleteTargetVolume;
use semio_s_artifact_puzzle_3d_mutations_catalog::disconnect_kind_compatibility::mutation::DisconnectKindCompatibility;
use semio_s_artifact_puzzle_3d_mutations_vortex::disconnect_vortices::mutation::DisconnectVortices;
use semio_s_artifact_puzzle_3d_mutations_selection::drag_selection::mutation::DragSelection;
use semio_s_artifact_puzzle_3d_mutations_object::edit_object_label::mutation::EditObjectLabel;
use semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::MoveObject;
use semio_s_artifact_puzzle_3d_mutations_reference::move_reference::mutation::MoveReference;
use semio_s_artifact_puzzle_3d_mutations_volume::move_target_volume::mutation::MoveTargetVolume;
use semio_s_artifact_puzzle_3d_mutations_vortex::remove_object_vortex::mutation::RemoveObjectVortex;
use semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::ReplaceAttractionGeometry;
use semio_s_artifact_puzzle_3d_mutations_catalog::replace_kind_catalogs::mutation::ReplaceKindCatalogs;
use semio_s_artifact_puzzle_3d_mutations_vortex::replace_object_vortex::mutation::ReplaceObjectVortex;
use semio_s_artifact_puzzle_3d_mutations_reference::replace_reference_source::mutation::ReplaceReferenceSource;
use semio_s_artifact_puzzle_3d_mutations_reference::resize_reference::mutation::ResizeReference;
use semio_s_artifact_puzzle_3d_mutations_pose::rotate_object::mutation::RotateObject;
use semio_s_artifact_puzzle_3d_mutations_selection::rotate_selection::mutation::RotateSelection;
use semio_s_artifact_puzzle_3d_mutations_volume::rotate_target_volume::mutation::RotateTargetVolume;
use semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::ScaleObject;
use semio_s_artifact_puzzle_3d_mutations_selection::scale_selection::mutation::ScaleSelection;
use semio_s_artifact_puzzle_3d_mutations_volume::scale_target_volume::mutation::ScaleTargetVolume;

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

#[cfg(test)]
#[path = "🧪️tests/🔬️committed-fixtures/🦀️.rs"]
mod committed_fixtures;
//#endregion 🧪️Tests

/// ▶️ Applies one mutation to a projection through the central applier (`vcs::apply_mutation`): the step of the editor tools' charts and of the mutation tests.
pub fn apply_puzzle3d_mutation(projection: &mut Puzzle3dSnapshot, mutation: &Puzzle3dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;
    *projection = next;
    Ok(())
}

//#region 🔖️SchemaLeaves
/// 🧬️ The mutations facet's five handcrafted schema leaves.
pub const SCHEMA_LEAVES: ::semio_framework_schema_registry::FacetLeaves = ::semio_framework_schema_registry::FacetLeaves {
    rust: include_str!("🦀️.rs"),
    typescript: include_str!("🟦️.ts"),
    graphql: include_str!("🔗️.graphql"),
    json_schema: include_str!("🔣️.json"),
    proto: include_str!("🛰️.proto"),
};
//#endregion 🔖️SchemaLeaves

//#region 🌳️Modules
#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
    #[path = "."]
    pub mod text {
        #[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
}
//#endregion 🌳️Modules
