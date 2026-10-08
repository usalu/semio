//! 🧬️ Puzzle 2d artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<Puzzle2dSnapshot>` and `impl protocol::SemanticMutation<Puzzle2dSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here. `dsl::DslEnum` supplies
//! `DslVariants` (keyed off each payload's own `#[dsl(keyword = ...)]`), consumed by `OpText`/
//! `OpBinary` in the sibling `📝️text`/`💾️binary` modules.
//!
//! The `serde_json::Value` bridge (`🔖️ValueBridge`) and the play app's `Puzzle2dPlaySnapshot`
//! newtype (`🔖️PlaySnapshot`) live here too: `puzzle-plugin`'s scene-mutation helpers predate this
//! typed projection and still mutate a bare `serde_json::Value` scratch snapshot directly (out of
//! scope for this ticket — see `.🧬semio/🦑️repo/🎫️tickets/…/convertpuzzle2d3d5dtotypeddslderiveengine`), so
//! the bridge round-trips through the typed `Puzzle2dSnapshot` (`serde_json::from_value`/
//! `serde_json::to_value`) instead of hand-rolling per-field JSON splicing — the typed
//! `Mutation`/`MutationDiff` impls above are the single source of truth either way.

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;
use protocol::{DiffAlgebra, Mutation, MutationDiff};
use serde_json::Value;
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned)]
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
    nodes: Vec<crate::standards::v1::subsets::any::schema::diff::Puzzle2dNodePatchEntry>,
    regions: Vec<crate::standards::v1::subsets::any::schema::diff::Puzzle2dTargetRegionPatchEntry>,
) -> protocol::MutationOutcome<Puzzle2dDiff> {
    use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dNodesDelta, Puzzle2dTargetRegionsDelta};
    if nodes.is_empty() && regions.is_empty() {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages(selection.warnings.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(targets.iter().map(PagedUtf8::to_string_owner).collect::<Vec<_>>())]));
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: (!nodes.is_empty()).then(|| Puzzle2dNodesDelta { patched: nodes, ..Default::default() }),
        target_regions: (!regions.is_empty()).then(|| Puzzle2dTargetRegionsDelta { patched: regions, ..Default::default() }),
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

//#region 🔖️SnapshotDelta
/// 🔀️ Diffs two typed snapshots into a minimal semantic mutation set — the single source of truth
/// both the VCS layer and the `serde_json::Value` scene bridge below replay through.
pub fn puzzle2d_snapshot_mutations(before: &Puzzle2dSnapshot, after: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let mut mutations = Vec::new();
    for node in &before.nodes {
        if !after.nodes.iter().any(|entry| entry.id == node.id) {
            mutations.push(delete_node(node.id.clone()));
        }
    }
    for node in &after.nodes {
        match before.nodes.iter().find(|entry| entry.id == node.id) {
            None => mutations.push(create_node(node.clone(), None)),
            Some(prior) => {
                if prior.x != node.x || prior.y != node.y {
                    mutations.push(move_node(node.id.clone(), node.x, node.y));
                }
                if prior.shape != node.shape || prior.radius != node.radius || prior.width != node.width || prior.height != node.height {
                    mutations.push(replace_node_geometry(node.id.clone(), node.shape.clone(), node.radius, node.width, node.height));
                }
                if prior.node_kind != node.node_kind {
                    mutations.push(change_node_kind(node.id.clone(), node.node_kind.clone()));
                }
                if prior.text != node.text {
                    mutations.push(edit_node_text(node.id.clone(), node.text.clone()));
                }
                if prior.icon_kind != node.icon_kind {
                    mutations.push(change_node_icon(node.id.clone(), node.icon_kind.clone()));
                }
                if prior.scale != node.scale {
                    mutations.push(scale_node(node.id.clone(), node.scale));
                }
                if prior.visible != node.visible {
                    mutations.push(change_node_visible(node.id.clone(), node.visible));
                }
                if prior.locked != node.locked {
                    mutations.push(change_node_locked(node.id.clone(), node.locked));
                }
                if prior.root != node.root {
                    mutations.push(change_node_root(node.id.clone(), node.root));
                }
                if prior.anchor != node.anchor {
                    mutations.push(change_node_anchor(node.id.clone(), node.anchor));
                }
                for handle in &prior.handles {
                    if !node.handles.iter().any(|entry| entry.id == handle.id) {
                        mutations.push(remove_node_handle(node.id.clone(), handle.id.clone()));
                    }
                }
                for handle in &node.handles {
                    match prior.handles.iter().find(|entry| entry.id == handle.id) {
                        None => mutations.push(add_node_handle(node.id.clone(), handle.clone(), None)),
                        Some(prior_handle) if prior_handle != handle => mutations.push(replace_node_handle(node.id.clone(), handle.id.clone(), handle.clone())),
                        Some(_) => {}
                    }
                }
            }
        }
    }
    for edge in &before.edges {
        if !after.edges.iter().any(|entry| entry.id == edge.id) {
            mutations.push(disconnect_handles(edge.id.clone()));
        }
    }
    for (index, edge) in after.edges.iter().enumerate() {
        match before.edges.iter().find(|entry| entry.id == edge.id) {
            None => super::connect_handles::restore_edge(edge, index, &mut mutations),
            Some(prior) if prior.source != edge.source || prior.target != edge.target => {
                mutations.push(disconnect_handles(edge.id.clone()));
                super::connect_handles::restore_edge(edge, index, &mut mutations);
            }
            Some(prior) => {
                if prior.gap != edge.gap || prior.shift != edge.shift || prior.rise != edge.rise || prior.rotation != edge.rotation || prior.turn != edge.turn || prior.tilt != edge.tilt || prior.x != edge.x || prior.y != edge.y {
                    mutations.push(replace_edge_geometry(edge.id.clone(), edge.gap, edge.shift, edge.rise, edge.rotation, edge.turn, edge.tilt, edge.x, edge.y));
                }
                if prior.edge_kind != edge.edge_kind {
                    mutations.push(change_edge_kind(edge.id.clone(), edge.edge_kind.clone()));
                }
                if prior.source_tip != edge.source_tip || prior.target_tip != edge.target_tip {
                    mutations.push(change_edge_tips(edge.id.clone(), edge.source_tip.clone(), edge.target_tip.clone()));
                }
                if prior.visible != edge.visible {
                    mutations.push(change_edge_visible(edge.id.clone(), edge.visible));
                }
                if prior.locked != edge.locked {
                    mutations.push(change_edge_locked(edge.id.clone(), edge.locked));
                }
            }
        }
    }
    for region in &before.target_regions {
        if !after.target_regions.iter().any(|entry| entry.id == region.id) {
            mutations.push(delete_target_region(region.id.clone()));
        }
    }
    for region in &after.target_regions {
        match before.target_regions.iter().find(|entry| entry.id == region.id) {
            None => mutations.push(create_target_region(region.clone(), None)),
            Some(prior) => {
                if prior.x != region.x || prior.y != region.y {
                    mutations.push(move_target_region(region.id.clone(), region.x, region.y));
                }
                if prior.width != region.width || prior.height != region.height {
                    mutations.push(resize_target_region(region.id.clone(), region.width, region.height));
                }
                if prior.label != region.label {
                    mutations.push(edit_target_region_label(region.id.clone(), region.label.clone()));
                }
                if prior.hidden != region.hidden {
                    mutations.push(change_target_region_hidden(region.id.clone(), region.hidden));
                }
                if prior.locked != region.locked {
                    mutations.push(change_target_region_locked(region.id.clone(), region.locked));
                }
            }
        }
    }
    if before.meta.manifest_id != after.meta.manifest_id {
        mutations.push(change_manifest_id(after.meta.manifest_id.clone()));
    }
    for row in &before.meta.kind_compatibility {
        if !after.meta.kind_compatibility.iter().any(|entry| entry.source == row.source && entry.target == row.target) {
            mutations.push(disconnect_kind_compatibility(row.source.clone(), row.target.clone()));
        }
    }
    for row in &after.meta.kind_compatibility {
        match before.meta.kind_compatibility.iter().find(|entry| entry.source == row.source && entry.target == row.target) {
            None => mutations.push(connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity, None)),
            Some(prior) if prior != row => {
                mutations.push(disconnect_kind_compatibility(row.source.clone(), row.target.clone()));
                mutations.push(connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity, None));
            }
            Some(_) => {}
        }
    }
    if before.meta.kind_catalogs != after.meta.kind_catalogs {
        mutations.push(replace_kind_catalogs(after.meta.kind_catalogs.clone()));
    }
    mutations
}
//#endregion 🔖️SnapshotDelta

/// ▶️ Applies `mutation` via its diff.
pub fn apply_puzzle2d_mutation(projection: &mut Puzzle2dSnapshot, mutation: &Puzzle2dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;

    *projection = next;
    Ok(())
}

pub fn inverse_puzzle2d_mutation(projection: &Puzzle2dSnapshot, mutation: &Puzzle2dMutation) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}

//#region 🔖️ValueBridge
// 🌉️ `puzzle-plugin`'s scene-mutation helpers predate this typed projection and stay on a bare
// `serde_json::Value` scratch snapshot (out of scope for this ticket — see
// `.🧬semio/🦑️repo/🎫️tickets/…/convertpuzzle2d3d5dtotypeddslderiveengine`). Bridging `Puzzle2dMutation`/
// `Puzzle2dDiff` onto that `Value` boundary round-trips through the typed `Puzzle2dSnapshot`
// (`serde_json::from_value`/`to_value`) rather than hand-splicing JSON per mutation kind — the
// typed `Mutation<Puzzle2dSnapshot>`/`MutationDiff<Puzzle2dSnapshot>` impls stay the single source
// of truth, so every one of this enum's 33 kinds gets `Value` support for free.
impl MutationDiff<Value> for Puzzle2dDiff {
    fn apply(&self, projection: &Value, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Value> {
        // 🩹️ Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS: routes
        // through `dsl::DslValue`/`dsl::ToValue`/`dsl::FromValue` instead of
        // `serde_json::from_value`/`to_value` on `Puzzle2dSnapshot` directly — that type only
        // derives `Serialize`/`Deserialize` under `#[cfg(test)]` now. `Value` (this bridge's own
        // boundary type) is untouched.
        let base: Puzzle2dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(projection)).map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-base", error.to_string()).at(["document"]))?;
        let next = MutationDiff::<Puzzle2dSnapshot>::apply(self, &base, capability).map_err(|error| error.under(["document"]))?;
        Ok(Value::from(semio_framework_value::ToValue::to_value(&next)))
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle2dSnapshot>::absorb(self, other);
    }
}

impl DiffAlgebra<Value> for Puzzle2dDiff {
    fn inverse(&self, base: &Value) -> Self {
        let base: Puzzle2dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(base)).unwrap_or_default();
        DiffAlgebra::<Puzzle2dSnapshot>::inverse(self, &base)
    }
    fn between(base: &Value, other: &Value) -> Self {
        let base: Puzzle2dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(base)).unwrap_or_default();
        let other: Puzzle2dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(other)).unwrap_or_default();
        <Self as DiffAlgebra<Puzzle2dSnapshot>>::between(&base, &other)
    }
    fn is_empty(&self) -> bool {
        DiffAlgebra::<Puzzle2dSnapshot>::is_empty(self)
    }
}

impl Mutation<Value> for Puzzle2dMutation {
    type Diff = Puzzle2dDiff;

    /// 🧷️ `#[derive(dsl::Mutations)]` above only generates `impl Mutation<Puzzle2dSnapshot>`
    /// (its declared `#[mutations(snapshot = ...)]`); this hand-written `Value` bridge is a
    /// separate impl of the same trait and forwards to that one rather than duplicating its
    /// 33-entry descriptor table.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle2dSnapshot>>::DESCRIPTORS;
    const INPUT_SCHEMAS: &'static [&'static str] = <Self as Mutation<Puzzle2dSnapshot>>::INPUT_SCHEMAS;
    const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = <Self as Mutation<Puzzle2dSnapshot>>::INPUT_SCHEMA_DOCUMENTS;

    fn input_schema(&self) -> Option<&'static str> {
        Mutation::<Puzzle2dSnapshot>::input_schema(self)
    }

    fn payload_value(&self) -> semio_framework_value::DslValue {
        Mutation::<Puzzle2dSnapshot>::payload_value(self)
    }

    fn with_payload_value(&self, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Mutation::<Puzzle2dSnapshot>::with_payload_value(self, value)
    }

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        <Self as Mutation<Puzzle2dSnapshot>>::descriptor(self)
    }

    fn inverse_rows(&self) -> usize {
        Mutation::<Puzzle2dSnapshot>::inverse_rows(self)
    }

    fn diff(&self, projection: &Value) -> protocol::MutationOutcome<Puzzle2dDiff> {
        let base: Puzzle2dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(projection)).unwrap_or_default();
        Mutation::<Puzzle2dSnapshot>::diff(self, &base)
    }

    fn inverse(&self, projection: &Value) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok({
        let base: Puzzle2dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(projection)).unwrap_or_default();
        Mutation::<Puzzle2dSnapshot>::inverse(self, &base)?
    
    })
}
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle2dSnapshot>::may_emit_foreign_steps(self)
    }
    fn from_payload_value(kind: &str, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Self as Mutation<Puzzle2dSnapshot>>::from_payload_value(kind, value)
    }
    fn conflict_target(&self) -> Vec<String> {
        Mutation::<Puzzle2dSnapshot>::conflict_target(self)
    }
}

/// 🧮️ Computes the exact typed semantic mutation sequence turning `before` into `after` (both the
/// bare snapshot JSON `puzzle-plugin` mutates), by round-tripping through the typed
/// `Puzzle2dSnapshot` and delegating to [`puzzle2d_snapshot_mutations`]. The camera is deliberately
/// not read here: it is session-only `Puzzle2dPlayRuntime` state (see `setCamera`'s
/// `ActionKind::View`), never persisted on the document, so a snapshot must never carry a top-level
/// `"camera"` key at all — `Puzzle2dSnapshot::camera` simply defaults when absent.
///
/// 🐛️ A side that does not decode is an ERROR, never an empty document: this used to
/// `unwrap_or_default()` both sides, so one malformed row in `after` (a brush-placed node whose handles
/// carried no `id`) decoded as the EMPTY board and the delta deleted every node, the manifest and every
/// compatibility row of the real document — `acceptSuggestion` on concrete-forest committed exactly that
/// (measured 2026-09-23, `📓️block-puzzle.md` §11).
pub fn puzzle2d_document_delta_operations(before: &Value, after: &Value) -> Result<Vec<Puzzle2dMutation>, String> {
    if before == after {
        return Ok(Vec::new());
    }
    let before_snapshot: Puzzle2dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(before)).map_err(|error| format!("puzzle2d delta base does not decode: {error}"))?;
    let after_snapshot: Puzzle2dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(after)).map_err(|error| format!("puzzle2d delta result does not decode: {error}"))?;
    if before_snapshot == after_snapshot {
        return Ok(Vec::new());
    }
    Ok(puzzle2d_snapshot_mutations(&before_snapshot, &after_snapshot))
}
//#endregion 🔖️ValueBridge

//#region 🔖️PlaySnapshot
/// 🌱️ The `Puzzle2dPlayApp` scene helpers still read the ad-hoc `serde_json::Value` snapshot shape, while every
/// Store mutation is TYPED. This snapshot keeps the typed `Puzzle2dSnapshot` as the one authority and
/// materializes the legacy `Value` projection lazily, at most once per immutable root — the shape `🧊️3d`'s
/// `Puzzle3dPlaySnapshot` and `🖐️5d`'s `Puzzle5dPlaySnapshot` have, kept identical on purpose.
///
/// 🐛️ It used to BE the `Value`, so every one-item Store preparation decoded the whole document into
/// `Puzzle2dSnapshot` for `inverse`, again for `diff`, again inside `apply`, and re-encoded the post root:
/// O(document) per folded mutation, O(n²) per example switch. Measured on 5d's twin bridge (capsule-dream,
/// `📓️block-puzzle.md` §11.6); 2d's bridge was byte-for-byte the same.
#[derive(Debug)]
pub struct Puzzle2dPlaySnapshot {
    typed: std::sync::Arc<Puzzle2dSnapshot>,
    value: std::sync::OnceLock<std::sync::Arc<Value>>,
}

impl Puzzle2dPlaySnapshot {
    /// 🎯️ Builds the typed authority once from a legacy projection and retains that projection.
    pub fn new(value: Value) -> Self {
        let typed = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&value)).unwrap_or_default();
        let projected = std::sync::OnceLock::new();
        let _ = projected.set(std::sync::Arc::new(value));
        Self { typed: std::sync::Arc::new(typed), value: projected }
    }

    /// 🧬️ A root produced by typed mutation application; its `Value` projection is deferred.
    pub(crate) fn from_typed(typed: Puzzle2dSnapshot) -> Self {
        Self { typed: std::sync::Arc::new(typed), value: std::sync::OnceLock::new() }
    }

    /// 👁️ The legacy play projection, materialized at most once per immutable snapshot.
    pub fn value(&self) -> &Value {
        self.value.get_or_init(|| std::sync::Arc::new(Value::from(semio_framework_value::ToValue::to_value(self.typed.as_ref())))).as_ref()
    }

    /// 🤝️ The legacy play projection as a shared root — what an owned tool event carries without copying the document.
    pub fn shared_value(&self) -> std::sync::Arc<Value> {
        std::sync::Arc::clone(self.value.get_or_init(|| std::sync::Arc::new(Value::from(semio_framework_value::ToValue::to_value(self.typed.as_ref())))))
    }

    /// 🫱️ The typed authority as a shared root, for the same reason.
    pub fn shared_typed(&self) -> std::sync::Arc<Puzzle2dSnapshot> {
        std::sync::Arc::clone(&self.typed)
    }

    /// 🧬️ The typed authority, without materializing the legacy projection.
    pub fn typed(&self) -> &Puzzle2dSnapshot {
        self.typed.as_ref()
    }
}

impl Clone for Puzzle2dPlaySnapshot {
    fn clone(&self) -> Self {
        let value = std::sync::OnceLock::new();
        if let Some(projected) = self.value.get() {
            let _ = value.set(std::sync::Arc::clone(projected));
        }
        Self { typed: std::sync::Arc::clone(&self.typed), value }
    }
}

impl PartialEq for Puzzle2dPlaySnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.typed == other.typed
    }
}

/// 🩹️ Hand-written: `ArtifactEditor::Snapshot` needs `ToValue + FromValue`, and this struct's typed/lazy
/// split has no field-wise derive shape, so both bridge through the `Value` projection `value()`/`new()`
/// maintain.
impl semio_framework_value::ToValue for Puzzle2dPlaySnapshot {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::from(self.value())
    }
}

impl semio_framework_value::FromValue for Puzzle2dPlaySnapshot {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Puzzle2dSnapshot as semio_framework_value::FromValue>::from_value(value).map(Self::from_typed)
    }
}



/// 🧒️ Composition view of the play snapshot: a puzzle 2d document owns no child artifacts.
impl semio_framework_schema_composition::ArtifactCompositionFields for Puzzle2dPlaySnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}



impl MutationDiff<Puzzle2dPlaySnapshot> for Puzzle2dDiff {
    fn apply(&self, projection: &Puzzle2dPlaySnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle2dPlaySnapshot> {
        MutationDiff::<Puzzle2dSnapshot>::apply(self, projection.typed(), capability).map(Puzzle2dPlaySnapshot::from_typed).map_err(|error| error.under(["document"]))
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle2dSnapshot>::absorb(self, other);
    }
}

impl DiffAlgebra<Puzzle2dPlaySnapshot> for Puzzle2dDiff {
    fn inverse(&self, base: &Puzzle2dPlaySnapshot) -> Self {
        DiffAlgebra::<Puzzle2dSnapshot>::inverse(self, base.typed())
    }
    fn between(base: &Puzzle2dPlaySnapshot, other: &Puzzle2dPlaySnapshot) -> Self {
        <Self as DiffAlgebra<Puzzle2dSnapshot>>::between(base.typed(), other.typed())
    }
    fn is_empty(&self) -> bool {
        DiffAlgebra::<Puzzle2dSnapshot>::is_empty(self)
    }
}

impl Mutation<Puzzle2dPlaySnapshot> for Puzzle2dMutation {
    type Diff = Puzzle2dDiff;

    /// 🧷️ Not hand-written — see the identical note on `impl Mutation<Value>` above. The metadata
    /// is projection-independent, so this forwards to the derive's own table too, same as
    /// `may_emit_foreign_steps` already does immediately below.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle2dSnapshot>>::DESCRIPTORS;
    const INPUT_SCHEMAS: &'static [&'static str] = <Self as Mutation<Puzzle2dSnapshot>>::INPUT_SCHEMAS;
    const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = <Self as Mutation<Puzzle2dSnapshot>>::INPUT_SCHEMA_DOCUMENTS;

    fn input_schema(&self) -> Option<&'static str> {
        Mutation::<Puzzle2dSnapshot>::input_schema(self)
    }

    fn payload_value(&self) -> semio_framework_value::DslValue {
        Mutation::<Puzzle2dSnapshot>::payload_value(self)
    }

    fn with_payload_value(&self, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Mutation::<Puzzle2dSnapshot>::with_payload_value(self, value)
    }

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        <Self as Mutation<Puzzle2dSnapshot>>::descriptor(self)
    }

    fn inverse_rows(&self) -> usize {
        Mutation::<Puzzle2dSnapshot>::inverse_rows(self)
    }

    fn diff(&self, projection: &Puzzle2dPlaySnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        Mutation::<Puzzle2dSnapshot>::diff(self, projection.typed())
    }

    fn inverse(&self, projection: &Puzzle2dPlaySnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        Mutation::<Puzzle2dSnapshot>::inverse(self, projection.typed())?
    
    })
}
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle2dSnapshot>::may_emit_foreign_steps(self)
    }
    fn from_payload_value(kind: &str, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Self as Mutation<Puzzle2dSnapshot>>::from_payload_value(kind, value)
    }
    fn conflict_target(&self) -> Vec<String> {
        Mutation::<Puzzle2dSnapshot>::conflict_target(self)
    }
}

/// 🪪️ `kinds`/`semantics`/`label`/`target` are projection-independent (the derive-generated
/// `SemanticMutation<Puzzle2dSnapshot>` impl above never actually reads `Puzzle2dSnapshot` data in
/// any of the four), so this bridges the same vocabulary onto `Puzzle2dPlaySnapshot` by forwarding
/// straight through — the `SemanticMutation` twin of the `Mutation<Puzzle2dPlaySnapshot>` bridge
/// immediately above, needed so `.editor_mutation_roster::<Puzzle2dPlayApp>()` can register this
/// dialect's real semantic vocabulary against the play app's own `Snapshot` type.
impl protocol::SemanticMutation<Puzzle2dPlaySnapshot> for Puzzle2dMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::kinds()
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::semantics(self)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::label(self)
    }
    fn target(&self) -> Vec<String> {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::target(self)
    }
}
//#endregion 🔖️PlaySnapshot

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
