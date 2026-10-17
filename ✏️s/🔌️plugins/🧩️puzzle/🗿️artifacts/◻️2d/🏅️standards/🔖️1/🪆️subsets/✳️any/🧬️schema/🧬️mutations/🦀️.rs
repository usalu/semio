//! 🧬️ Puzzle 2d artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<Puzzle2dSnapshot>` and `impl protocol::SemanticMutation<Puzzle2dSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here. `dsl::DslEnum` supplies
//! `DslVariants` (keyed off each payload's own `#[dsl(keyword = ...)]`), consumed by `OpText`/
//! `OpBinary` in the sibling `📝️text`/`💾️binary` modules.
//!
//! The native editor owns its immutable play root; semantic mutation authority remains the
//! canonical typed snapshot and its typed diff algebra.

use crate::standards::v1::subsets::any::schema::records::puzzle2d_targets_invariant;
use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;
use protocol::{DiffAlgebra, Mutation};
use semio_framework_value::{list::PagedList, paged::PagedUtf8};

//#region 🔖️Mutations
/// 🧮️ Semantic puzzle-2d document mutation vocabulary: id-keyed node/edge create-delete plus
/// per-field/per-facet edits (spatial, geometry, presentation flags, handle membership), a
/// handle-to-handle connect/disconnect relationship, document-meta edits (manifest reference,
/// kind-compatibility connect/disconnect, kind-catalog replace), and the three parametric selection
/// transforms (`drag-`, `rotate-`, `scale-selection`) that record a gesture's own inputs. There is deliberately no camera
/// mutation: the camera is session-only `Puzzle2dPlayRuntime` state in the play app (see
/// `setCamera`'s `ActionKind::View`), never a VCS-tracked document edit. There is deliberately no
/// whole-document mutation: import/reset/example-load goes through `store::ArtifactStore::reset`
/// (non-history), never through this enum.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = Puzzle2dSnapshot, diff = Puzzle2dDiff, schema = "puzzle.puzzle2d")]
pub enum Puzzle2dMutation {
    CreateNode(CreateNode),
    DeleteNode(DeleteNode),
    MoveNode(MoveNode),
    ReplaceNodeGeometry(ReplaceNodeGeometry),
    ChangeNodeKind(ChangeNodeKind),
    EditNodeText(EditNodeText),
    ChangeNodeIcon(ChangeNodeIcon),
    ScaleNode(ScaleNode),
    ChangeNodeVisible(ChangeNodeVisible),
    ChangeNodeLocked(ChangeNodeLocked),
    ChangeNodeRoot(ChangeNodeRoot),
    ChangeNodeAnchor(ChangeNodeAnchor),
    AddNodeHandle(AddNodeHandle),
    RemoveNodeHandle(RemoveNodeHandle),
    ReplaceNodeHandle(ReplaceNodeHandle),
    ConnectHandles(ConnectHandles),
    DisconnectHandles(DisconnectHandles),
    ReplaceEdgeGeometry(ReplaceEdgeGeometry),
    ChangeEdgeKind(ChangeEdgeKind),
    ChangeEdgeTips(ChangeEdgeTips),
    ChangeEdgeVisible(ChangeEdgeVisible),
    ChangeEdgeLocked(ChangeEdgeLocked),
    ChangeManifestId(ChangeManifestId),
    ConnectKindCompatibility(ConnectKindCompatibility),
    DisconnectKindCompatibility(DisconnectKindCompatibility),
    ReplaceKindCatalogs(ReplaceKindCatalogs),
    CreateTargetRegion(CreateTargetRegion),
    DeleteTargetRegion(DeleteTargetRegion),
    MoveTargetRegion(MoveTargetRegion),
    ResizeTargetRegion(ResizeTargetRegion),
    EditTargetRegionLabel(EditTargetRegionLabel),
    ChangeTargetRegionHidden(ChangeTargetRegionHidden),
    ChangeTargetRegionLocked(ChangeTargetRegionLocked),
    DragSelection(DragSelection),
    RotateSelection(RotateSelection),
    ScaleSelection(ScaleSelection),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Puzzle2dMutation`] variant, in declaration order — the exact
/// vocabulary the `puzzle-2d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `◻️mutate-puzzle-2d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-node",
    "delete-node",
    "move-node",
    "replace-node-geometry",
    "change-node-kind",
    "edit-node-text",
    "change-node-icon",
    "scale-node",
    "change-node-visible",
    "change-node-locked",
    "change-node-root",
    "change-node-anchor",
    "add-node-handle",
    "remove-node-handle",
    "replace-node-handle",
    "connect-handles",
    "disconnect-handles",
    "replace-edge-geometry",
    "change-edge-kind",
    "change-edge-tips",
    "change-edge-visible",
    "change-edge-locked",
    "change-manifest-id",
    "connect-kind-compatibility",
    "disconnect-kind-compatibility",
    "replace-kind-catalogs",
    "create-target-region",
    "delete-target-region",
    "move-target-region",
    "resize-target-region",
    "edit-target-region-label",
    "change-target-region-hidden",
    "change-target-region-locked",
    "drag-selection",
    "rotate-selection",
    "scale-selection",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

pub use crate::standards::v1::subsets::any::schema::mutations::add_node_handle::{add_node_handle, AddNodeHandle};
pub use crate::standards::v1::subsets::any::schema::mutations::change_edge_kind::{change_edge_kind, ChangeEdgeKind};
pub use crate::standards::v1::subsets::any::schema::mutations::change_edge_locked::{change_edge_locked, ChangeEdgeLocked};
pub use crate::standards::v1::subsets::any::schema::mutations::change_edge_tips::{change_edge_tips, ChangeEdgeTips};
pub use crate::standards::v1::subsets::any::schema::mutations::change_edge_visible::{change_edge_visible, ChangeEdgeVisible};
pub use crate::standards::v1::subsets::any::schema::mutations::change_manifest_id::{change_manifest_id, ChangeManifestId};
pub use crate::standards::v1::subsets::any::schema::mutations::change_node_anchor::{change_node_anchor, ChangeNodeAnchor};
pub use crate::standards::v1::subsets::any::schema::mutations::change_node_icon::{change_node_icon, ChangeNodeIcon};
pub use crate::standards::v1::subsets::any::schema::mutations::change_node_kind::{change_node_kind, ChangeNodeKind};
pub use crate::standards::v1::subsets::any::schema::mutations::change_node_locked::{change_node_locked, ChangeNodeLocked};
pub use crate::standards::v1::subsets::any::schema::mutations::change_node_root::{change_node_root, ChangeNodeRoot};
pub use crate::standards::v1::subsets::any::schema::mutations::change_node_visible::{change_node_visible, ChangeNodeVisible};
pub use crate::standards::v1::subsets::any::schema::mutations::change_target_region_hidden::{change_target_region_hidden, ChangeTargetRegionHidden};
pub use crate::standards::v1::subsets::any::schema::mutations::change_target_region_locked::{change_target_region_locked, ChangeTargetRegionLocked};
pub use crate::standards::v1::subsets::any::schema::mutations::connect_handles::{connect_handles, connect_handles_in_proximity, ConnectHandles};
pub use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::{connect_kind_compatibility, ConnectKindCompatibility};
pub use crate::standards::v1::subsets::any::schema::mutations::create_node::{create_node, CreateNode};
pub use crate::standards::v1::subsets::any::schema::mutations::create_target_region::{create_target_region, CreateTargetRegion};
pub use crate::standards::v1::subsets::any::schema::mutations::delete_node::{delete_node, DeleteNode};
pub use crate::standards::v1::subsets::any::schema::mutations::delete_target_region::{delete_target_region, DeleteTargetRegion};
pub use crate::standards::v1::subsets::any::schema::mutations::drag_selection::{drag_selection, DragSelection};
pub use crate::standards::v1::subsets::any::schema::mutations::disconnect_handles::{disconnect_handles, DisconnectHandles};
pub use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::{disconnect_kind_compatibility, DisconnectKindCompatibility};
pub use crate::standards::v1::subsets::any::schema::mutations::edit_node_text::{edit_node_text, EditNodeText};
pub use crate::standards::v1::subsets::any::schema::mutations::edit_target_region_label::{edit_target_region_label, EditTargetRegionLabel};
pub use crate::standards::v1::subsets::any::schema::mutations::move_node::{move_node, MoveNode};
pub use crate::standards::v1::subsets::any::schema::mutations::move_target_region::{move_target_region, MoveTargetRegion};
pub use crate::standards::v1::subsets::any::schema::mutations::remove_node_handle::{remove_node_handle, RemoveNodeHandle};
pub use crate::standards::v1::subsets::any::schema::mutations::replace_edge_geometry::{replace_edge_geometry, ReplaceEdgeGeometry};
pub use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::{replace_kind_catalogs, ReplaceKindCatalogs};
pub use crate::standards::v1::subsets::any::schema::mutations::replace_node_geometry::{replace_node_geometry, ReplaceNodeGeometry};
pub use crate::standards::v1::subsets::any::schema::mutations::replace_node_handle::{replace_node_handle, ReplaceNodeHandle};
pub use crate::standards::v1::subsets::any::schema::mutations::resize_target_region::{resize_target_region, ResizeTargetRegion};
pub use crate::standards::v1::subsets::any::schema::mutations::rotate_selection::{rotate_selection, RotateSelection};
pub use crate::standards::v1::subsets::any::schema::mutations::scale_node::{scale_node, ScaleNode};
pub use crate::standards::v1::subsets::any::schema::mutations::scale_selection::{scale_selection, ScaleSelection};

//#region 🔖️SelectionTransform











//#endregion 🔖️SelectionTransform

//#region 🔖️Invariants















//#endregion 🔖️Invariants

//#region 🔖️HandleGeometry



//#endregion 🔖️HandleGeometry

//#region 🎚️DeclaredPrecision

//#endregion 🎚️DeclaredPrecision



pub fn inverse_puzzle2d_mutation(projection: &Puzzle2dSnapshot, mutation: &Puzzle2dMutation) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}





//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests



use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;

/// ▶️ Applies one mutation to a projection through the central applier (`vcs::apply_mutation`): the step of the editor tools' charts and of the mutation tests.
pub fn apply_puzzle2d_mutation(projection: &mut Puzzle2dSnapshot, mutation: &Puzzle2dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;
    *projection = next;
    Ok(())
}

pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::selection;
