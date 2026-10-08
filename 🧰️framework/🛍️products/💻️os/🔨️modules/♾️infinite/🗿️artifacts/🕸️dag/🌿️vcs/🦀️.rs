//! 🧬️ DAG mutation, diff, and document codecs.
use crate::snapshot::*;
use ::graph::manifest::PropertyBag;
#[cfg(test)]
use ::graph::manifest::PropertyValue;
#[cfg(test)]
use semio_framework_pack_json::Value;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::HashSet;

// #region 🔖️ArtifactVcs
#[cfg(test)]
use crate::os_spr::Mutation;
#[cfg(test)]
use crate::os_spr::{ArtifactId, Edit, SchemaId};
pub use crate::os_spr::ActorId;
use crate::os_spr::{ApplyCapability, DiffAlgebra, Identified, MutationDiff, Patchable};
use crate::os_store::create_document_envelope;
#[cfg(test)]
use crate::os_store::ArtifactCommand;
use crate::os_store::{ArtifactEnvelope, ArtifactStore};

pub const DAG_DOCUMENT_SCHEMA: &str = "dag.host_snapshot";

fn dag_artifact_schema() -> String {
    DAG_DOCUMENT_SCHEMA.into()
}

/// 🧾️ The persistent DAG projection — nodes and edges only. Camera/viewport and selection are
/// ephemeral view state kept in the plugin runtime, never recorded in the document's undo history.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(layout="lines")]
#[artifact(id="dag.dag")]
pub struct DagSnapshot {
    #[value(default = "dag_artifact_schema")]
    pub schema: String,
    #[value(default)]
    pub nodes: Vec<DagNodeSpec>,
    #[value(default)]
    #[dsl(table)]
    pub edges: Vec<DagHostSnapshotEdge>,
}

pub fn empty_dag_document() -> DagSnapshot {
    DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() }
}

/// 🌱️ The demo document seed (nodes/edges from `🕸️demo.dag`), sharing the fixture's example.
pub fn default_dag_document() -> DagSnapshot {
    dag_document_from_host_snapshot(&DagHostSnapshot::default())
}

/// 🔁️ Projects a {@link DagHostSnapshot} (which also carries a camera) down to the persistent {@link DagSnapshot}.
pub fn dag_document_from_host_snapshot(host_snapshot: &DagHostSnapshot) -> DagSnapshot {
    DagSnapshot { schema: host_snapshot.schema.clone(), nodes: host_snapshot.nodes.clone(), edges: host_snapshot.edges.clone() }
}

/// 🔁️ Rehydrates a full {@link DagHostSnapshot} from a {@link DagSnapshot} plus a runtime `camera`, so the
/// existing fixture-shaped helpers (`dag_host_snapshot_to_wire_literal`, `DagHost`, …) can be reused.
pub fn dag_host_snapshot_from_document(document: &DagSnapshot, camera: DagCamera) -> DagHostSnapshot {
    DagHostSnapshot { schema: document.schema.clone(), camera, nodes: document.nodes.clone(), edges: document.edges.clone() }
}

//#region 🔖️ExternalPatchSupport
// 🧬️ `DagNodePatch`/`DagEdgePatch` (+ the `Identified`/`Patchable` impls that make them usable) are no
// longer consumed by THIS file's own `DagMutation`/`DagDiff` (see `🔖️DiffDeltas` below for the
// dedicated per-verb delta types that replaced them here) — but they are re-exported verbatim
// (`pub use infinite_board_port_directed_dag::{DagEdgePatch, DagNodePatch, ...}`,
// `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️.rs:9-11`) and directly consumed as a live,
// load-bearing dependency by that plugin's own already-landed `🧬️mutations` facet: its
// `apply_nodes_delta`/`apply_edges_delta`/`apply_identified_delta` helpers
// (`…/🚪️io/📝️text/🔺️diff/🦀️.rs`) are generic over `T: Identified<String> + Patchable<P>`
// and call `.apply_patch(...)` on `DagNodeSpec`/`DagHostSnapshotEdge` using exactly these impls. Deleting
// them would compile-break that plugin's facet, which is the same "boundary that separates a
// definition from its registration is a race" failure this ticket's own doctrine warns against —
// so this file keeps them, unrelated to and independent of the `CollectionMutation<TId,TItem,TPatch>`
// elimination below (this crate's own `DagMutation` never wraps them in that banned generic type).
impl Identified<String> for DagNodeSpec {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for DagHostSnapshotEdge {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹️ Sparse patch of a {@link DagNodeSpec} — layout fields plus a whole-`kind` replacement for
/// kind-specific edits (slider value/min/max, note text, …). Consumed by `✏️s/🔌️plugins/🕸️dag`'s own
/// `DagNodeExtraPatch` deviation (fields this type has no slot for: `id`/`icon`/`abbreviation`/
/// `operator_kind`/`properties` — that plugin's own workaround, not extended here on their behalf).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DagNodePatch {
    #[value(default)]
    pub name: Option<String>,
    #[value(default)]
    pub x: Option<f64>,
    #[value(default)]
    pub y: Option<f64>,
    #[value(default)]
    pub width: Option<f64>,
    #[value(default)]
    pub height: Option<f64>,
    #[value(default)]
    pub kind: Option<DagNodeKind>,
}

impl Patchable<DagNodePatch> for DagNodeSpec {
    fn apply_patch(&mut self, patch: &DagNodePatch) {
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(x) = patch.x {
            self.x = x;
        }
        if let Some(y) = patch.y {
            self.y = y;
        }
        if let Some(width) = patch.width {
            self.width = width;
        }
        if let Some(height) = patch.height {
            self.height = height;
        }
        if let Some(kind) = &patch.kind {
            self.kind = kind.clone();
        }
    }

    /// 🧮️ `self`-relative-to-`other` diff (crate::os_spr::Patchable convention).
    fn diff_patch(&self, other: &Self) -> Option<DagNodePatch> {
        Some(DagNodePatch {
            name: (self.name != other.name).then(|| other.name.clone()),
            x: (self.x != other.x).then_some(other.x),
            y: (self.y != other.y).then_some(other.y),
            width: (self.width != other.width).then_some(other.width),
            height: (self.height != other.height).then_some(other.height),
            kind: (self.kind != other.kind).then(|| other.kind.clone()),
        })
    }
}

/// 🩹️ Sparse patch of a {@link DagHostSnapshotEdge}'s endpoints.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct DagEdgePatch {
    pub source: Option<String>,
    pub target: Option<String>,
}

impl Patchable<DagEdgePatch> for DagHostSnapshotEdge {
    fn apply_patch(&mut self, patch: &DagEdgePatch) {
        if let Some(source) = &patch.source {
            self.source = source.clone();
        }
        if let Some(target) = &patch.target {
            self.target = target.clone();
        }
    }

    /// 🧮️ `self`-relative-to-`other` diff — see {@link DagNodeSpec}'s `Patchable` impl above.
    fn diff_patch(&self, other: &Self) -> Option<DagEdgePatch> {
        Some(DagEdgePatch { source: (self.source != other.source).then(|| other.source.clone()), target: (self.target != other.target).then(|| other.target.clone()) })
    }
}
//#endregion 🔖️ExternalPatchSupport

//#region 🔖️DiffDeltas
const _: () = assert!(usize::BITS <= u64::BITS);

fn dag_index_from_wire(index: u64) -> protocol::MutationApplyResult<usize> {
    usize::try_from(index).map_err(|_| protocol::MutationApplyError::new("mutation.apply.invalid-index", format!("DAG wire index {index} is not representable on this target")))
}

/// 🔢️ Encodes a target-sized DAG index without truncation.
pub fn dag_index_to_wire(index: usize) -> u64 {
    u64::try_from(index).expect("usize::BITS <= u64::BITS")
}

/// 🏷️ `id` → `new_id` — `rename-node`'s delta. `id` is the node's identity field (its display `name`
/// has its own `ChangedNodeName`), so this also drives every `"<id>@<port>"` edge endpoint rewrite.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct RenamedNode {
    pub id: String,
    pub new_id: String,
}

/// ↔️ `move-node`'s delta — FINAL-state absolute `(x, y)`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct MovedNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

/// 📐️ `resize-node`'s delta — FINAL-state absolute `(width, height)`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct ResizedNode {
    pub id: String,
    pub width: f64,
    pub height: f64,
}

/// 🔤️ `change-node-name`'s delta — the node's display label (distinct from its `id`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct ChangedNodeName {
    pub id: String,
    pub new_name: String,
}

/// 🖼️ `change-node-icon`'s delta.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct ChangedNodeIcon {
    pub id: String,
    pub new_icon: String,
}

/// 🔡️ `change-node-abbreviation`'s delta.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct ChangedNodeAbbreviation {
    pub id: String,
    pub new_abbreviation: String,
}

/// 🧮️ `change-node-operator-kind`'s delta — a single (non-nested) `Option<String>`, since the delta
/// struct's own presence on {@link DagDiff} already distinguishes "untouched" from "touched".
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct ChangedNodeOperatorKind {
    pub id: String,
    pub new_operator_kind: Option<String>,
}

/// 🔁️ `replace-node-kind`'s delta — whole-value swap of the tagged `kind` (an 11-variant enum whose
/// interior the editor edits via a clone-mutate-refit cycle, never a sparse per-field patch — see this
/// ticket's report for the measurement that ruled out finer per-variant verbs).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub struct ReplacedNodeKind {
    pub id: String,
    pub new_kind: DagNodeKind,
}

/// 🗃️ `replace-node-properties`'s delta — whole-value swap of the node's `PropertyBag` (no piecewise
/// per-property editing gesture exists on this board).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct ReplacedNodeProperties {
    pub id: String,
    pub new_properties: PropertyBag,
}

/// ↩️ `rename-node`'s edge-endpoint cascade — one entry per edge whose `source`/`target` string
/// referenced the renamed id. `None` means that side of the edge wasn't touched.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct RewrittenEdgeEndpoint {
    pub id: String,
    pub new_source: Option<String>,
    pub new_target: Option<String>,
}
//#endregion 🔖️DiffDeltas

/// 🧬️ Direct leaf-owned mutation vocabulary and mechanical aggregate.
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;

/// 🔺️ Sparse field delta — every field records WHAT CHANGED (an id, a new value, a captured payload),
/// never a whole post-mutation record or a whole-collection/whole-document snapshot.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DagDelta {
    pub created_node: Option<DagNodeSpec>,
    /// 🔢️ Companion to `created_node` — the FINAL-state insertion index (this board's node order is
    /// its z-stack: `DagHost`'s hit-testing walks `nodes.iter().rev()`, later index = frontmost).
    /// Kept as a sibling field rather than nested inside `created_node`, matching `🪐️space`'s
    /// `created_folder_at` convention (the derive engine has no first-class "record + position" shape).
    pub created_node_at: Option<u64>,
    pub deleted_node_ids: Option<Vec<String>>,
    pub renamed_node: Option<RenamedNode>,
    pub moved_node: Option<MovedNode>,
    pub resized_node: Option<ResizedNode>,
    pub changed_node_name: Option<ChangedNodeName>,
    pub changed_node_icon: Option<ChangedNodeIcon>,
    pub changed_node_abbreviation: Option<ChangedNodeAbbreviation>,
    pub changed_node_operator_kind: Option<ChangedNodeOperatorKind>,
    pub replaced_node_kind: Option<ReplacedNodeKind>,
    pub replaced_node_properties: Option<ReplacedNodeProperties>,
    pub reordered_nodes: Option<Vec<String>>,
    pub connected_edge: Option<DagHostSnapshotEdge>,
    pub connected_edge_at: Option<u64>,
    pub disconnected_edge_ids: Option<Vec<String>>,
    /// 🩹️ `rename-node`'s edge cascade only — no direct edge field-change verb exists; any other
    /// endpoint/route/property change on an existing edge is `disconnect-nodes` + `connect-nodes`.
    pub rewritten_edge_endpoints: Option<Vec<RewrittenEdgeEndpoint>>,
}

impl DagDelta {
    fn apply_into(&self, next: &mut DagSnapshot) -> protocol::MutationApplyResult<()> {
        if self.created_node.is_some() != self.created_node_at.is_some() {
            return Err(protocol::MutationApplyError::new("mutation.apply.incomplete-diff", "created node and its final index must be present together").at(["createdNode"]));
        }
        if let (Some(node), Some(wire_at)) = (&self.created_node, self.created_node_at) {
            let at = dag_index_from_wire(wire_at)?;
            if next.nodes.iter().any(|entry| entry.id == node.id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "created node identity already exists").at(["nodes", node.id.as_str()]));
            }
            if at > next.nodes.len() {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", format!("created node final index {at} is out of range for length {}", next.nodes.len())).at(["createdNodeAt"]));
            }
            next.nodes.insert(at, node.clone());
        }
        if let Some(ids) = &self.deleted_node_ids {
            let mut seen = HashSet::new();
            for id in ids {
                if !seen.insert(id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "node is deleted more than once").at(["nodes", id.as_str()]));
                }
                if !next.nodes.iter().any(|node| node.id == *id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "deleted node does not exist").at(["nodes", id.as_str()]));
                }
            }
            next.nodes.retain(|node| !ids.contains(&node.id));
        }
        if let Some(renamed) = &self.renamed_node {
            if next.nodes.iter().any(|node| node.id == renamed.new_id && node.id != renamed.id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "renamed node identity already exists").at(["nodes", renamed.new_id.as_str()]));
            }
            let node = next.nodes.iter_mut().find(|node| node.id == renamed.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "renamed node does not exist").at(["nodes", renamed.id.as_str()]))?;
            node.id = renamed.new_id.clone();
        }
        if let Some(moved) = &self.moved_node {
            let node = next.nodes.iter_mut().find(|node| node.id == moved.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "moved node does not exist").at(["nodes", moved.id.as_str()]))?;
            node.x = moved.x;
            node.y = moved.y;
        }
        if let Some(resized) = &self.resized_node {
            let node = next.nodes.iter_mut().find(|node| node.id == resized.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "resized node does not exist").at(["nodes", resized.id.as_str()]))?;
            node.width = resized.width;
            node.height = resized.height;
        }
        if let Some(changed) = &self.changed_node_name {
            let node = next.nodes.iter_mut().find(|node| node.id == changed.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "changed node does not exist").at(["nodes", changed.id.as_str()]))?;
            node.name = changed.new_name.clone();
        }
        if let Some(changed) = &self.changed_node_icon {
            let node = next.nodes.iter_mut().find(|node| node.id == changed.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "changed node does not exist").at(["nodes", changed.id.as_str()]))?;
            node.icon = changed.new_icon.clone();
        }
        if let Some(changed) = &self.changed_node_abbreviation {
            let node = next.nodes.iter_mut().find(|node| node.id == changed.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "changed node does not exist").at(["nodes", changed.id.as_str()]))?;
            node.abbreviation = changed.new_abbreviation.clone();
        }
        if let Some(changed) = &self.changed_node_operator_kind {
            let node = next.nodes.iter_mut().find(|node| node.id == changed.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "changed node does not exist").at(["nodes", changed.id.as_str()]))?;
            node.operator_kind = changed.new_operator_kind.clone();
        }
        if let Some(replaced) = &self.replaced_node_kind {
            let node = next.nodes.iter_mut().find(|node| node.id == replaced.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "replaced node does not exist").at(["nodes", replaced.id.as_str()]))?;
            node.kind = replaced.new_kind.clone();
        }
        if let Some(replaced) = &self.replaced_node_properties {
            let node = next.nodes.iter_mut().find(|node| node.id == replaced.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "replaced node does not exist").at(["nodes", replaced.id.as_str()]))?;
            node.properties = replaced.new_properties.clone();
        }
        if let Some(order) = &self.reordered_nodes {
            if order.len() != next.nodes.len() {
                return Err(protocol::MutationApplyError::new("mutation.apply.incomplete-diff", format!("node order has length {}, expected {}", order.len(), next.nodes.len())).at(["reorderedNodes"]));
            }
            let mut seen = HashSet::new();
            for id in order {
                if !seen.insert(id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "node appears more than once in order").at(["reorderedNodes", id.as_str()]));
                }
                if !next.nodes.iter().any(|node| node.id == *id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "ordered node does not exist").at(["reorderedNodes", id.as_str()]));
                }
            }
            let mut reordered: Vec<DagNodeSpec> = Vec::with_capacity(next.nodes.len());
            for id in order {
                let at = next.nodes.iter().position(|node| &node.id == id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "ordered node does not exist").at(["reorderedNodes", id.as_str()]))?;
                reordered.push(next.nodes.remove(at));
            }
            next.nodes = reordered;
        }
        if self.connected_edge.is_some() != self.connected_edge_at.is_some() {
            return Err(protocol::MutationApplyError::new("mutation.apply.incomplete-diff", "connected edge and its final index must be present together").at(["connectedEdge"]));
        }
        if let (Some(edge), Some(wire_at)) = (&self.connected_edge, self.connected_edge_at) {
            let at = dag_index_from_wire(wire_at)?;
            if next.edges.iter().any(|entry| entry.id == edge.id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "connected edge identity already exists").at(["edges", edge.id.as_str()]));
            }
            for endpoint in [&edge.source, &edge.target] {
                let node_id = split_dag_endpoint(endpoint).0;
                if !next.nodes.iter().any(|node| node.id == node_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "connected edge endpoint node does not exist").at(["edges", edge.id.as_str(), node_id.as_str()]));
                }
            }
            if at > next.edges.len() {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", format!("connected edge final index {at} is out of range for length {}", next.edges.len())).at(["connectedEdgeAt"]));
            }
            next.edges.insert(at, edge.clone());
        }
        if let Some(ids) = &self.disconnected_edge_ids {
            let mut seen = HashSet::new();
            for id in ids {
                if !seen.insert(id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "edge is disconnected more than once").at(["edges", id.as_str()]));
                }
                if !next.edges.iter().any(|edge| edge.id == *id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "disconnected edge does not exist").at(["edges", id.as_str()]));
                }
            }
            next.edges.retain(|edge| !ids.contains(&edge.id));
        }
        if let Some(rewrites) = &self.rewritten_edge_endpoints {
            let mut seen = HashSet::new();
            for rewrite in rewrites {
                if !seen.insert(&rewrite.id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "edge endpoint is rewritten more than once").at(["edges", rewrite.id.as_str()]));
                }
                if rewrite.new_source.is_none() && rewrite.new_target.is_none() {
                    return Err(protocol::MutationApplyError::new("mutation.apply.incomplete-diff", "edge endpoint rewrite contains no endpoint").at(["edges", rewrite.id.as_str()]));
                }
                for endpoint in [rewrite.new_source.as_ref(), rewrite.new_target.as_ref()].into_iter().flatten() {
                    let node_id = split_dag_endpoint(endpoint).0;
                    if !next.nodes.iter().any(|node| node.id == node_id) {
                        return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "rewritten edge endpoint node does not exist").at(["edges", rewrite.id.as_str(), node_id.as_str()]));
                    }
                }
                let edge = next.edges.iter_mut().find(|edge| edge.id == rewrite.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "rewritten edge does not exist").at(["edges", rewrite.id.as_str()]))?;
                if let Some(source) = &rewrite.new_source {
                    edge.source = source.clone();
                }
                if let Some(target) = &rewrite.new_target {
                    edge.target = target.clone();
                }
            }
        }
        Ok(())
    }
}

/// 🎞️ Ordered structural deltas; composition preserves every preceding effect and rejection.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DagDiff {
    pub steps: Vec<DagDelta>,
}

impl From<DagDelta> for DagDiff {
    fn from(delta: DagDelta) -> Self {
        if delta == DagDelta::default() {
            Self::default()
        } else {
            Self { steps: vec![delta] }
        }
    }
}

impl DagDelta {
    /// ⚛️ One single-slot delta per populated slot, in `apply_into` order, so each can be inverted against the exact state it ran on.
    fn atoms(&self) -> Vec<DagDelta> {
        let mut atoms = Vec::new();
        if self.created_node.is_some() || self.created_node_at.is_some() {
            atoms.push(DagDelta { created_node: self.created_node.clone(), created_node_at: self.created_node_at, ..DagDelta::default() });
        }
        if self.deleted_node_ids.is_some() {
            atoms.push(DagDelta { deleted_node_ids: self.deleted_node_ids.clone(), ..DagDelta::default() });
        }
        if self.renamed_node.is_some() {
            atoms.push(DagDelta { renamed_node: self.renamed_node.clone(), ..DagDelta::default() });
        }
        if self.moved_node.is_some() {
            atoms.push(DagDelta { moved_node: self.moved_node.clone(), ..DagDelta::default() });
        }
        if self.resized_node.is_some() {
            atoms.push(DagDelta { resized_node: self.resized_node.clone(), ..DagDelta::default() });
        }
        if self.changed_node_name.is_some() {
            atoms.push(DagDelta { changed_node_name: self.changed_node_name.clone(), ..DagDelta::default() });
        }
        if self.changed_node_icon.is_some() {
            atoms.push(DagDelta { changed_node_icon: self.changed_node_icon.clone(), ..DagDelta::default() });
        }
        if self.changed_node_abbreviation.is_some() {
            atoms.push(DagDelta { changed_node_abbreviation: self.changed_node_abbreviation.clone(), ..DagDelta::default() });
        }
        if self.changed_node_operator_kind.is_some() {
            atoms.push(DagDelta { changed_node_operator_kind: self.changed_node_operator_kind.clone(), ..DagDelta::default() });
        }
        if self.replaced_node_kind.is_some() {
            atoms.push(DagDelta { replaced_node_kind: self.replaced_node_kind.clone(), ..DagDelta::default() });
        }
        if self.replaced_node_properties.is_some() {
            atoms.push(DagDelta { replaced_node_properties: self.replaced_node_properties.clone(), ..DagDelta::default() });
        }
        if self.reordered_nodes.is_some() {
            atoms.push(DagDelta { reordered_nodes: self.reordered_nodes.clone(), ..DagDelta::default() });
        }
        if self.connected_edge.is_some() || self.connected_edge_at.is_some() {
            atoms.push(DagDelta { connected_edge: self.connected_edge.clone(), connected_edge_at: self.connected_edge_at, ..DagDelta::default() });
        }
        if self.disconnected_edge_ids.is_some() {
            atoms.push(DagDelta { disconnected_edge_ids: self.disconnected_edge_ids.clone(), ..DagDelta::default() });
        }
        if self.rewritten_edge_endpoints.is_some() {
            atoms.push(DagDelta { rewritten_edge_endpoints: self.rewritten_edge_endpoints.clone(), ..DagDelta::default() });
        }
        atoms
    }

    /// ↩️ The steps that undo one single-slot delta, in application order, read from the state `before` it ran on.
    fn atom_inverse(&self, before: &DagSnapshot) -> Vec<DagDelta> {
        let node = |id: &str| before.nodes.iter().find(|node| node.id == id);
        let mut steps = Vec::new();
        if let Some(created) = &self.created_node {
            steps.push(DagDelta { deleted_node_ids: Some(vec![created.id.clone()]), ..DagDelta::default() });
        }
        if let Some(ids) = &self.deleted_node_ids {
            for (at, node) in before.nodes.iter().enumerate().filter(|(_, node)| ids.contains(&node.id)) {
                steps.push(DagDelta { created_node: Some(node.clone()), created_node_at: Some(dag_index_to_wire(at)), ..DagDelta::default() });
            }
        }
        if let Some(renamed) = &self.renamed_node {
            steps.push(DagDelta { renamed_node: Some(RenamedNode { id: renamed.new_id.clone(), new_id: renamed.id.clone() }), ..DagDelta::default() });
        }
        if let Some(moved) = &self.moved_node {
            steps.extend(node(&moved.id).map(|node| DagDelta { moved_node: Some(MovedNode { id: moved.id.clone(), x: node.x, y: node.y }), ..DagDelta::default() }));
        }
        if let Some(resized) = &self.resized_node {
            steps.extend(node(&resized.id).map(|node| DagDelta { resized_node: Some(ResizedNode { id: resized.id.clone(), width: node.width, height: node.height }), ..DagDelta::default() }));
        }
        if let Some(changed) = &self.changed_node_name {
            steps.extend(node(&changed.id).map(|node| DagDelta { changed_node_name: Some(ChangedNodeName { id: changed.id.clone(), new_name: node.name.clone() }), ..DagDelta::default() }));
        }
        if let Some(changed) = &self.changed_node_icon {
            steps.extend(node(&changed.id).map(|node| DagDelta { changed_node_icon: Some(ChangedNodeIcon { id: changed.id.clone(), new_icon: node.icon.clone() }), ..DagDelta::default() }));
        }
        if let Some(changed) = &self.changed_node_abbreviation {
            steps.extend(node(&changed.id).map(|node| DagDelta { changed_node_abbreviation: Some(ChangedNodeAbbreviation { id: changed.id.clone(), new_abbreviation: node.abbreviation.clone() }), ..DagDelta::default() }));
        }
        if let Some(changed) = &self.changed_node_operator_kind {
            steps.extend(node(&changed.id).map(|node| DagDelta { changed_node_operator_kind: Some(ChangedNodeOperatorKind { id: changed.id.clone(), new_operator_kind: node.operator_kind.clone() }), ..DagDelta::default() }));
        }
        if let Some(replaced) = &self.replaced_node_kind {
            steps.extend(node(&replaced.id).map(|node| DagDelta { replaced_node_kind: Some(ReplacedNodeKind { id: replaced.id.clone(), new_kind: node.kind.clone() }), ..DagDelta::default() }));
        }
        if let Some(replaced) = &self.replaced_node_properties {
            steps.extend(node(&replaced.id).map(|node| DagDelta { replaced_node_properties: Some(ReplacedNodeProperties { id: replaced.id.clone(), new_properties: node.properties.clone() }), ..DagDelta::default() }));
        }
        if self.reordered_nodes.is_some() {
            steps.push(DagDelta { reordered_nodes: Some(before.nodes.iter().map(|node| node.id.clone()).collect()), ..DagDelta::default() });
        }
        if let Some(edge) = &self.connected_edge {
            steps.push(DagDelta { disconnected_edge_ids: Some(vec![edge.id.clone()]), ..DagDelta::default() });
        }
        if let Some(ids) = &self.disconnected_edge_ids {
            for (at, edge) in before.edges.iter().enumerate().filter(|(_, edge)| ids.contains(&edge.id)) {
                steps.push(DagDelta { connected_edge: Some(edge.clone()), connected_edge_at: Some(dag_index_to_wire(at)), ..DagDelta::default() });
            }
        }
        if let Some(rewrites) = &self.rewritten_edge_endpoints {
            let restored: Vec<RewrittenEdgeEndpoint> = rewrites
                .iter()
                .filter_map(|rewrite| {
                    before.edges.iter().find(|edge| edge.id == rewrite.id).map(|edge| RewrittenEdgeEndpoint {
                        id: rewrite.id.clone(),
                        new_source: rewrite.new_source.as_ref().map(|_| edge.source.clone()),
                        new_target: rewrite.new_target.as_ref().map(|_| edge.target.clone()),
                    })
                })
                .collect();
            steps.push(DagDelta { rewritten_edge_endpoints: Some(restored), ..DagDelta::default() });
        }
        steps
    }
}

impl DiffAlgebra<DagSnapshot> for DagDiff {
    fn inverse(&self, base: &DagSnapshot) -> Self {
        let mut state = base.clone();
        let mut groups = Vec::new();
        for atom in self.steps.iter().flat_map(DagDelta::atoms) {
            groups.push(atom.atom_inverse(&state));
            if atom.apply_into(&mut state).is_err() {
                return Self::default();
            }
        }
        Self { steps: groups.into_iter().rev().flatten().collect() }
    }

    fn between(base: &DagSnapshot, other: &DagSnapshot) -> Self {
        let mut steps = vec![DagDelta { disconnected_edge_ids: Some(base.edges.iter().map(|edge| edge.id.clone()).collect()), ..DagDelta::default() }, DagDelta { deleted_node_ids: Some(base.nodes.iter().map(|node| node.id.clone()).collect()), ..DagDelta::default() }];
        steps.extend(other.nodes.iter().enumerate().map(|(at, node)| DagDelta { created_node: Some(node.clone()), created_node_at: Some(dag_index_to_wire(at)), ..DagDelta::default() }));
        steps.extend(other.edges.iter().enumerate().map(|(at, edge)| DagDelta { connected_edge: Some(edge.clone()), connected_edge_at: Some(dag_index_to_wire(at)), ..DagDelta::default() }));
        steps.retain(|step| step.disconnected_edge_ids.as_ref().is_none_or(|ids| !ids.is_empty()) && step.deleted_node_ids.as_ref().is_none_or(|ids| !ids.is_empty()));
        Self { steps }
    }

    fn is_empty(&self) -> bool {
        self.steps.iter().all(|step| *step == DagDelta::default())
    }
}

impl MutationDiff<DagSnapshot> for DagDiff {
    fn apply(&self, snapshot: &DagSnapshot, _capability: ApplyCapability) -> protocol::MutationApplyResult<DagSnapshot> {
        let mut next = snapshot.clone();
        for step in &self.steps {
            step.apply_into(&mut next)?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.steps.extend(other.steps);
    }
}

pub type DagEnvelope = ArtifactEnvelope<DagSnapshot, DagMutation>;
pub type DagStore = ArtifactStore<DagSnapshot, DagMutation>;

/// 🏭️ Creates a DAG store with the exact snapshot, mutation, and cursor retirement owners installed.
pub async fn create_dag_store(id: &str, snapshot: DagSnapshot, actor: ActorId) -> Result<DagStore, crate::os_store::VcsError> {
    use crate::os_store::MemberStoreOwner as _;
    let mut store = DagStore::new(create_document_envelope(DAG_DOCUMENT_SCHEMA, id, snapshot, None), actor).await?;
    store.install_document_store_owners_exact(DagSnapshot::member_store_owners());
    Ok(store)
}

#[cfg(test)]
fn opened_dag_test_actor() -> ActorId {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪪️demo-ownership/🔣️.json")).unwrap();
    ActorId(fixture["openedActor"].as_str().unwrap().into())
}

#[cfg(test)]
fn close_dag_test_store(mut store: DagStore) {
    loop {
        match store.close_owned_store_step(1, 4_096).expect("bounded DAG test-store close") {
            crate::os_store::SnapshotRetirementStep::Complete => break,
            crate::os_store::SnapshotRetirementStep::Pending { .. } => {}
            crate::os_store::SnapshotRetirementStep::Blocked => panic!("nonzero DAG test-store close grant blocked"),
        }
    }
    assert!(store.close_owned_store_terminal_is_empty());
}

//#region 🔖️Dsl


//#endregion 🔖️Dsl

//#region 🔖️OpText



//#endregion 🔖️OpText

#[cfg(test)]
#[path = "🧪️tests/🔬️dag-vcs/🦀️.rs"]
mod dag_vcs_tests;
// #endregion 🔖️ArtifactVcs

#[cfg(test)]
#[path = "🧪️tests/🔬️dag-direct/🦀️.rs"]
mod dag_direct_tests;

#[path = "../🚪️io/🦀️.rs"]
pub mod io;
