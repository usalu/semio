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
use crate::os_spr::{Identified, MutationDiff, Patchable};
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

impl MutationDiff<DagSnapshot> for DagDiff {
    fn apply(&self, snapshot: &DagSnapshot) -> protocol::MutationApplyResult<DagSnapshot> {
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
pub async fn create_dag_store(id: &str, snapshot: DagSnapshot) -> Result<DagStore, crate::os_store::VcsError> {
    use crate::os_store::MemberStoreOwner as _;
    let mut store = DagStore::new(create_document_envelope(DAG_DOCUMENT_SCHEMA, id, snapshot, None)).await?;
    store.install_document_store_owners_exact(DagSnapshot::member_store_owners());
    Ok(store)
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
