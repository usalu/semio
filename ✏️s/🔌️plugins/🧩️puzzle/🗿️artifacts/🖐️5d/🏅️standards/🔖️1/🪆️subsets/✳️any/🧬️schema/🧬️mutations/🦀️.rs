//! 🧬️ Puzzle 5d artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<Puzzle5dSnapshot>` and `impl protocol::SemanticMutation<Puzzle5dSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here. `dsl::DslEnum` supplies
//! `DslVariants`, consumed by `OpText`/`OpBinary` in the sibling `📝️text`/`💾️binary` modules.
//!
//! The `serde_json::Value` bridge (`🔖️ValueBridge`) and the play app's `Puzzle5dPlaySnapshot`
//! newtype (`🔖️PlaySnapshot`) live here too, same shape as `puzzle2d`'s: the bridge round-trips
//! through the typed `Puzzle5dSnapshot` instead of hand-splicing JSON per mutation kind.

use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;
use protocol::{DiffAlgebra, Mutation};

//#region 🔖️Mutations
/// 🧮️ Semantic puzzle-5d document mutation vocabulary: id-keyed part create-delete plus per-2d/
/// per-3d-projection field edits, grip membership, a grip-to-grip fastener connect/disconnect
/// relationship, and document-level edits (label rename, domain/description change,
/// kind-compatibility connect/disconnect, kind-catalog replace), and the four parametric selection
/// transforms (`drag-selection2d` on the board, `drag-`, `rotate-`, `scale-selection3d` in the world) that
/// record a gesture's own inputs. There is deliberately no camera
/// mutation: camera pose is session-only app runtime state (`ActionKind::View`), never a document
/// operation. There is deliberately no whole-document mutation: import/reset/example-load goes
/// through `store::ArtifactStore::reset` (non-history), never through this enum.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = Puzzle5dSnapshot, diff = Puzzle5dDiff, schema = "puzzle.puzzle5d")]
pub enum Puzzle5dMutation {
    CreatePart(CreatePart),
    DeletePart(DeletePart),
    MovePart2d(MovePart2d),
    ReplacePart2dGeometry(ReplacePart2dGeometry),
    EditPart2dText(EditPart2dText),
    ChangePart2dIcon(ChangePart2dIcon),
    ChangePart2dHidden(ChangePart2dHidden),
    ChangePart2dLocked(ChangePart2dLocked),
    MovePart3d(MovePart3d),
    RotatePart3d(RotatePart3d),
    ScalePart3d(ScalePart3d),
    ChangePart3dMesh(ChangePart3dMesh),
    EditPart3dLabel(EditPart3dLabel),
    ChangePartKind(ChangePartKind),
    ChangePartAnchor(ChangePartAnchor),
    AddPartGrip(AddPartGrip),
    RemovePartGrip(RemovePartGrip),
    ReplacePartGrip(ReplacePartGrip),
    ConnectGrips(ConnectGrips),
    DisconnectGrips(DisconnectGrips),
    ReplaceFastenerGeometry(ReplaceFastenerGeometry),
    ChangeFastenerKind(ChangeFastenerKind),
    RenamePuzzle5d(RenamePuzzle5d),
    ChangeDomain(ChangeDomain),
    ChangeDescription(ChangeDescription),
    ConnectKindCompatibility(ConnectKindCompatibility),
    DisconnectKindCompatibility(DisconnectKindCompatibility),
    ReplaceKindCatalogs(ReplaceKindCatalogs),
    CreateTargetVolume(CreateTargetVolume),
    DeleteTargetVolume(DeleteTargetVolume),
    MoveTargetVolume(MoveTargetVolume),
    RotateTargetVolume(RotateTargetVolume),
    ScaleTargetVolume(ScaleTargetVolume),
    ChangeTargetVolumeHidden(ChangeTargetVolumeHidden),
    ChangeTargetVolumeLocked(ChangeTargetVolumeLocked),
    DragSelection2d(DragSelection2d),
    DragSelection3d(DragSelection3d),
    RotateSelection3d(RotateSelection3d),
    ScaleSelection3d(ScaleSelection3d),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Puzzle5dMutation`] variant, in declaration order — the exact
/// vocabulary the `puzzle-5d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🖐️mutate-puzzle-5d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-part",
    "delete-part",
    "move-part2d",
    "replace-part2d-geometry",
    "edit-part2d-text",
    "change-part2d-icon",
    "change-part2d-hidden",
    "change-part2d-locked",
    "move-part3d",
    "rotate-part3d",
    "scale-part3d",
    "change-part3d-mesh",
    "edit-part3d-label",
    "change-part-kind",
    "change-part-anchor",
    "add-part-grip",
    "remove-part-grip",
    "replace-part-grip",
    "connect-grips",
    "disconnect-grips",
    "replace-fastener-geometry",
    "change-fastener-kind",
    "rename-puzzle5d",
    "change-domain",
    "change-description",
    "connect-kind-compatibility",
    "disconnect-kind-compatibility",
    "replace-kind-catalogs",
    "create-target-volume",
    "delete-target-volume",
    "move-target-volume",
    "rotate-target-volume",
    "scale-target-volume",
    "change-target-volume-hidden",
    "change-target-volume-locked",
    "drag-selection2d",
    "drag-selection3d",
    "rotate-selection3d",
    "scale-selection3d",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

pub use crate::standards::v1::subsets::any::schema::mutations::add_part_grip::{add_part_grip, AddPartGrip};
pub use crate::standards::v1::subsets::any::schema::mutations::change_description::ChangeDescription;
pub use crate::standards::v1::subsets::any::schema::mutations::change_domain::ChangeDomain;
pub use crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind::ChangeFastenerKind;
pub use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::ChangePart2dHidden;
pub use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::ChangePart2dIcon;
pub use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::ChangePart2dLocked;
pub use crate::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::ChangePart3dMesh;
pub use crate::standards::v1::subsets::any::schema::mutations::change_part_anchor::ChangePartAnchor;
pub use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::ChangeTargetVolumeHidden;
pub use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::ChangeTargetVolumeLocked;
pub use crate::standards::v1::subsets::any::schema::mutations::change_part_kind::ChangePartKind;
pub use crate::standards::v1::subsets::any::schema::mutations::connect_grips::ConnectGrips;
pub use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::{connect_kind_compatibility, ConnectKindCompatibility};
pub use crate::standards::v1::subsets::any::schema::mutations::create_part::{create_part, CreatePart};
pub use crate::standards::v1::subsets::any::schema::mutations::create_target_volume::{create_target_volume, CreateTargetVolume};
pub use crate::standards::v1::subsets::any::schema::mutations::delete_part::DeletePart;
pub use crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::DeleteTargetVolume;
pub use crate::standards::v1::subsets::any::schema::mutations::disconnect_grips::DisconnectGrips;
pub use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::DisconnectKindCompatibility;
pub use crate::standards::v1::subsets::any::schema::mutations::drag_selection_2d::DragSelection2d;
pub use crate::standards::v1::subsets::any::schema::mutations::drag_selection_3d::DragSelection3d;
pub use crate::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::EditPart2dText;
pub use crate::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::EditPart3dLabel;
pub use crate::standards::v1::subsets::any::schema::mutations::move_part_2d::MovePart2d;
pub use crate::standards::v1::subsets::any::schema::mutations::move_part_3d::MovePart3d;
pub use crate::standards::v1::subsets::any::schema::mutations::move_target_volume::MoveTargetVolume;
pub use crate::standards::v1::subsets::any::schema::mutations::remove_part_grip::RemovePartGrip;
pub use crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::RenamePuzzle5d;
pub use crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::ReplaceFastenerGeometry;
pub use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::{replace_kind_catalogs, ReplaceKindCatalogs};
pub use crate::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::ReplacePart2dGeometry;
pub use crate::standards::v1::subsets::any::schema::mutations::replace_part_grip::{replace_part_grip, ReplacePartGrip};
pub use crate::standards::v1::subsets::any::schema::mutations::rotate_part_3d::RotatePart3d;
pub use crate::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::RotateSelection3d;
pub use crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::RotateTargetVolume;
pub use crate::standards::v1::subsets::any::schema::mutations::scale_part_3d::ScalePart3d;
pub use crate::standards::v1::subsets::any::schema::mutations::scale_selection_3d::ScaleSelection3d;
pub use crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::ScaleTargetVolume;
pub use {semio_s_artifact_puzzle_3d_mutations_selection::puzzle3d_selection_items as puzzle5d_selection_items, semio_s_artifact_puzzle_3d_mutations_selection::puzzle3d_selection_number as puzzle5d_selection_number, semio_s_artifact_puzzle_3d_mutations_selection::puzzle3d_selection_triple as puzzle5d_selection_triple, semio_s_artifact_puzzle_3d_mutations_selection::puzzle3d_targets_invariant as puzzle5d_targets_invariant, semio_s_artifact_puzzle_3d_model::quat_from_axis_angle, semio_s_artifact_puzzle_3d_model::quat_mul};
pub use semio_s_artifact_puzzle_5d_kind_catalogs::standards::v1::subsets::any::schema::mutations::support;
pub use support::{PUZZLE5D_FLAT_TO_WORLD, puzzle5d_scaled};






pub fn inverse_puzzle5d_mutation(projection: &Puzzle5dSnapshot, mutation: &Puzzle5dMutation) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}





//#region 🧪️Tests



#[cfg(test)]
#[path = "🧪️tests/🔬️committed-fixtures/🦀️.rs"]
mod committed_fixtures;
//#endregion 🧪️Tests

//#region 🔖️ConcreteBuilders
/// 🏗️ Builder of the `change_description` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_description(new_description: String) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_description::change_description::<Puzzle5dMutation>(new_description)
}

/// 🏗️ Builder of the `change_domain` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_domain(new_domain: String) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_domain::change_domain::<Puzzle5dMutation>(new_domain)
}

/// 🏗️ Builder of the `change_fastener_kind` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_fastener_kind(id: String, new_fastener_kind: Option<String>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind::change_fastener_kind::<Puzzle5dMutation>(id, new_fastener_kind)
}

/// 🏗️ Builder of the `change_part_2d_hidden` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_part_2d_hidden(id: String, new_hidden: Option<bool>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::change_part_2d_hidden::<Puzzle5dMutation>(id, new_hidden)
}

/// 🏗️ Builder of the `change_part_2d_icon` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_part_2d_icon(id: String, new_icon_kind: Option<String>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::change_part_2d_icon::<Puzzle5dMutation>(id, new_icon_kind)
}

/// 🏗️ Builder of the `change_part_2d_locked` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_part_2d_locked(id: String, new_locked: Option<bool>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::change_part_2d_locked::<Puzzle5dMutation>(id, new_locked)
}

/// 🏗️ Builder of the `change_part_3d_mesh` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_part_3d_mesh(id: String, new_mesh_url: Option<String>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::change_part_3d_mesh::<Puzzle5dMutation>(id, new_mesh_url)
}

/// 🏗️ Builder of the `change_part_anchor` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_part_anchor(id: String, new_anchor: crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPartAnchor) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_part_anchor::change_part_anchor::<Puzzle5dMutation>(id, new_anchor)
}

/// 🏗️ Builder of the `change_part_kind` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_part_kind(id: String, new_part_kind: Option<String>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_part_kind::change_part_kind::<Puzzle5dMutation>(id, new_part_kind)
}

/// 🏗️ Builder of the `change_target_volume_hidden` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_target_volume_hidden(id: String, new_hidden: bool) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::change_target_volume_hidden::<Puzzle5dMutation>(id, new_hidden)
}

/// 🏗️ Builder of the `change_target_volume_locked` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_target_volume_locked(id: String, new_locked: bool) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::change_target_volume_locked::<Puzzle5dMutation>(id, new_locked)
}

/// 🏗️ Builder of the `connect_grips` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn connect_grips(id: String, source: String, target: String, fastener_kind: Option<String>, gap: f64, shift: f64, rise: f64, rotation: f64, turn: f64, tilt: f64, x: f64, y: f64, index: Option<usize>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::connect_grips::connect_grips::<Puzzle5dMutation>(id, source, target, fastener_kind, gap, shift, rise, rotation, turn, tilt, x, y, index)
}

/// 🏗️ Builder of the `delete_part` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_part(id: String) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::delete_part::delete_part::<Puzzle5dMutation>(id)
}

/// 🏗️ Builder of the `delete_target_volume` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_target_volume(id: String) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::delete_target_volume::<Puzzle5dMutation>(id)
}

/// 🏗️ Builder of the `disconnect_grips` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn disconnect_grips(id: String) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::disconnect_grips::disconnect_grips::<Puzzle5dMutation>(id)
}

/// 🏗️ Builder of the `disconnect_kind_compatibility` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn disconnect_kind_compatibility(source: String, target: String) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::disconnect_kind_compatibility::<Puzzle5dMutation>(source, target)
}

/// 🏗️ Builder of the `drag_selection_2d` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn drag_selection_2d(targets: Vec<String>, dx: f64, dy: f64) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::drag_selection_2d::drag_selection_2d::<Puzzle5dMutation>(targets, dx, dy)
}

/// 🏗️ Builder of the `drag_selection_3d` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn drag_selection_3d(targets: Vec<String>, offset: [f64; 3]) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::drag_selection_3d::drag_selection_3d::<Puzzle5dMutation>(targets, offset)
}

/// 🏗️ Builder of the `edit_part_2d_text` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn edit_part_2d_text(id: String, new_text: Option<String>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::edit_part_2d_text::<Puzzle5dMutation>(id, new_text)
}

/// 🏗️ Builder of the `edit_part_3d_label` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn edit_part_3d_label(id: String, new_label: Option<String>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::edit_part_3d_label::<Puzzle5dMutation>(id, new_label)
}

/// 🏗️ Builder of the `move_part_2d` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn move_part_2d(id: String, new_x: f64, new_y: f64) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::move_part_2d::move_part_2d::<Puzzle5dMutation>(id, new_x, new_y)
}

/// 🏗️ Builder of the `move_part_3d` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn move_part_3d(id: String, new_origin: [f64; 3]) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::move_part_3d::move_part_3d::<Puzzle5dMutation>(id, new_origin)
}

/// 🏗️ Builder of the `move_target_volume` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn move_target_volume(id: String, new_origin: [f64; 3]) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::move_target_volume::move_target_volume::<Puzzle5dMutation>(id, new_origin)
}

/// 🏗️ Builder of the `remove_part_grip` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn remove_part_grip(part_id: String, grip_id: String) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::remove_part_grip::remove_part_grip::<Puzzle5dMutation>(part_id, grip_id)
}

/// 🏗️ Builder of the `rename_puzzle5d` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn rename_puzzle5d(new_label: Option<String>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::rename_puzzle5d::<Puzzle5dMutation>(new_label)
}

/// 🏗️ Builder of the `replace_fastener_geometry` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn replace_fastener_geometry(geometry: ReplaceFastenerGeometry) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::replace_fastener_geometry::<Puzzle5dMutation>(geometry)
}

/// 🏗️ Builder of the `replace_part_2d_geometry` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn replace_part_2d_geometry(id: String, new_shape: Option<String>, new_radius: Option<f64>, new_width: Option<f64>, new_height: Option<f64>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::replace_part_2d_geometry::<Puzzle5dMutation>(id, new_shape, new_radius, new_width, new_height)
}

/// 🏗️ Builder of the `rotate_part_3d` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn rotate_part_3d(id: String, new_orientation: Option<[f64; 4]>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::rotate_part_3d::rotate_part_3d::<Puzzle5dMutation>(id, new_orientation)
}

/// 🏗️ Builder of the `rotate_selection_3d` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn rotate_selection_3d(targets: Vec<String>, axis: [f64; 3], angle: f64) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::rotate_selection_3d::<Puzzle5dMutation>(targets, axis, angle)
}

/// 🏗️ Builder of the `rotate_target_volume` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn rotate_target_volume(id: String, new_orientation: Option<[f64; 4]>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::rotate_target_volume::<Puzzle5dMutation>(id, new_orientation)
}

/// 🏗️ Builder of the `scale_part_3d` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn scale_part_3d(id: String, new_scale: Option<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dScale>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::scale_part_3d::scale_part_3d::<Puzzle5dMutation>(id, new_scale)
}

/// 🏗️ Builder of the `scale_selection_3d` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn scale_selection_3d(targets: Vec<String>, factors: [f64; 3]) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::scale_selection_3d::scale_selection_3d::<Puzzle5dMutation>(targets, factors)
}

/// 🏗️ Builder of the `scale_target_volume` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn scale_target_volume(id: String, new_scale: Option<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dScale>) -> Puzzle5dMutation {
    crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::scale_target_volume::<Puzzle5dMutation>(id, new_scale)
}
//#endregion 🔖️ConcreteBuilders

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
pub use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;

/// ▶️ Applies one mutation to a projection through the central applier (`vcs::apply_mutation`): the step of the editor tools' charts and of the mutation tests.
pub fn apply_puzzle5d_mutation(projection: &mut Puzzle5dSnapshot, mutation: &Puzzle5dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;
    *projection = next;
    Ok(())
}
}
//#endregion 🧩️ArtifactItems6
