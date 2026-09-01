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
//! typed projection and still mutate a bare `serde_json::Value` scratch fixture directly (out of
//! scope for this ticket — see `.🦑️repo/🎫️tickets/…/convertpuzzle2d3d5dtotypeddslderiveengine`), so
//! the bridge round-trips through the typed `Puzzle2dSnapshot` (`serde_json::from_value`/
//! `serde_json::to_value`) instead of hand-rolling per-field JSON splicing — the typed
//! `Mutation`/`MutationDiff` impls above are the single source of truth either way.

use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Mutations
/// 🧮️ Semantic puzzle-2d document mutation vocabulary: id-keyed node/edge create-delete plus
/// per-field/per-facet edits (spatial, geometry, presentation flags, handle membership), a
/// handle-to-handle connect/disconnect relationship, and document-meta edits (manifest reference,
/// kind-compatibility connect/disconnect, kind-catalog replace). There is deliberately no camera
/// mutation: the camera is session-only `Puzzle2dPlayRuntime` state in the play app (see
/// `setCamera`'s `ActionKind::View`), never a VCS-tracked document edit. There is deliberately no
/// whole-document mutation: import/reset/example-load goes through `store::ArtifactStore::reset`
/// (non-history), never through this enum.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
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
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Puzzle2dMutation`] variant, in declaration order — the exact
/// vocabulary the `puzzle-2d-1-any` mutation catalog (`../../🧪️oracle/🔣️.json`) declares and
/// the `mutate-puzzle-2d-1` exhaustive case measures itself against. The framework never parses Rust, so
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
pub use super::connect_handles::{connect_handles, ConnectHandles};
pub use super::connect_kind_compatibility::{connect_kind_compatibility, ConnectKindCompatibility};
pub use super::create_node::{create_node, CreateNode};
pub use super::delete_node::{delete_node, DeleteNode};
pub use super::disconnect_handles::{disconnect_handles, DisconnectHandles};
pub use super::disconnect_kind_compatibility::{disconnect_kind_compatibility, DisconnectKindCompatibility};
pub use super::edit_node_text::{edit_node_text, EditNodeText};
pub use super::move_node::{move_node, MoveNode};
pub use super::remove_node_handle::{remove_node_handle, RemoveNodeHandle};
pub use super::replace_edge_geometry::{replace_edge_geometry, ReplaceEdgeGeometry};
pub use super::replace_kind_catalogs::{replace_kind_catalogs, ReplaceKindCatalogs};
pub use super::replace_node_geometry::{replace_node_geometry, ReplaceNodeGeometry};
pub use super::replace_node_handle::{replace_node_handle, ReplaceNodeHandle};
pub use super::scale_node::{scale_node, ScaleNode};

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
    for edge in &after.edges {
        match before.edges.iter().find(|entry| entry.id == edge.id) {
            None => mutations.push(connect_handles(
                edge.id.clone(),
                edge.source.clone(),
                edge.target.clone(),
                edge.edge_kind.clone(),
                edge.gap,
                edge.shift,
                edge.rise,
                edge.rotation,
                edge.turn,
                edge.tilt,
                edge.x,
                edge.y,
                edge.source_tip.clone(),
                edge.target_tip.clone(),
            )),
            Some(prior) if prior.source != edge.source || prior.target != edge.target => {
                mutations.push(disconnect_handles(edge.id.clone()));
                mutations.push(connect_handles(
                    edge.id.clone(),
                    edge.source.clone(),
                    edge.target.clone(),
                    edge.edge_kind.clone(),
                    edge.gap,
                    edge.shift,
                    edge.rise,
                    edge.rotation,
                    edge.turn,
                    edge.tilt,
                    edge.x,
                    edge.y,
                    edge.source_tip.clone(),
                    edge.target_tip.clone(),
                ));
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
            None => mutations.push(connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity)),
            Some(prior) if prior != row => {
                mutations.push(disconnect_kind_compatibility(row.source.clone(), row.target.clone()));
                mutations.push(connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity));
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

pub fn inverse_puzzle2d_mutation(projection: &Puzzle2dSnapshot, mutation: &Puzzle2dMutation) -> Vec<Puzzle2dMutation> {
    mutation.inverse(projection)
}

//#region 🔖️ValueBridge
// 🌉️ `puzzle-plugin`'s scene-mutation helpers predate this typed projection and stay on a bare
// `serde_json::Value` scratch fixture (out of scope for this ticket — see
// `.🦑️repo/🎫️tickets/…/convertpuzzle2d3d5dtotypeddslderiveengine`). Bridging `Puzzle2dMutation`/
// `Puzzle2dDiff` onto that `Value` boundary round-trips through the typed `Puzzle2dSnapshot`
// (`serde_json::from_value`/`to_value`) rather than hand-splicing JSON per mutation kind — the
// typed `Mutation<Puzzle2dSnapshot>`/`MutationDiff<Puzzle2dSnapshot>` impls stay the single source
// of truth, so every one of this enum's 26 kinds gets `Value` support for free.
impl MutationDiff<Value> for Puzzle2dDiff {
    fn apply(&self, projection: &Value) -> protocol::MutationApplyResult<Value> {
        let base: Puzzle2dSnapshot = serde_json::from_value(projection.clone()).map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-base", error.to_string()).at(["document"]))?;
        let next = MutationDiff::<Puzzle2dSnapshot>::apply(self, &base).map_err(|error| error.under(["document"]))?;
        serde_json::to_value(next).map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-result", error.to_string()).at(["document"]))
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle2dSnapshot>::absorb(self, other);
    }
}

impl Mutation<Value> for Puzzle2dMutation {
    type Diff = Puzzle2dDiff;

    /// 🧷️ `#[derive(dsl::Mutations)]` above only generates `impl Mutation<Puzzle2dSnapshot>`
    /// (its declared `#[mutations(snapshot = ...)]`); this hand-written `Value` bridge is a
    /// separate impl of the same trait and forwards to that one rather than duplicating its
    /// 26-entry descriptor table.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle2dSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        <Self as Mutation<Puzzle2dSnapshot>>::descriptor(self)
    }

    fn diff(&self, projection: &Value) -> protocol::MutationOutcome<Puzzle2dDiff> {
        let base: Puzzle2dSnapshot = serde_json::from_value(projection.clone()).unwrap_or_default();
        Mutation::<Puzzle2dSnapshot>::diff(self, &base)
    }

    fn inverse(&self, projection: &Value) -> Vec<Self> {
        let base: Puzzle2dSnapshot = serde_json::from_value(projection.clone()).unwrap_or_default();
        Mutation::<Puzzle2dSnapshot>::inverse(self, &base)
    }
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle2dSnapshot>::may_emit_foreign_steps(self)
    }
}

/// 🧮️ Computes the exact typed semantic mutation sequence turning `before` into `after` (both the
/// bare fixture JSON `puzzle-plugin` mutates), by round-tripping through the typed
/// `Puzzle2dSnapshot` and delegating to [`puzzle2d_snapshot_mutations`]. The camera is deliberately
/// not read here: it is session-only `Puzzle2dPlayRuntime` state (see `setCamera`'s
/// `ActionKind::View`), never persisted on the document, so a fixture must never carry a top-level
/// `"camera"` key at all — `Puzzle2dSnapshot::camera` simply defaults when absent.
pub fn puzzle2d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle2dMutation> {
    if before == after {
        return Vec::new();
    }
    let before_snapshot: Puzzle2dSnapshot = serde_json::from_value(before.clone()).unwrap_or_default();
    let after_snapshot: Puzzle2dSnapshot = serde_json::from_value(after.clone()).unwrap_or_default();
    if before_snapshot == after_snapshot {
        return Vec::new();
    }
    puzzle2d_snapshot_mutations(&before_snapshot, &after_snapshot)
}
//#endregion 🔖️ValueBridge

//#region 🔖️PlaySnapshot
/// 🌱️ The `Puzzle2dPlayApp` predates the typed `Puzzle2dSnapshot` above and stays on this ad-hoc
/// `serde_json::Value` fixture shape for its hundreds of Value-manipulating scene-mutation
/// helpers (see the app's own module docs) — out of scope to retrofit onto the typed struct.
/// This newtype exists only to satisfy `ArtifactApp::Snapshot: store::ArtifactDsl + store::ArtifactPack`
/// post the repo-wide `store::ArtifactDsl for serde_json::Value` bridge's removal (final DSL-syntax
/// convergence gate); `parse_dsl`/`print_dsl`/`encode_pack_with`/`decode_pack_with` all round-trip
/// straight through the still-standing `serde_json::Value` impls (JSON text / JSON-bridge pack
/// encoding respectively), same local-bridge shape as `semio_compose_rs`'s `KitSnapshot`. `Mutation`/
/// `MutationDiff` delegate straight through to the `Value` impls above too.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Puzzle2dPlaySnapshot(pub Value);

impl PartialEq for Puzzle2dPlaySnapshot {
    fn eq(&self, other: &Self) -> bool {
        store::pack_rt::json_values_equal(&self.0, &other.0)
    }
}

impl store::ArtifactDsl for Puzzle2dPlaySnapshot {
    const EXTENSION: &'static str = "puzzle2d-play";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map(Puzzle2dPlaySnapshot).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(&self.0).unwrap_or_default()
    }
}

impl store::ArtifactPack for Puzzle2dPlaySnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(&self.0).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map(Puzzle2dPlaySnapshot).map_err(store::PackError::Schema)
    }
}

impl MutationDiff<Puzzle2dPlaySnapshot> for Puzzle2dDiff {
    fn apply(&self, projection: &Puzzle2dPlaySnapshot) -> protocol::MutationApplyResult<Puzzle2dPlaySnapshot> {
        MutationDiff::<Value>::apply(self, &projection.0).map(Puzzle2dPlaySnapshot)
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle2dSnapshot>::absorb(self, other);
    }
}

impl Mutation<Puzzle2dPlaySnapshot> for Puzzle2dMutation {
    type Diff = Puzzle2dDiff;

    /// 🧷️ Same rationale as the `Mutation<Value>` bridge above: forwards to the
    /// derive-generated `Mutation<Puzzle2dSnapshot>` impl's own descriptors rather than
    /// duplicating them for this second bridging projection.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle2dSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        <Self as Mutation<Puzzle2dSnapshot>>::descriptor(self)
    }

    fn diff(&self, projection: &Puzzle2dPlaySnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        Mutation::<Value>::diff(self, &projection.0)
    }

    fn inverse(&self, projection: &Puzzle2dPlaySnapshot) -> Vec<Puzzle2dMutation> {
        Mutation::<Value>::inverse(self, &projection.0)
    }
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle2dSnapshot>::may_emit_foreign_steps(self)
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
    fn label(&self) -> String {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::label(self)
    }
    fn target(&self) -> Vec<String> {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::target(self)
    }
}
//#endregion 🔖️PlaySnapshot

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle2d::PUZZLE_2D_SCHEMA;
    use protocol::os_spr::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::SemanticMutation;
    use serde_json::json;

    #[test]
    fn puzzle2d_delta_ops_are_granular_and_round_trip() {
        let before = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [{ "id": "n1", "anchor": "fixed", "x": 0.0, "y": 0.0, "handles": [] }, { "id": "n2", "anchor": "fixed", "x": 10.0, "y": 0.0, "handles": [] }], "edges": [] });
        let after = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [{ "id": "n2", "anchor": "fixed", "x": 99.0, "y": 0.0, "handles": [] }, { "id": "n3", "anchor": "fixed", "x": 1.0, "y": 0.0, "handles": [] }], "edges": [] });
        let canonical = |value: &Value| serde_json::to_value(serde_json::from_value::<Puzzle2dSnapshot>(value.clone()).expect("typed puzzle2d fixture")).expect("canonical puzzle2d JSON");
        let operations = puzzle2d_document_delta_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::MoveNode(_))));
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::CreateNode(_))));
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::DeleteNode(_))));
        // The typed bridge canonicalizes optional/default JSON fields while preserving the
        // artifact value and every operation's backwards restores the canonical pre-edit value.
        let mut forward = before.clone();
        let mut inverses = Vec::new();
        for operation in &operations {
            inverses.extend(Mutation::<Value>::inverse(operation, &forward));
            forward = Mutation::<Value>::diff(operation, &forward).diff().apply(&forward).expect("valid mutation diff");
        }
        assert_eq!(forward, canonical(&after));
        for inverse in inverses.iter().rev() {
            forward = Mutation::<Value>::diff(inverse, &forward).diff().apply(&forward).expect("valid mutation diff");
        }
        assert_eq!(forward, canonical(&before), "backwards operations must restore the pre-edit document");
    }

    #[test]
    fn sparse_node_without_anchor_still_emits_create_node() {
        let before = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [], "edges": [] });
        let after = json!({
            "schema": PUZZLE_2D_SCHEMA,
            "nodes": [{ "id": "n1", "nodeKind": "seed", "shape": "circle", "x": 0.0, "y": 0.0, "text": "n1", "handles": [], "radius": 24.0 }],
            "edges": []
        });
        let operations = puzzle2d_document_delta_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dMutation::CreateNode(_))), "sparse add must stay granular");
    }

    //#region 🔖️MutationLaws
    #[test]
    fn create_delete_node_inverse_law() {
        use crate::artifacts::puzzle2d::{schema::empty_puzzle2d_snapshot, Puzzle2dNode};
        let base = empty_puzzle2d_snapshot();
        let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &create_node(node.clone(), None)));
        let with_node = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node, None).diff(&base).diff(), &base).expect("valid mutation diff");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &delete_node("n1".into())));
    }

    #[test]
    fn move_node_inverse_and_absorb_law() {
        use crate::artifacts::puzzle2d::{schema::empty_puzzle2d_snapshot, Puzzle2dNode};
        let base = empty_puzzle2d_snapshot();
        let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
        let with_node = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node, None).diff(&base).diff(), &base).expect("valid mutation diff");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &move_node("n1".into(), 5.0, 6.0)));
        let d1 = move_node("n1".into(), 10.0, 10.0).diff(&with_node).into_parts().0;
        let mid = MutationDiff::<Puzzle2dSnapshot>::apply(&d1, &with_node).expect("valid mutation diff");
        let d2 = move_node("n1".into(), 20.0, 30.0).diff(&mid).into_parts().0;
        semio_framework::io::resolve_ready(assert_mutation_diff_absorb_law(&with_node, d1, d2));
    }

    #[test]
    fn node_field_mutations_inverse_law() {
        use crate::artifacts::puzzle2d::{schema::empty_puzzle2d_snapshot, Puzzle2dHandle, Puzzle2dNode, Puzzle2dNodeAnchor};
        let base = empty_puzzle2d_snapshot();
        let node = Puzzle2dNode { id: "n1".into(), handles: vec![Puzzle2dHandle { id: "h1".into(), ..Default::default() }], ..Default::default() };
        let with_node = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node, None).diff(&base).diff(), &base).expect("valid mutation diff");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &replace_node_geometry("n1".into(), Some("rectangle".into()), None, Some(4.0), Some(2.0))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_kind("n1".into(), Some("core.capsule".into()))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &edit_node_text("n1".into(), Some("hello".into()))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_icon("n1".into(), Some("star".into()))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &scale_node("n1".into(), Some(2.0))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_visible("n1".into(), Some(false))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_locked("n1".into(), Some(true))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_root("n1".into(), Some(true))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &change_node_anchor("n1".into(), Puzzle2dNodeAnchor::Derived)));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &add_node_handle("n1".into(), Puzzle2dHandle { id: "h2".into(), ..Default::default() }, None)));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &remove_node_handle("n1".into(), "h1".into())));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&with_node, &replace_node_handle("n1".into(), "h1".into(), Puzzle2dHandle { id: "h1".into(), angle: 1.5, ..Default::default() })));
    }

    #[test]
    fn connect_disconnect_handles_inverse_law() {
        use crate::artifacts::puzzle2d::{schema::empty_puzzle2d_snapshot, Puzzle2dHandle, Puzzle2dNode};
        let base = empty_puzzle2d_snapshot();
        let node_a = Puzzle2dNode { id: "a".into(), handles: vec![Puzzle2dHandle { id: "ha".into(), ..Default::default() }], ..Default::default() };
        let node_b = Puzzle2dNode { id: "b".into(), handles: vec![Puzzle2dHandle { id: "hb".into(), ..Default::default() }], ..Default::default() };
        let mut projection = base.clone();
        projection = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node_a, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
        projection = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node_b, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&projection, &connect_handles("e1".into(), "ha".into(), "hb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None)));
        let connected = MutationDiff::<Puzzle2dSnapshot>::apply(connect_handles("e1".into(), "ha".into(), "hb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &disconnect_handles("e1".into())));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &replace_edge_geometry("e1".into(), 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0)));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &change_edge_kind("e1".into(), Some("core.link".into()))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &change_edge_tips("e1".into(), Some("arrow".into()), Some("dot".into()))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &change_edge_visible("e1".into(), Some(false))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &change_edge_locked("e1".into(), Some(true))));
    }

    #[test]
    fn delete_node_severs_and_reconnects_edges() {
        use crate::artifacts::puzzle2d::{schema::empty_puzzle2d_snapshot, Puzzle2dHandle, Puzzle2dNode};
        let base = empty_puzzle2d_snapshot();
        let node_a = Puzzle2dNode { id: "a".into(), handles: vec![Puzzle2dHandle { id: "ha".into(), ..Default::default() }], ..Default::default() };
        let node_b = Puzzle2dNode { id: "b".into(), handles: vec![Puzzle2dHandle { id: "hb".into(), ..Default::default() }], ..Default::default() };
        let mut projection = base;
        projection = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node_a, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
        projection = MutationDiff::<Puzzle2dSnapshot>::apply(create_node(node_b, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
        projection = MutationDiff::<Puzzle2dSnapshot>::apply(connect_handles("e1".into(), "ha".into(), "hb".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None).diff(&projection).diff(), &projection).expect("valid mutation diff");
        assert!(projection.edges.iter().any(|edge| edge.id == "e1"));
        let removed = delete_node("a".into());
        let after = MutationDiff::<Puzzle2dSnapshot>::apply(removed.diff(&projection).diff(), &projection).expect("valid mutation diff");
        assert!(!after.edges.iter().any(|edge| edge.id == "e1"), "delete-node must sever edges touching its handles");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&projection, &removed));
    }

    #[test]
    fn meta_mutations_inverse_law() {
        use crate::artifacts::puzzle2d::{schema::empty_puzzle2d_snapshot, Puzzle2dCompatSpecificity, Puzzle2dKindCatalogs};
        let base = empty_puzzle2d_snapshot();
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &change_manifest_id(Some("manifest-1".into()))));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle2dCompatSpecificity::Handle)));
        let connected = MutationDiff::<Puzzle2dSnapshot>::apply(connect_kind_compatibility("a".into(), "b".into(), true, false, Puzzle2dCompatSpecificity::Handle).diff(&base).diff(), &base).expect("valid mutation diff");
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&connected, &disconnect_kind_compatibility("a".into(), "b".into())));
        semio_framework::io::resolve_ready(assert_mutation_inverse_law(&base, &replace_kind_catalogs(Some(Puzzle2dKindCatalogs::default()))));
    }

    #[test]
    fn dispatch_registers_semantic_descriptors() {
        register_puzzle2d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
        for kind in <Puzzle2dMutation as protocol::SemanticMutation<Puzzle2dSnapshot>>::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(<Puzzle2dMutation as protocol::SemanticMutation<Puzzle2dSnapshot>>::kinds().len(), 26);
    }
    //#endregion 🔖️MutationLaws

    //#region 🔖️OutcomeLaws
    // 🎫️ 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — see
    // `📓️w3-f-block-puzzle-report.md` for the `assert_outcome_policy_matrix` pending-helper note.
    use protocol::os_spr::testkit::{assert_fatal_never_applies, assert_missing_target_is_error};

    #[test]
    fn missing_target_is_error_per_verb_family() {
        use crate::artifacts::puzzle2d::schema::empty_puzzle2d_snapshot;
        let base = empty_puzzle2d_snapshot();
        semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &delete_node("missing".into()))); // delete
        semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &remove_node_handle("missing".into(), "h0".into()))); // remove
        semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &change_node_visible("missing".into(), Some(false)))); // change/set/update
        semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &move_node("missing".into(), 1.0, 1.0))); // move/drag/rotate/scale/resize
        semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &edit_node_text("missing".into(), Some("x".into())))); // edit/replace
        semio_framework::io::resolve_ready(assert_missing_target_is_error(&base, &disconnect_handles("missing".into())));
        // disconnect/unbind
    }

    #[test]
    fn create_duplicate_id_is_fatal_and_never_applies() {
        use crate::artifacts::puzzle2d::{schema::empty_puzzle2d_snapshot, Puzzle2dNode};
        let mut base = empty_puzzle2d_snapshot();
        let node = Puzzle2dNode { id: "n0".into(), ..Default::default() };
        base.nodes.push(node.clone());
        let outcome = create_node(node, None).diff(&base);
        semio_framework::io::resolve_ready(assert_fatal_never_applies(&outcome));
        assert_eq!(outcome.worst_level(), Some(dsl::Severity::Fatal));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.duplicate-id"));
    }
    //#endregion 🔖️OutcomeLaws

    //#region 🧪️KindsCatalog
    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed oracle
    /// manifest's catalog — the framework never parses Rust, so this is the only thing that keeps the
    /// declared vocabulary and the measured one from drifting apart.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <Puzzle2dMutation as protocol::SemanticMutation<Puzzle2dSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared Puzzle2dMutation variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
    //#endregion 🧪️KindsCatalog
}
//#endregion 🧪️Tests
