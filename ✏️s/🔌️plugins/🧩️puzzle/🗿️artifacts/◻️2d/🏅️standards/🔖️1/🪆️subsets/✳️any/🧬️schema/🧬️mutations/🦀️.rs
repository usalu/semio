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

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;
use protocol::{DiffAlgebra, Mutation, MutationDiff};
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

pub use super::add_node_handle::{add_node_handle, AddNodeHandle};
pub use super::change_edge_kind::{change_edge_kind, ChangeEdgeKind};
pub use super::change_edge_locked::{change_edge_locked, ChangeEdgeLocked};
pub use super::change_edge_tips::{change_edge_tips, ChangeEdgeTips};
pub use super::change_edge_visible::{change_edge_visible, ChangeEdgeVisible};
pub use super::change_manifest_id::{change_manifest_id, ChangeManifestId};
pub use super::change_node_anchor::{change_node_anchor, ChangeNodeAnchor};
pub use super::change_node_icon::{change_node_icon, ChangeNodeIcon};
pub use super::change_node_kind::{change_node_kind, ChangeNodeKind};
pub use super::change_node_locked::{change_node_locked, ChangeNodeLocked};
pub use super::change_node_root::{change_node_root, ChangeNodeRoot};
pub use super::change_node_visible::{change_node_visible, ChangeNodeVisible};
pub use super::change_target_region_hidden::{change_target_region_hidden, ChangeTargetRegionHidden};
pub use super::change_target_region_locked::{change_target_region_locked, ChangeTargetRegionLocked};
pub use super::connect_handles::{connect_handles, connect_handles_in_proximity, ConnectHandles};
pub use super::connect_kind_compatibility::{connect_kind_compatibility, ConnectKindCompatibility};
pub use super::create_node::{create_node, CreateNode};
pub use super::create_target_region::{create_target_region, CreateTargetRegion};
pub use super::delete_node::{delete_node, DeleteNode};
pub use super::delete_target_region::{delete_target_region, DeleteTargetRegion};
pub use super::drag_selection::{drag_selection, DragSelection};
pub use super::disconnect_handles::{disconnect_handles, DisconnectHandles};
pub use super::disconnect_kind_compatibility::{disconnect_kind_compatibility, DisconnectKindCompatibility};
pub use super::edit_node_text::{edit_node_text, EditNodeText};
pub use super::edit_target_region_label::{edit_target_region_label, EditTargetRegionLabel};
pub use super::move_node::{move_node, MoveNode};
pub use super::move_target_region::{move_target_region, MoveTargetRegion};
pub use super::remove_node_handle::{remove_node_handle, RemoveNodeHandle};
pub use super::replace_edge_geometry::{replace_edge_geometry, ReplaceEdgeGeometry};
pub use super::replace_kind_catalogs::{replace_kind_catalogs, ReplaceKindCatalogs};
pub use super::replace_node_geometry::{replace_node_geometry, ReplaceNodeGeometry};
pub use super::replace_node_handle::{replace_node_handle, ReplaceNodeHandle};
pub use super::resize_target_region::{resize_target_region, ResizeTargetRegion};
pub use super::rotate_selection::{rotate_selection, RotateSelection};
pub use super::scale_node::{scale_node, ScaleNode};
pub use super::scale_selection::{scale_selection, ScaleSelection};

//#region 🔖️SelectionTransform
/// 🧭️ The members of a parametric selection leaf's target set the transform acts on, in document order, with the
/// `mutation.partial` warnings of the ones it skips. `targets` is classified by document membership against `base`: a
/// node id goes through `nodes`, a target-region id through `regions` (none when the transform has no meaning for an
/// axis-aligned rectangle). An empty or repeated target set is the Fatal `mutation.invariant` the payload schema's
/// `minItems`/`uniqueItems` forbid. Absent ids, locked members and inapplicable regions are skipped with one warning
/// per reason (in that order, ids in payload order); nothing left is `mutation.target-missing`.
pub struct Puzzle2dSelection<'a> {
    pub nodes: Vec<&'a crate::Puzzle2dNode>,
    pub regions: Vec<&'a crate::Puzzle2dTargetRegion>,
    pub warnings: Vec<protocol::MutationMessage>,
}

/// 🗃️ Classifies a selection leaf's `targets` against `base`; `Err` is the leaf's final refusal outcome.
pub fn puzzle2d_selection<'a>(base: &'a Puzzle2dSnapshot, targets: &PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>, regions_apply: bool) -> Result<Puzzle2dSelection<'a>, protocol::MutationOutcome<Puzzle2dDiff>> {
    let owners = || targets.iter().map(PagedUtf8::to_string_owner).collect::<Vec<_>>();
    if let Err(reason) = puzzle2d_targets_invariant(targets) {
        return Err(protocol::MutationOutcome::fatal("mutation.invariant", reason, owners()));
    }
    let (mut missing, mut locked, mut fixed, mut survivors) = (Vec::<String>::new(), Vec::<String>::new(), Vec::<String>::new(), std::collections::BTreeSet::<&PagedUtf8<{ usize::MAX }>>::new());
    for id in targets {
        match (base.nodes.iter().find(|entry| &entry.id == id), base.target_regions.iter().find(|entry| &entry.id == id)) {
            (Some(entry), _) if entry.locked != Some(true) => {
                survivors.insert(id);
            }
            (Some(_), _) => locked.push(id.to_string_owner()),
            (None, Some(entry)) if entry.locked => locked.push(id.to_string_owner()),
            (None, Some(_)) if !regions_apply => fixed.push(id.to_string_owner()),
            (None, Some(_)) => {
                survivors.insert(id);
            }
            (None, None) => missing.push(id.to_string_owner()),
        }
    }
    if survivors.is_empty() {
        return Err(protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is an unlocked node or target region this transform applies to", targets.len()), owners()));
    }
    let warnings = [(missing, "not on this board"), (locked, "locked"), (fixed, "axis-aligned target regions do not rotate")]
        .into_iter()
        .filter(|(ids, _)| !ids.is_empty())
        .map(|(ids, reason)| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped ({reason}): {}", ids.len(), targets.len(), ids.join(", "))).at(ids))
        .collect();
    Ok(Puzzle2dSelection {
        nodes: base.nodes.iter().filter(|entry| survivors.contains(&entry.id)).collect(),
        regions: base.target_regions.iter().filter(|entry| regions_apply && survivors.contains(&entry.id)).collect(),
        warnings,
    })
}

/// 📦️ The outcome of a selection leaf's per-member patches: the sparse diff plus the classification warnings, or the
/// `mutation.no-op` warning (after them) when no surviving member changes.
pub fn puzzle2d_selection_outcome(
    selection: Puzzle2dSelection<'_>,
    targets: &PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>,
    nodes: Vec<crate::standards::v1::subsets::any::schema::diff::Puzzle2dNodeModification>,
    regions: Vec<crate::standards::v1::subsets::any::schema::diff::Puzzle2dTargetRegionModification>,
) -> protocol::MutationOutcome<Puzzle2dDiff> {
    use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dNodesDelta, Puzzle2dTargetRegionsDelta};
    if nodes.is_empty() && regions.is_empty() {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages(selection.warnings.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(targets.iter().map(PagedUtf8::to_string_owner).collect::<Vec<_>>())]));
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: (!nodes.is_empty()).then(|| Puzzle2dNodesDelta { modified: nodes, ..Default::default() }),
        target_regions: (!regions.is_empty()).then(|| Puzzle2dTargetRegionsDelta { modified: regions, ..Default::default() }),
        ..Default::default()
    })
    .absorb_messages(selection.warnings)
}

/// 🔄️ A point turned by an angle (`sin`, `cos`) about a pivot.
pub fn puzzle2d_rotated(point: (f64, f64), pivot: (f64, f64), sin: f64, cos: f64) -> (f64, f64) {
    (pivot.0 + (point.0 - pivot.0) * cos - (point.1 - pivot.1) * sin, pivot.1 + (point.0 - pivot.0) * sin + (point.1 - pivot.1) * cos)
}

/// 🔢️ A selection label's number as `(en, de)`: at most two decimals, trailing zeros and a negative zero
/// dropped, a decimal point in English and a decimal comma in German.
pub fn puzzle2d_selection_number(value: f64) -> (String, String) {
    let rounded = (value * 100.0).round() / 100.0;
    let text = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded });
    let en = text.trim_end_matches('0').trim_end_matches('.').to_string();
    let de = en.replace('.', ",");
    (en, de)
}

/// 🔠️ A selection label's counted noun, `(en, de)`: "1 item" / "1 Element", "3 items" / "3 Elemente".
pub fn puzzle2d_selection_items(count: usize) -> (String, String) {
    match count {
        1 => ("1 item".to_string(), "1 Element".to_string()),
        count => (format!("{count} items"), format!("{count} Elemente")),
    }
}
//#endregion 🔖️SelectionTransform

//#region 🔖️Invariants
// 🚨️ Schema-first payload invariants: every value a puzzle 2d leaf payload schema forbids through its hard
// bounds — a non-finite number, a non-positive extent, scale or factor, a shape outside `circle|rectangle`, a
// template rim parameter outside `0..=1`, a negative catalogue order or rank, an empty or repeated target set —
// is refused by the leaf's diff as a Fatal `mutation.invariant` with the default diff (the frozen verb-family
// rule "Fatal non-finite or non-positive"), before the base is consulted.

/// ♾️ Every named number is finite.
pub fn puzzle2d_finite(values: &[(&str, f64)]) -> Result<(), String> {
    values.iter().find(|(_, value)| !value.is_finite()).map_or(Ok(()), |(name, _)| Err(format!("{name} must be a finite number")))
}

/// ➕️ Every present named number is finite and greater than zero.
pub fn puzzle2d_positive(values: &[(&str, Option<f64>)]) -> Result<(), String> {
    values.iter().find(|(_, value)| value.is_some_and(|value| !(value.is_finite() && value > 0.0))).map_or(Ok(()), |(name, _)| Err(format!("{name} must be a finite number greater than 0")))
}

/// 🔷️ A present shape names one of the two figures a node can be.
pub fn puzzle2d_shape(name: &str, shape: Option<&PagedUtf8<{ usize::MAX }>>) -> Result<(), String> {
    match shape {
        Some(shape) if !(shape.eq_str("circle") || shape.eq_str("rectangle")) => Err(format!("{name} must be circle or rectangle, not {shape:?}")),
        _ => Ok(()),
    }
}

/// 🗃️ A selection target set names at least one id and no id twice.
pub fn puzzle2d_targets_invariant(targets: &PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>) -> Result<(), String> {
    if targets.is_empty() {
        return Err("targets must name at least one id".to_string());
    }
    match targets.iter().enumerate().find(|(at, id)| targets.iter().take(*at).any(|previous| previous == *id)) {
        Some((_, id)) => Err(format!("targets must not repeat {id:?}")),
        None => Ok(()),
    }
}

/// 🔘️ A handle record's bounds: a finite angle, a positive radius and scale when present.
pub fn puzzle2d_handle_invariant(handle: &crate::Puzzle2dHandle) -> Result<(), String> {
    puzzle2d_finite(&[("handle angle", handle.angle)]).and_then(|()| puzzle2d_positive(&[("handle radius", handle.radius), ("handle scale", handle.scale)]))
}

/// 🔵️ A node record's bounds: a finite position, a known shape, positive extents and scale, valid handles.
pub fn puzzle2d_node_invariant(node: &crate::Puzzle2dNode) -> Result<(), String> {
    puzzle2d_finite(&[("node x", node.x), ("node y", node.y)])
        .and_then(|()| puzzle2d_shape("node shape", node.shape.as_ref()))
        .and_then(|()| puzzle2d_positive(&[("node radius", node.radius), ("node width", node.width), ("node height", node.height), ("node scale", node.scale)]))
        .and_then(|()| node.handles.iter().try_for_each(puzzle2d_handle_invariant))
}

/// 📐️ A target region's bounds: a finite corner and extent. A negative or zero extent is a brush stroke
/// drawn from any corner, which the schema admits on purpose.
pub fn puzzle2d_region_invariant(region: &crate::Puzzle2dTargetRegion) -> Result<(), String> {
    puzzle2d_finite(&[("region x", region.x), ("region y", region.y), ("region width", region.width), ("region height", region.height)])
}

/// 📚️ Kind catalogue bounds: a handle template's finite angle, positive radius and rim parameter `t` within
/// `0..=1`, a non-negative author rank and a non-negative handle-kind order.
pub fn puzzle2d_catalogs_invariant(catalogs: &crate::Puzzle2dKindCatalogs) -> Result<(), String> {
    for kind in &catalogs.nodes {
        for template in &kind.handles {
            puzzle2d_finite(&[("template angle", template.angle)])?;
            puzzle2d_positive(&[("template radius", template.radius)])?;
            if template.t.is_some_and(|t| !(0.0..=1.0).contains(&t)) {
                return Err(format!("template {:?} rim parameter t must lie within 0..=1", template.id));
            }
        }
        if let Some(author) = kind.authors.iter().find(|author| author.rank.is_some_and(|rank| rank < 0)) {
            return Err(format!("author {:?} rank must not be negative", author.id));
        }
    }
    match catalogs.handles.iter().find(|kind| kind.order.is_some_and(|order| order < 0)) {
        Some(kind) => Err(format!("handle kind {:?} order must not be negative", kind.id)),
        None => Ok(()),
    }
}
//#endregion 🔖️Invariants

//#region 🔖️HandleGeometry
/// 📍️ The board position of the handle `handle_id` on `document`: its node's rim point at the handle's angle, the
/// same rim geometry the board engine draws with and the editor's proximity search measures
/// (`puzzle2d_handle_world_position`) — a circle's east-zero angle on the node's radius, a rectangle's north-zero
/// angle on its outline. `None` when no node carries the handle.
pub fn puzzle2d_handle_position(document: &Puzzle2dSnapshot, handle_id: &PagedUtf8<{ usize::MAX }>) -> Option<(f64, f64)> {
    let (node, handle) = document.nodes.iter().find_map(|node| node.handles.iter().find(|handle| &handle.id == handle_id).map(|handle| (node, handle)))?;
    let centre = semio_framework_geometry::Point::new(node.x, node.y);
    let point = if node.shape.as_ref().is_some_and(|shape| shape.eq_str("rectangle")) {
        semio_framework_graph::drawing::routing::handle_position_on_rectangle(centre, node.width.unwrap_or(48.0), node.height.unwrap_or(48.0), handle.angle)
    } else {
        semio_framework_graph::drawing::routing::handle_position_on_circle(centre, node.radius.unwrap_or(24.0), handle.angle)
    };
    Some((point.x, point.y))
}

/// 📏️ The board distance between the handles `source` and `target` of `document` — what a recorded proximity
/// tolerance is measured against. `None` when either handle is on no node.
pub fn puzzle2d_handle_distance(document: &Puzzle2dSnapshot, source: &PagedUtf8<{ usize::MAX }>, target: &PagedUtf8<{ usize::MAX }>) -> Option<f64> {
    let ((source_x, source_y), (target_x, target_y)) = (puzzle2d_handle_position(document, source)?, puzzle2d_handle_position(document, target)?);
    Some((target_x - source_x).hypot(target_y - source_y))
}
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

#[path = "🏷️literal-id-inverse/🦀️.rs"]
pub mod literal_id_inverse;

#[path = "⚑️flag/🎮️prepare/🦀️.rs"]
pub mod flag_preparation;

#[path = "🔤️text/🎮️prepare/🦀️.rs"]
pub mod text_preparation;

#[path = "🔤️text/↩️inverse/🦀️.rs"]
pub mod text_inverse;

#[path = "🎮️prepare/🧰️child/🦀️.rs"]
pub(crate) mod native_preparation_child;

#[path="🧵️canonical/🦀️.rs"]
mod canonical_fields;
