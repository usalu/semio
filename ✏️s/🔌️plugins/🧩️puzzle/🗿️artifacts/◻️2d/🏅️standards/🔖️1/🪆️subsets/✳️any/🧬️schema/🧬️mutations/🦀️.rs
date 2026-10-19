//! ☝️ single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! 🧬️ Puzzle 2d artifact — semantic document mutation dispatch enum. Every variant is a
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
use protocol::ColdMutation;
use protocol::ColdDiffAlgebra;
use protocol::DiffAlgebra;
use protocol::Mutation;
use semio_framework_value_persistent::list::PagedList;
use semio_framework_value::paged::PagedUtf8;

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
#[mutations(snapshot = Puzzle2dSnapshot, diff = Puzzle2dDiff, schema = "puzzle.puzzle2d", reference = Puzzle2dSnapshot, reference_diff = Puzzle2dDiff)]
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

pub use crate::standards::v1::subsets::any::schema::mutations::add_node_handle::add_node_handle;

pub use super::add_node_handle::AddNodeHandle;
pub use crate::standards::v1::subsets::any::schema::mutations::change_edge_kind::change_edge_kind;
pub use super::change_edge_kind::ChangeEdgeKind;
pub use crate::standards::v1::subsets::any::schema::mutations::change_edge_locked::change_edge_locked;
pub use super::change_edge_locked::ChangeEdgeLocked;
pub use crate::standards::v1::subsets::any::schema::mutations::change_edge_tips::change_edge_tips;
pub use super::change_edge_tips::ChangeEdgeTips;
pub use crate::standards::v1::subsets::any::schema::mutations::change_edge_visible::change_edge_visible;
pub use super::change_edge_visible::ChangeEdgeVisible;
pub use crate::standards::v1::subsets::any::schema::mutations::change_manifest_id::change_manifest_id;
pub use super::change_manifest_id::ChangeManifestId;
pub use crate::standards::v1::subsets::any::schema::mutations::change_node_anchor::change_node_anchor;
pub use super::change_node_anchor::ChangeNodeAnchor;
pub use crate::standards::v1::subsets::any::schema::mutations::change_node_icon::change_node_icon;
pub use super::change_node_icon::ChangeNodeIcon;
pub use crate::standards::v1::subsets::any::schema::mutations::change_node_kind::change_node_kind;
pub use super::change_node_kind::ChangeNodeKind;
pub use crate::standards::v1::subsets::any::schema::mutations::change_node_locked::change_node_locked;
pub use super::change_node_locked::ChangeNodeLocked;
pub use crate::standards::v1::subsets::any::schema::mutations::change_node_root::change_node_root;
pub use super::change_node_root::ChangeNodeRoot;
pub use crate::standards::v1::subsets::any::schema::mutations::change_node_visible::change_node_visible;
pub use super::change_node_visible::ChangeNodeVisible;
pub use crate::standards::v1::subsets::any::schema::mutations::change_target_region_hidden::change_target_region_hidden;
pub use super::change_target_region_hidden::ChangeTargetRegionHidden;
pub use crate::standards::v1::subsets::any::schema::mutations::change_target_region_locked::change_target_region_locked;
pub use super::change_target_region_locked::ChangeTargetRegionLocked;
pub use crate::standards::v1::subsets::any::schema::mutations::connect_handles::connect_handles;
pub use super::connect_handles::connect_handles_in_proximity;
pub use super::connect_handles::ConnectHandles;
pub use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::connect_kind_compatibility;
pub use super::connect_kind_compatibility::ConnectKindCompatibility;
pub use crate::standards::v1::subsets::any::schema::mutations::create_node::create_node;
pub use super::create_node::CreateNode;
pub use crate::standards::v1::subsets::any::schema::mutations::create_target_region::create_target_region;
pub use super::create_target_region::CreateTargetRegion;
pub use crate::standards::v1::subsets::any::schema::mutations::delete_node::delete_node;
pub use super::delete_node::DeleteNode;
pub use crate::standards::v1::subsets::any::schema::mutations::delete_target_region::delete_target_region;
pub use super::delete_target_region::DeleteTargetRegion;
pub use crate::standards::v1::subsets::any::schema::mutations::drag_selection::drag_selection;
pub use super::drag_selection::DragSelection;
pub use crate::standards::v1::subsets::any::schema::mutations::disconnect_handles::disconnect_handles;
pub use super::disconnect_handles::DisconnectHandles;
pub use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::disconnect_kind_compatibility;
pub use super::disconnect_kind_compatibility::DisconnectKindCompatibility;
pub use crate::standards::v1::subsets::any::schema::mutations::edit_node_text::edit_node_text;
pub use super::edit_node_text::EditNodeText;
pub use crate::standards::v1::subsets::any::schema::mutations::edit_target_region_label::edit_target_region_label;
pub use super::edit_target_region_label::EditTargetRegionLabel;
pub use crate::standards::v1::subsets::any::schema::mutations::move_node::move_node;
pub use super::move_node::MoveNode;
pub use crate::standards::v1::subsets::any::schema::mutations::move_target_region::move_target_region;
pub use super::move_target_region::MoveTargetRegion;
pub use crate::standards::v1::subsets::any::schema::mutations::remove_node_handle::remove_node_handle;
pub use super::remove_node_handle::RemoveNodeHandle;
pub use crate::standards::v1::subsets::any::schema::mutations::replace_edge_geometry::replace_edge_geometry;
pub use super::replace_edge_geometry::ReplaceEdgeGeometry;
pub use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::replace_kind_catalogs;
pub use super::replace_kind_catalogs::ReplaceKindCatalogs;
pub use crate::standards::v1::subsets::any::schema::mutations::replace_node_geometry::replace_node_geometry;
pub use super::replace_node_geometry::ReplaceNodeGeometry;
pub use crate::standards::v1::subsets::any::schema::mutations::replace_node_handle::replace_node_handle;
pub use super::replace_node_handle::ReplaceNodeHandle;
pub use crate::standards::v1::subsets::any::schema::mutations::resize_target_region::resize_target_region;
pub use super::resize_target_region::ResizeTargetRegion;
pub use crate::standards::v1::subsets::any::schema::mutations::rotate_selection::rotate_selection;
pub use super::rotate_selection::RotateSelection;
pub use crate::standards::v1::subsets::any::schema::mutations::scale_node::scale_node;
pub use super::scale_node::ScaleNode;
pub use crate::standards::v1::subsets::any::schema::mutations::scale_selection::scale_selection;
pub use super::scale_selection::ScaleSelection;

//#region 🔖️SelectionTransform





/// 🔍️ One coordinate scaled about a pivot. The unit factor is the identity exactly (`pivot + (value - pivot)` alone lands a ulp off
/// for an awkward pivot), so an identity scaling states no row.
pub fn puzzle2d_scaled(pivot: f64, value: f64, factor: f64) -> f64 {
    if factor == 1.0 { value } else { pivot + (value - pivot) * factor }
}









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



/// ▶️ Applies one mutation to a projection through the central applier (`vcs::apply_mutation`): the step of the editor tools' charts and of the mutation tests.
pub fn apply_puzzle2d_mutation(projection: &mut Puzzle2dSnapshot, mutation: &Puzzle2dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;
    *projection = next;
    Ok(())
}

pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::selection;
