//! 🧬️ DAG mutation, diff, and document codecs.
use crate::snapshot::*;
use ::graph::manifest::PropertyBag;
#[cfg(test)]
use ::graph::manifest::PropertyValue;
#[cfg(test)]
use dsl::os_pack::json::Value;
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use std::collections::BTreeSet;
use std::collections::HashSet;

// #region 🔖️ArtifactVcs
#[cfg(test)]
use crate::os_spr::Mutation;
#[cfg(test)]
use crate::os_spr::{ArtifactId, Edit, SchemaId};
use crate::os_spr::{Identified, MutationDiff, Patchable};
#[cfg(any(test, all(target_arch = "wasm32", not(target_env = "p2"))))]
use crate::os_store::create_document_envelope;
#[cfg(test)]
use crate::os_store::ArtifactCommand;
use crate::os_store::{ArtifactEnvelope, ArtifactStore};

pub const DAG_DOCUMENT_SCHEMA: &str = "dag.fixture";

fn dag_document_schema() -> String {
    DAG_DOCUMENT_SCHEMA.into()
}

/// 🧾️ The persistent DAG projection — nodes and edges only. Camera/viewport and selection are
/// ephemeral view state kept in the plugin runtime, never recorded in the document's undo history.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DagSnapshot {
    #[value(default = "dag_document_schema")]
    pub schema: String,
    #[value(default)]
    pub nodes: Vec<DagNodeSpec>,
    #[value(default)]
    pub edges: Vec<DagFixtureEdge>,
}

pub fn empty_dag_document() -> DagSnapshot {
    DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() }
}

/// 🌱️ The demo document seed (nodes/edges from `🕸️demo.dag`), sharing the fixture's example.
pub fn default_dag_document() -> DagSnapshot {
    dag_document_from_fixture(&DagFixture::default())
}

/// 🔁️ Projects a {@link DagFixture} (which also carries a camera) down to the persistent {@link DagSnapshot}.
pub fn dag_document_from_fixture(fixture: &DagFixture) -> DagSnapshot {
    DagSnapshot { schema: fixture.schema.clone(), nodes: fixture.nodes.clone(), edges: fixture.edges.clone() }
}

/// 🔁️ Rehydrates a full {@link DagFixture} from a {@link DagSnapshot} plus a runtime `camera`, so the
/// existing fixture-shaped helpers (`dag_fixture_to_wire_literal`, `DagHost`, …) can be reused.
pub fn dag_fixture_from_document(document: &DagSnapshot, camera: DagCamera) -> DagFixture {
    DagFixture { schema: document.schema.clone(), camera, nodes: document.nodes.clone(), edges: document.edges.clone() }
}

//#region 🔖️ExternalPatchSupport
// 🧬️ `DagNodePatch`/`DagEdgePatch` (+ the `Identified`/`Patchable` impls that make them usable) are no
// longer consumed by THIS file's own `DagMutation`/`DagDiff` (see `🔖️DiffDeltas` below for the
// dedicated per-verb delta types that replaced them here) — but they are re-exported verbatim
// (`pub use infinite_board_port_directed_dag::{DagEdgePatch, DagNodePatch, ...}`,
// `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️.rs:9-11`) and directly consumed as a live,
// load-bearing dependency by that plugin's own already-landed `🧬️mutations` facet: its
// `apply_nodes_delta`/`apply_edges_delta`/`apply_identified_delta` helpers
// (`…/🧬️schema/🔺️diff/📝️text/🦀️.rs`) are generic over `T: Identified<String> + Patchable<P>`
// and call `.apply_patch(...)` on `DagNodeSpec`/`DagFixtureEdge` using exactly these impls. Deleting
// them would compile-break that plugin's facet, which is the same "boundary that separates a
// definition from its registration is a race" failure this ticket's own doctrine warns against —
// so this file keeps them, unrelated to and independent of the `CollectionMutation<TId,TItem,TPatch>`
// elimination below (this crate's own `DagMutation` never wraps them in that banned generic type).
impl Identified<String> for DagNodeSpec {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for DagFixtureEdge {
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

/// 🩹️ Sparse patch of a {@link DagFixtureEdge}'s endpoints.
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
pub struct DagEdgePatch {
    pub source: Option<String>,
    pub target: Option<String>,
}

impl Patchable<DagEdgePatch> for DagFixtureEdge {
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct RenamedNode {
    pub id: String,
    pub new_id: String,
}

/// ↔️ `move-node`'s delta — FINAL-state absolute `(x, y)`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct MovedNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

/// 📐️ `resize-node`'s delta — FINAL-state absolute `(width, height)`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct ResizedNode {
    pub id: String,
    pub width: f64,
    pub height: f64,
}

/// 🔤️ `change-node-name`'s delta — the node's display label (distinct from its `id`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct ChangedNodeName {
    pub id: String,
    pub new_name: String,
}

/// 🖼️ `change-node-icon`'s delta.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct ChangedNodeIcon {
    pub id: String,
    pub new_icon: String,
}

/// 🔡️ `change-node-abbreviation`'s delta.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct ChangedNodeAbbreviation {
    pub id: String,
    pub new_abbreviation: String,
}

/// 🧮️ `change-node-operator-kind`'s delta — a single (non-nested) `Option<String>`, since the delta
/// struct's own presence on {@link DagDiff} already distinguishes "untouched" from "touched".
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct ReplacedNodeProperties {
    pub id: String,
    pub new_properties: PropertyBag,
}

/// ↩️ `rename-node`'s edge-endpoint cascade — one entry per edge whose `source`/`target` string
/// referenced the renamed id. `None` means that side of the edge wasn't touched.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
    pub connected_edge: Option<DagFixtureEdge>,
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

//#region 🔖️Dsl
// 🧬️ `.dag` document DSL via the `crate::os_dsl::` derive engine (see `🔖️DslMirror` below) — every persisted
// type (`DagSnapshot`/`DagNodeSpec`/`DagNodeKind`/`DagFixtureEdge`/`IoPortSpec`/`DagMedia`/
// `DagPreviewContent`/`PortShape`/`EdgeRouteStyle`/`DagMediaKind`/patches) either derives a
// `dsl::Dsl*` macro directly or, where the real Rust field shape can't satisfy the derive engine
// (a bare tagged-enum field where the engine requires `Box<T>`), converts through a small local
// mirror type at the `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary. This replaces the old
// hand-rolled `graph::dsl` wire-literal-based printer/parser that used to live in this
// region (deleted; `dag_fixture_to_wire_literal`/`dag_fixture_execution_rows` near {@link DagHost}
// still use the wire-literal grammar directly for their own, unrelated purpose and are untouched).

//#region 🔖️DslMirror
// 🧬️ `DagNodeKind` is `#[serde(flatten)]`-merged onto `DagNodeSpec` at the JSON level, and its own
// `Preview` variant carries a nested tagged enum (`DagPreviewContent`). The crate::os_dsl:: derive engine
// represents "exactly one nested tagged value" via `#[dsl(statements)] Box<T>` (`RequiredStatements`),
// which needs a `Box` wrapper the REAL `DagNodeKind`/`DagNodeSpec` fields deliberately don't carry
// (dozens of call sites here and in `dag-plugin`/`framework/surface/node-graph`/`flow/core` destructure
// `node.kind`/`DagNodeKind::Preview { content, .. }` directly — boxing those fields would ripple far
// outside this crate's ownership). So, exactly like `imperative/core/rs`'s `ProcedureMutationDsl`
// mirror, `DagNodeKindDsl`/`DagNodeSpecDsl`/`DagSnapshotDsl` are
// LOCAL structural twins that box only where the derive requires it; the real domain types keep their
// original unboxed shape and never leave this crate — conversion happens right at this boundary.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
pub enum DagNodeKindDsl {
    Computation {
        #[dsl(table)]
        inputs: Vec<IoPortSpec>,
        #[dsl(table)]
        outputs: Vec<IoPortSpec>,
        variadic_inputs: bool,
        variadic_outputs: bool,
    },
    Slider {
        min: f64,
        max: f64,
        step: f64,
        value: f64,
        output: IoPortSpec,
    },
    Select {
        options: Vec<String>,
        selected: u64,
        output: IoPortSpec,
    },
    Screen {
        media: Option<DagMedia>,
        input: IoPortSpec,
    },
    Note {
        text: String,
        output: IoPortSpec,
    },
    Image {
        src: String,
        output: IoPortSpec,
    },
    Preview {
        #[dsl(statements)]
        content: Box<DagPreviewContent>,
        expanded: Vec<String>,
        input: IoPortSpec,
    },
    Action {
        label: String,
        input: IoPortSpec,
    },
    Export {
        label: String,
        format: String,
        input: IoPortSpec,
    },
    Cluster {
        #[dsl(table)]
        inputs: Vec<IoPortSpec>,
        #[dsl(table)]
        outputs: Vec<IoPortSpec>,
    },
    AppInstance {
        instance_id: String,
        plugin_id: String,
        app_id: String,
        icon: String,
        #[dsl(table)]
        inputs: Vec<IoPortSpec>,
        #[dsl(table)]
        outputs: Vec<IoPortSpec>,
    },
}

fn dag_node_kind_to_dsl(kind: &DagNodeKind) -> DagNodeKindDsl {
    match kind {
        DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs } => DagNodeKindDsl::Computation { inputs: inputs.clone(), outputs: outputs.clone(), variadic_inputs: *variadic_inputs, variadic_outputs: *variadic_outputs },
        DagNodeKind::Slider { min, max, step, value, output } => DagNodeKindDsl::Slider { min: *min, max: *max, step: *step, value: *value, output: output.clone() },
        DagNodeKind::Select { options, selected, output } => DagNodeKindDsl::Select { options: options.clone(), selected: *selected, output: output.clone() },
        DagNodeKind::Screen { media, input } => DagNodeKindDsl::Screen { media: media.clone(), input: input.clone() },
        DagNodeKind::Note { text, output } => DagNodeKindDsl::Note { text: text.clone(), output: output.clone() },
        DagNodeKind::Image { src, output } => DagNodeKindDsl::Image { src: src.clone(), output: output.clone() },
        DagNodeKind::Preview { content, expanded, input } => DagNodeKindDsl::Preview { content: Box::new(content.clone()), expanded: expanded.iter().cloned().collect(), input: input.clone() },
        DagNodeKind::Action { label, input } => DagNodeKindDsl::Action { label: label.clone(), input: input.clone() },
        DagNodeKind::Export { label, format, input } => DagNodeKindDsl::Export { label: label.clone(), format: format.clone(), input: input.clone() },
        DagNodeKind::Cluster { inputs, outputs } => DagNodeKindDsl::Cluster { inputs: inputs.clone(), outputs: outputs.clone() },
        DagNodeKind::AppInstance { instance_id, plugin_id, app_id, icon, inputs, outputs } => {
            DagNodeKindDsl::AppInstance { instance_id: instance_id.clone(), plugin_id: plugin_id.clone(), app_id: app_id.clone(), icon: icon.clone(), inputs: inputs.clone(), outputs: outputs.clone() }
        }
    }
}

fn dag_node_kind_from_dsl(kind: DagNodeKindDsl) -> DagNodeKind {
    match kind {
        DagNodeKindDsl::Computation { inputs, outputs, variadic_inputs, variadic_outputs } => DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs },
        DagNodeKindDsl::Slider { min, max, step, value, output } => DagNodeKind::Slider { min, max, step, value, output },
        DagNodeKindDsl::Select { options, selected, output } => DagNodeKind::Select { options, selected, output },
        DagNodeKindDsl::Screen { media, input } => DagNodeKind::Screen { media, input },
        DagNodeKindDsl::Note { text, output } => DagNodeKind::Note { text, output },
        DagNodeKindDsl::Image { src, output } => DagNodeKind::Image { src, output },
        DagNodeKindDsl::Preview { content, expanded, input } => DagNodeKind::Preview { content: *content, expanded: expanded.into_iter().collect(), input },
        DagNodeKindDsl::Action { label, input } => DagNodeKind::Action { label, input },
        DagNodeKindDsl::Export { label, format, input } => DagNodeKind::Export { label, format, input },
        DagNodeKindDsl::Cluster { inputs, outputs } => DagNodeKind::Cluster { inputs, outputs },
        DagNodeKindDsl::AppInstance { instance_id, plugin_id, app_id, icon, inputs, outputs } => DagNodeKind::AppInstance { instance_id, plugin_id, app_id, icon, inputs, outputs },
    }
}

/// 🧬️ Mirror of {@link DagNodeSpec} — every field identical except `kind`, boxed only here (see the
/// region's opening doc comment).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct DagNodeSpecDsl {
    id: String,
    name: String,
    abbreviation: String,
    icon: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    operator_kind: Option<String>,
    properties: PropertyBag,
    #[dsl(statements)]
    kind: Box<DagNodeKindDsl>,
}

fn dag_node_spec_to_dsl(node: &DagNodeSpec) -> DagNodeSpecDsl {
    DagNodeSpecDsl {
        id: node.id.clone(),
        name: node.name.clone(),
        abbreviation: node.abbreviation.clone(),
        icon: node.icon.clone(),
        x: node.x,
        y: node.y,
        width: node.width,
        height: node.height,
        operator_kind: node.operator_kind.clone(),
        properties: node.properties.clone(),
        kind: Box::new(dag_node_kind_to_dsl(&node.kind)),
    }
}

fn dag_node_spec_from_dsl(mirror: DagNodeSpecDsl) -> DagNodeSpec {
    DagNodeSpec {
        id: mirror.id,
        name: mirror.name,
        abbreviation: mirror.abbreviation,
        icon: mirror.icon,
        x: mirror.x,
        y: mirror.y,
        width: mirror.width,
        height: mirror.height,
        operator_kind: mirror.operator_kind,
        properties: mirror.properties,
        kind: dag_node_kind_from_dsl(*mirror.kind),
    }
}

impl dsl::DslField for DagNodeKind {
    fn shape() -> dsl::Shape {
        dsl::Shape::Statements(<DagNodeKindDsl as dsl::DslVariants>::variants())
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Statements(vec![<DagNodeKindDsl as dsl::DslVariants>::to_named_record(&dag_node_kind_to_dsl(self))])
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Statements(items) if items.len() == 1 => <DagNodeKindDsl as dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1).map(dag_node_kind_from_dsl).map_err(|error| error.to_string()),
            other => Err(format!("expected exactly one DagNodeKind statement, found {other:?}")),
        }
    }
}

impl dsl::DslField for DagNodeSpec {
    fn shape() -> dsl::Shape {
        <DagNodeSpecDsl as dsl::DslField>::shape()
    }
    fn to_value(&self) -> dsl::FieldValue {
        <DagNodeSpecDsl as dsl::DslField>::to_value(&dag_node_spec_to_dsl(self))
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        <DagNodeSpecDsl as dsl::DslField>::from_value(value).map(dag_node_spec_from_dsl)
    }
}

/// 🧬️ Document lowering shares the intrinsic node record representation with mutation payloads.
#[derive(Clone, Debug, PartialEq, dsl::DslArtifact)]
#[dsl(id = "dag.dag")]
#[dsl(layout = "lines")]
struct DagSnapshotDsl {
    schema: String,
    nodes: Vec<DagNodeSpecDsl>,
    #[dsl(table)]
    edges: Vec<DagFixtureEdge>,
}

fn dag_snapshot_to_dsl(document: &DagSnapshot) -> DagSnapshotDsl {
    DagSnapshotDsl { schema: document.schema.clone(), nodes: document.nodes.iter().map(dag_node_spec_to_dsl).collect(), edges: document.edges.clone() }
}

fn dag_snapshot_from_dsl(mirror: DagSnapshotDsl) -> DagSnapshot {
    DagSnapshot { schema: mirror.schema, nodes: mirror.nodes.into_iter().map(dag_node_spec_from_dsl).collect(), edges: mirror.edges }
}

/// 📜️ Handcrafted ArtifactDsl (P6): derive no longer emits ArtifactDsl/ArtifactPack.
impl crate::os_store::ArtifactDsl for DagSnapshotDsl {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, crate::os_store::TextError> {
        let body = match crate::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl crate::os_store::ArtifactPack for DagSnapshotDsl {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let inner = crate::os_store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as crate::os_store::ArtifactDsl>::envelope_id() {
            return Err(crate::os_store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as crate::os_store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = crate::os_store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl crate::os_store::ArtifactDsl for DagSnapshot {
    const EXTENSION: &'static str = "dag";

    fn parse_dsl(text: &str) -> Result<Self, crate::os_store::TextError> {
        Ok(dag_snapshot_from_dsl(<DagSnapshotDsl as crate::os_store::ArtifactDsl>::parse_dsl(text)?))
    }

    fn print_dsl(&self) -> String {
        <DagSnapshotDsl as crate::os_store::ArtifactDsl>::print_dsl(&dag_snapshot_to_dsl(self))
    }
}

/// 📦️ Binary counterpart of the `ArtifactDsl` impl above — `DagSnapshot` can't `#[derive(crate::os_dsl::
/// DslArtifact)]` directly (see this region's opening doc comment), so `ArtifactPack` is hand-routed
/// through the same `DagSnapshotDsl` mirror, which does derive it.
impl crate::os_store::ArtifactPack for DagSnapshot {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        <DagSnapshotDsl as crate::os_store::ArtifactPack>::encode_pack_with(&dag_snapshot_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        Ok(dag_snapshot_from_dsl(<DagSnapshotDsl as crate::os_store::ArtifactPack>::decode_pack_with(bytes, options)?))
    }
}
//#endregion 🔖️DslMirror
//#endregion 🔖️Dsl

//#region 🔖️OpText
impl crate::os_spr::OpText for DagMutation {
    fn parse_op(line: &str) -> Result<Self, crate::os_store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }

    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(name, _)| name == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl crate::os_spr::OpBinary for DagMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️OpText

#[cfg(test)]
mod dag_vcs_tests {
    use super::*;

    fn sample_node(id: &str) -> DagNodeSpec {
        DagNodeSpec { id: id.into(), name: id.into(), ..Default::default() }
    }

    fn round_trip(document: &DagSnapshot, operation: &DagMutation) -> DagSnapshot {
        let forward = operation.diff(document).diff().apply(document).expect("valid DAG diff");
        let mut restored = forward.clone();
        for back in operation.inverse(document).into_iter().rev() {
            restored = back.diff(&restored).diff().apply(&restored).expect("valid inverse DAG diff");
        }
        assert_eq!(&restored, document, "inverse() must exactly restore the pre-operation document");
        forward
    }

    #[semio_framework_async_macros::async_test]
    async fn dag_document_vcs_replays_node_operations() {
        let mut store = DagStore::new(create_document_envelope(DAG_DOCUMENT_SCHEMA, "dag", empty_dag_document(), None)).await.expect("store");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 })], description: None }).await.expect("apply");
        assert_eq!(store.snapshot().expect("projection").nodes.len(), 1);
    }

    #[test]
    fn node_create_move_resize_delete_round_trip() {
        let document = empty_dag_document();
        let added = round_trip(&document, &DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
        assert_eq!(added.nodes.len(), 1);
        let moved = round_trip(&added, &DagMutation::MoveNode(MoveNode { id: "n1".into(), x: 42.0, y: 7.0 }));
        assert_eq!(moved.nodes[0].x, 42.0);
        assert_eq!(moved.nodes[0].y, 7.0);
        let resized = round_trip(&moved, &DagMutation::ResizeNode(ResizeNode { id: "n1".into(), width: 200.0, height: 90.0 }));
        assert_eq!(resized.nodes[0].width, 200.0);
        assert_eq!(resized.nodes[0].height, 90.0);
        let removed = round_trip(&resized, &DagMutation::DeleteNode(DeleteNode { id: "n1".into() }));
        assert!(removed.nodes.is_empty());
    }

    #[test]
    fn node_scalar_field_changes_round_trip() {
        let document = round_trip(&empty_dag_document(), &DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
        let renamed_label = round_trip(&document, &DagMutation::ChangeNodeName(ChangeNodeName { id: "n1".into(), new_name: "Renamed label".into() }));
        assert_eq!(renamed_label.nodes[0].name, "Renamed label");
        let iconed = round_trip(&renamed_label, &DagMutation::ChangeNodeIcon(ChangeNodeIcon { id: "n1".into(), new_icon: "emoji:🧪️".into() }));
        assert_eq!(iconed.nodes[0].icon, "emoji:🧪️");
        let abbreviated = round_trip(&iconed, &DagMutation::ChangeNodeAbbreviation(ChangeNodeAbbreviation { id: "n1".into(), new_abbreviation: "N1".into() }));
        assert_eq!(abbreviated.nodes[0].abbreviation, "N1");
        let with_operator = round_trip(&abbreviated, &DagMutation::ChangeNodeOperatorKind(ChangeNodeOperatorKind { id: "n1".into(), new_operator_kind: Some("math.add".into()) }));
        assert_eq!(with_operator.nodes[0].operator_kind.as_deref(), Some("math.add"));
        let without_operator = round_trip(&with_operator, &DagMutation::ChangeNodeOperatorKind(ChangeNodeOperatorKind { id: "n1".into(), new_operator_kind: None }));
        assert_eq!(without_operator.nodes[0].operator_kind, None);
    }

    #[test]
    fn replace_node_kind_and_properties_round_trip() {
        let document = round_trip(&empty_dag_document(), &DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
        let new_kind = DagNodeKind::Slider { min: 0.0, max: 1.0, step: 0.1, value: 0.5, output: IoPortSpec::simple("out", "value") };
        let replaced_kind = round_trip(&document, &DagMutation::ReplaceNodeKind(ReplaceNodeKind { id: "n1".into(), new_kind: new_kind.clone() }));
        assert_eq!(replaced_kind.nodes[0].kind, new_kind);
        let new_properties = PropertyBag::from([("weight".to_string(), PropertyValue::Number(3.0))]);
        let replaced_properties = round_trip(&replaced_kind, &DagMutation::ReplaceNodeProperties(ReplaceNodeProperties { id: "n1".into(), new_properties: new_properties.clone() }));
        assert_eq!(replaced_properties.nodes[0].properties, new_properties);
    }

    #[test]
    fn rename_node_cascades_edge_endpoints() {
        let mut document = empty_dag_document();
        document.nodes = vec![sample_node("a"), sample_node("b")];
        document.edges = vec![DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() }];
        let renamed = round_trip(&document, &DagMutation::RenameNode(RenameNode { id: "a".into(), new_id: "aa".into() }));
        assert!(renamed.nodes.iter().any(|node| node.id == "aa"));
        assert_eq!(renamed.edges[0].source, "aa@out");
        assert_eq!(renamed.edges[0].target, "b@in");
    }

    #[test]
    fn delete_node_severs_and_reconnects_edges() {
        let mut document = empty_dag_document();
        document.nodes = vec![sample_node("a"), sample_node("b")];
        document.edges = vec![DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::from([("weight".to_string(), PropertyValue::Number(2.0))]) }];
        let deleted = round_trip(&document, &DagMutation::DeleteNode(DeleteNode { id: "a".into() }));
        assert!(deleted.nodes.iter().all(|node| node.id != "a"));
        assert!(deleted.edges.is_empty(), "the severed edge must be removed by the same delete-node diff, not left dangling");
    }

    #[test]
    fn reorder_nodes_round_trips() {
        let mut document = empty_dag_document();
        document.nodes = vec![sample_node("a"), sample_node("b"), sample_node("c")];
        let reordered = round_trip(&document, &DagMutation::ReorderNodes(ReorderNodes { order: vec!["c".into(), "a".into(), "b".into()] }));
        let ids: Vec<&str> = reordered.nodes.iter().map(|node| node.id.as_str()).collect();
        assert_eq!(ids, vec!["c", "a", "b"]);
        document.nodes = reordered.nodes;
    }

    #[test]
    fn connect_disconnect_nodes_round_trip() {
        let mut document = empty_dag_document();
        document.nodes = vec![sample_node("a"), sample_node("b")];
        let connected = round_trip(&document, &DagMutation::ConnectNodes(ConnectNodes { index: 0, id: "e1".into(), source: "a@out".into(), target: "b@in".into(), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::default() }));
        assert_eq!(connected.edges.len(), 1);
        let disconnected = round_trip(&connected, &DagMutation::DisconnectNodes(DisconnectNodes { id: "e1".into() }));
        assert!(disconnected.edges.is_empty());
    }

    //#region 🔖️MutationLaws
    #[test]
    fn diff_and_inverse_are_deterministic() {
        let document = round_trip(&empty_dag_document(), &DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
        let mutation = DagMutation::MoveNode(MoveNode { id: "n1".into(), x: 12.0, y: 34.0 });
        assert_eq!(Mutation::diff(&mutation, &document), Mutation::diff(&mutation, &document), "diff(payload, base) must be a pure function of its inputs");
        assert_eq!(mutation.inverse(&document), mutation.inverse(&document), "inverse(payload, base) must be a pure function of its inputs");
    }

    #[test]
    fn move_node_diff_is_consistent_with_direct_field_mutation() {
        let document = round_trip(&empty_dag_document(), &DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
        let mutation = DagMutation::MoveNode(MoveNode { id: "n1".into(), x: 5.0, y: 6.0 });
        let via_diff = Mutation::diff(&mutation, &document).diff().apply(&document).expect("valid DAG diff");
        let mut via_direct = document.clone();
        via_direct.nodes[0].x = 5.0;
        via_direct.nodes[0].y = 6.0;
        assert_eq!(via_diff, via_direct, "diff().apply() must match the mutation's own documented field-level effect");
    }

    #[test]
    fn move_node_diff_absorb_law_holds() {
        let document = round_trip(&empty_dag_document(), &DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
        let (mut d1, _) = Mutation::diff(&DagMutation::MoveNode(MoveNode { id: "n1".into(), x: 10.0, y: 10.0 }), &document).into_parts();
        let mid = d1.apply(&document).expect("valid first DAG diff");
        let (d2, _) = Mutation::diff(&DagMutation::MoveNode(MoveNode { id: "n1".into(), x: 20.0, y: 30.0 }), &mid).into_parts();
        d1.absorb(d2);
        let absorbed = d1.apply(&document).expect("valid absorbed DAG diff");
        assert_eq!(absorbed.nodes[0].x, 20.0, "absorb must converge to the LATER move, not the earlier one");
        assert_eq!(absorbed.nodes[0].y, 30.0);
    }

    #[test]
    fn missing_target_inverse_and_diff_are_no_ops() {
        let document = empty_dag_document();
        assert_eq!(Mutation::diff(&DagMutation::MoveNode(MoveNode { id: "ghost".into(), x: 1.0, y: 1.0 }), &document).diff(), &DagDiff::default());
        assert!(DagMutation::MoveNode(MoveNode { id: "ghost".into(), x: 1.0, y: 1.0 }).inverse(&document).is_empty());
        assert!(DagMutation::DeleteNode(DeleteNode { id: "ghost".into() }).inverse(&document).is_empty());
        assert!(DagMutation::DisconnectNodes(DisconnectNodes { id: "ghost".into() }).inverse(&document).is_empty());
    }
    //#endregion 🔖️MutationLaws

    //#region 🔖️DslTests
    /// 🧩️ One node per `DagNodeKind` tag (safe field values only — no raw JSON literals in
    /// `default`/`value`/`Tree.json`, see {@link json_to_property}'s docstring), so the DSL round trip
    /// exercises every kind-specific payload shape the wire-literal property bag needs to carry.
    fn kitchen_sink_snapshot() -> DagSnapshot {
        let port = |id: &str, label: &str| IoPortSpec::simple(id, label);
        let nodes = vec![
            DagNodeSpec {
                id: "comp".into(),
                name: "Comp".into(),
                abbreviation: "Cmp".into(),
                icon: "emoji:🧮️".into(),
                x: -120.0,
                y: -40.0,
                width: 104.0,
                height: 14.0,
                kind: DagNodeKind::Computation { inputs: vec![port("in", "In")], outputs: vec![port("out", "Out")], variadic_inputs: true, variadic_outputs: false },
                ..Default::default()
            },
            DagNodeSpec { id: "slider".into(), name: "Amount".into(), x: -400.0, y: -40.0, width: 70.0, height: 14.0, kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 5.0, output: port("out", "value") }, ..Default::default() },
            DagNodeSpec {
                id: "mode".into(),
                name: "Mode".into(),
                x: -400.0,
                y: 80.0,
                width: 56.0,
                height: 28.0,
                kind: DagNodeKind::Select { options: vec!["Add".into(), "Multiply".into()], selected: 1, output: port("out", "mode") },
                ..Default::default()
            },
            DagNodeSpec {
                id: "screen".into(),
                name: "Preview".into(),
                x: 400.0,
                y: 0.0,
                width: 200.0,
                height: 140.0,
                kind: DagNodeKind::Screen { media: Some(DagMedia { kind: DagMediaKind::Svg, src: "data:image/svg+xml,%3Csvg viewBox='0 0 1 1'%3E%3C/svg%3E".into() }), input: port("in", "result") },
                ..Default::default()
            },
            DagNodeSpec { id: "note".into(), name: "Note".into(), x: 0.0, y: 200.0, kind: DagNodeKind::Note { text: "line one\nline two — with a ' quote and a % sign".into(), output: port("out", "text") }, ..Default::default() },
            DagNodeSpec { id: "image".into(), name: "Image".into(), x: 0.0, y: 260.0, kind: DagNodeKind::Image { src: "data:image/png;base64,AAA=".into(), output: port("out", "img") }, ..Default::default() },
            DagNodeSpec {
                id: "preview".into(),
                name: "Preview2".into(),
                x: 0.0,
                y: 320.0,
                kind: DagNodeKind::Preview { content: DagPreviewContent::Scalar { text: "42".into() }, expanded: BTreeSet::from(["a.b".to_string()]), input: port("in", "value") },
                ..Default::default()
            },
            DagNodeSpec { id: "action".into(), name: "Action".into(), x: 0.0, y: 380.0, kind: DagNodeKind::Action { label: "Run".into(), input: port("in", "trigger") }, ..Default::default() },
            DagNodeSpec { id: "export".into(), name: "Export".into(), x: 0.0, y: 440.0, kind: DagNodeKind::Export { label: "Save".into(), format: "png".into(), input: port("in", "value") }, ..Default::default() },
            DagNodeSpec { id: "cluster".into(), name: "Cluster".into(), x: 0.0, y: 500.0, kind: DagNodeKind::Cluster { inputs: vec![port("in", "In")], outputs: vec![port("out", "Out")] }, ..Default::default() },
            DagNodeSpec {
                id: "app".into(),
                name: "App".into(),
                x: 0.0,
                y: 560.0,
                kind: DagNodeKind::AppInstance { instance_id: "inst-1".into(), plugin_id: "prog-1".into(), app_id: "note".into(), icon: "emoji:📦️".into(), inputs: vec![], outputs: vec![port("out", "Out")] },
                ..Default::default()
            },
        ];
        let edges = vec![
            DagFixtureEdge { id: "e1".into(), source: "slider@out".into(), target: "comp@in".into(), ..Default::default() },
            DagFixtureEdge { id: "e2".into(), source: "comp@out".into(), target: "screen@in".into(), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::from([("weight".to_string(), PropertyValue::Number(2.0))]) },
        ];
        DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), nodes, edges }
    }

    #[test]
    fn dag_document_dsl_round_trips_the_demo_fixture() {
        crate::os_store::test_support::assert_dsl_round_trip(&default_dag_document());
        crate::os_store::test_support::assert_dsl_pack_equivalence(&default_dag_document());
    }

    #[test]
    fn bundled_demo_fixture_is_canonical() {
        let actual = include_str!("../../../../../../../../✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        let expected = <DagSnapshot as crate::os_store::ArtifactDsl>::print_dsl(&default_dag_document());
        assert_eq!(actual, expected, "bundled demo fixture must stay in canonical owned DSL form");
    }

    #[test]
    fn dag_document_dsl_round_trips_every_node_kind() {
        crate::os_store::test_support::assert_dsl_round_trip(&kitchen_sink_snapshot());
        crate::os_store::test_support::assert_dsl_pack_equivalence(&kitchen_sink_snapshot());
    }

    #[test]
    fn dag_document_dsl_round_trips_the_empty_document() {
        crate::os_store::test_support::assert_dsl_round_trip(&empty_dag_document());
        crate::os_store::test_support::assert_dsl_pack_equivalence(&empty_dag_document());
    }

    #[test]
    fn op_text_round_trips_create_node() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
    }

    #[test]
    fn op_text_round_trips_delete_node() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::DeleteNode(DeleteNode { id: "n1".into() }));
    }

    #[test]
    fn op_text_round_trips_rename_node() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::RenameNode(RenameNode { id: "n1".into(), new_id: "n1-renamed".into() }));
    }

    #[test]
    fn op_text_round_trips_change_node_name() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ChangeNodeName(ChangeNodeName { id: "n1".into(), new_name: "Renamed".into() }));
    }

    #[test]
    fn op_text_round_trips_move_node() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::MoveNode(MoveNode { id: "n1".into(), x: 42.0, y: 7.0 }));
    }

    #[test]
    fn op_text_round_trips_resize_node() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ResizeNode(ResizeNode { id: "n1".into(), width: 200.0, height: 90.0 }));
    }

    #[test]
    fn op_text_round_trips_change_node_icon() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ChangeNodeIcon(ChangeNodeIcon { id: "n1".into(), new_icon: "emoji:🧪️".into() }));
    }

    #[test]
    fn op_text_round_trips_change_node_abbreviation() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ChangeNodeAbbreviation(ChangeNodeAbbreviation { id: "n1".into(), new_abbreviation: "N1".into() }));
    }

    #[test]
    fn op_text_round_trips_change_node_operator_kind_some_and_none() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ChangeNodeOperatorKind(ChangeNodeOperatorKind { id: "n1".into(), new_operator_kind: Some("math.add".into()) }));
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ChangeNodeOperatorKind(ChangeNodeOperatorKind { id: "n1".into(), new_operator_kind: None }));
    }

    #[test]
    fn op_text_round_trips_replace_node_kind() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ReplaceNodeKind(ReplaceNodeKind {
            id: "n1".into(),
            new_kind: DagNodeKind::Slider { min: 0.0, max: 1.0, step: 0.1, value: 0.5, output: IoPortSpec::simple("out", "value") },
        }));
    }

    #[test]
    fn op_text_round_trips_replace_node_properties() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ReplaceNodeProperties(ReplaceNodeProperties { id: "n1".into(), new_properties: PropertyBag::from([("weight".to_string(), PropertyValue::Number(2.0))]) }));
    }

    #[test]
    fn op_text_round_trips_reorder_nodes() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ReorderNodes(ReorderNodes { order: vec!["a".into(), "b".into(), "c".into()] }));
    }

    #[test]
    fn op_text_round_trips_connect_nodes() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ConnectNodes(ConnectNodes {
            index: 0,
            id: "e1".into(),
            source: "a@out".into(),
            target: "b@in".into(),
            route_style: EdgeRouteStyle::SharpSz,
            properties: PropertyBag::from([("weight".to_string(), PropertyValue::Number(2.0))]),
        }));
    }

    #[test]
    fn op_text_round_trips_disconnect_nodes() {
        crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::DisconnectNodes(DisconnectNodes { id: "e1".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn document_text_round_trips_a_store_with_an_applied_operation() {
        let mut store = DagStore::new(create_document_envelope(DAG_DOCUMENT_SCHEMA, "dag", kitchen_sink_snapshot(), None)).await.expect("store");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DagMutation::CreateNode(CreateNode { node: sample_node("extra"), index: 0 })], description: None }).await.expect("apply");
        crate::os_store::test_support::assert_document_text_round_trip(&store).await;
        crate::os_store::test_support::assert_document_pack_round_trip(&store).await;
    }

    /// 🎫️ Command envelopes preserve the aggregate's direct leaf-owned operation codecs.
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        let mut store = DagStore::new(create_document_envelope(DAG_DOCUMENT_SCHEMA, "dag", kitchen_sink_snapshot(), None)).await.expect("store");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DagMutation::CreateNode(CreateNode { node: sample_node("extra"), index: 0 })], description: None }).await.expect("apply");
        let envelope = store.envelope();
        let edit: &Edit<DagMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
        crate::os_store::test_support::assert_command_envelope_round_trip::<DagSnapshot, DagMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())).await;
    }
    //#endregion 🔖️DslTests
}
// #endregion 🔖️ArtifactVcs

#[cfg(test)]
mod dag_direct_tests {
    use super::*;
    use protocol::{MutationLeaf, OpBinary, OpText, SemanticMutation};

    fn fixture() -> Value {
        dsl::os_pack::json::parse(include_str!("🧪️fixtures/🔣️mutations.json")).expect("neutral Dag mutation fixture")
    }

    /// 🌉️ `T: FromValue` decode of a pack JSON [`Value`] — the in-house `serde_json::from_value` analog.
    fn from_pack_value<T: dsl::FromValue>(value: Value) -> Result<T, dsl::ValueError> {
        <T as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(&value))
    }

    /// 🌉️ `T: ToValue` encode into a pack JSON [`Value`] — the in-house `serde_json::to_value` analog.
    fn to_pack_value<T: dsl::ToValue>(value: &T) -> Value {
        dsl::os_pack::json::from_dsl_value(&<T as dsl::ToValue>::to_value(value))
    }

    fn node(id: &str) -> DagNodeSpec {
        DagNodeSpec { id: id.into(), name: id.into(), ..Default::default() }
    }

    fn base() -> DagSnapshot {
        DagSnapshot {
            schema: DAG_DOCUMENT_SCHEMA.into(),
            nodes: vec![node("a"), node("b"), node("c")],
            edges: vec![
                DagFixtureEdge { id: "e".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() },
                DagFixtureEdge { id: "keep".into(), source: "b@out".into(), target: "c@in".into(), ..Default::default() },
                DagFixtureEdge { id: "last".into(), source: "c@out".into(), target: "a@in".into(), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::from([("weight".into(), PropertyValue::Number(2.0))]) },
            ],
        }
    }

    fn apply(base: &DagSnapshot, mutation: &DagMutation) -> DagSnapshot {
        mutation.diff(base).diff().apply(base).expect("valid direct Dag mutation")
    }

    fn assert_codecs(mutation: &DagMutation) {
        let json = dsl::os_pack::json::to_json_string(mutation);
        assert_eq!(dsl::os_pack::json::from_json_str::<DagMutation>(&json).expect("deserialize direct mutation"), *mutation);
        let text = mutation.print_op();
        assert!(text.starts_with(mutation.descriptor().text_opcode.expect("text opcode")));
        assert_eq!(DagMutation::parse_op(&text).expect("direct text decode"), *mutation);
        let bytes = mutation.encode_op().expect("direct binary encode");
        assert_eq!(bytes[0], dsl::variants_binary::OP_BINARY_FORMAT);
        assert_eq!(u32::from(bytes[1]), mutation.descriptor().binary_tag.expect("binary tag"));
        assert_eq!(DagMutation::decode_op(&bytes).expect("direct binary decode"), *mutation);
    }

    pub(crate) fn assert_leaf_contract<T>(index: usize, wrap: fn(T) -> DagMutation, descriptor: &str)
    where
        T: MutationLeaf + dsl::ToValue + dsl::FromValue,
    {
        let fixture = fixture();
        let row = &fixture["valid"][index];
        let payload = from_pack_value::<T>(row["payload"].clone()).expect("neutral direct payload");
        let mutation = wrap(payload);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::os_pack::json::to_json_string(&T::DESCRIPTOR)).expect("descriptor JSON"), serde_json::from_str::<serde_json::Value>(descriptor).expect("owned descriptor"));
        assert_eq!(mutation.descriptor(), &T::DESCRIPTOR);
        assert_eq!(mutation.descriptor().binary_tag, Some(u32::try_from(index).expect("small roster index")));
        assert_eq!(to_pack_value(&mutation)["operation"], row["operation"]);
        let mut unknown_payload = row["payload"].clone();
        if let Some(object) = unknown_payload.as_object_mut() {
            object.insert("unknown".to_string(), Value::from(true));
        }
        assert!(from_pack_value::<T>(unknown_payload).is_err());
        let mut unknown_operation = to_pack_value(&mutation);
        if let Some(object) = unknown_operation.as_object_mut() {
            object.insert("unknown".to_string(), Value::from(true));
        }
        assert!(from_pack_value::<DagMutation>(unknown_operation).is_err());
        let payload_object = row["payload"].as_object().expect("payload object");
        let payload_keys: Vec<String> = payload_object.iter().map(|(key, _)| key.to_string()).filter(|key| key != "newOperatorKind").collect();
        for key in payload_keys {
            let missing: Value = Value::Object(payload_object.iter().filter(|(k, _)| *k != key).map(|(k, v)| (k.to_string(), v.clone())).collect());
            assert!(from_pack_value::<T>(missing.clone()).is_err(), "missing {key}");
            let mut missing_aggregate = missing;
            if let Some(object) = missing_aggregate.as_object_mut() {
                object.insert("operation".to_string(), row["operation"].clone());
            }
            assert!(from_pack_value::<DagMutation>(missing_aggregate).is_err(), "missing aggregate {key}");
        }
        assert_codecs(&mutation);
        let before = base();
        let mut restored = apply(&before, &mutation);
        let inverse = mutation.inverse(&before);
        assert!(!inverse.is_empty());
        for inverse in inverse.into_iter().rev() {
            restored = apply(&restored, &inverse);
        }
        assert_eq!(restored, before);
    }

    #[test]
    fn direct_leaf_roster_and_codec_contracts() {
        let fixture = fixture();
        assert_eq!(DagMutation::kinds().len(), 14);
        assert_eq!(<DagMutation as Mutation<DagSnapshot>>::DESCRIPTORS.len(), 14);
        for (index, row) in fixture["valid"].as_array().expect("valid vectors").iter().enumerate() {
            let mut json = row["payload"].clone();
            if let Some(object) = json.as_object_mut() {
                object.insert("operation".to_string(), row["operation"].clone());
            }
            let mutation = from_pack_value::<DagMutation>(json).expect("neutral aggregate");
            assert_eq!(mutation.descriptor().binary_tag, Some(u32::try_from(index).expect("small index")));
            assert_eq!(mutation.descriptor().diff_participation, protocol::MutationDiffParticipation::ApplyOnly);
            assert_codecs(&mutation);
        }
        for row in fixture["invalid"].as_array().expect("invalid vectors") {
            let mut json = row["payload"].clone();
            if let Some(object) = json.as_object_mut() {
                object.insert("operation".to_string(), row["operation"].clone());
            }
            assert!(from_pack_value::<DagMutation>(json).is_err(), "{}", row["name"]);
        }
        for row in fixture["additionalValid"].as_array().expect("additional input vectors") {
            let mut json = row["payload"].clone();
            if let Some(object) = json.as_object_mut() {
                object.insert("operation".to_string(), row["operation"].clone());
            }
            assert_codecs(&from_pack_value::<DagMutation>(json).expect("additional pack-value input"));
        }
    }

    #[test]
    fn direct_delete_inverse_declares_descending_edges_before_node() {
        let mut before = base();
        before.edges.push(DagFixtureEdge { id: "loop".into(), source: "a@out".into(), target: "a@in".into(), ..Default::default() });
        let mutation = DagMutation::DeleteNode(DeleteNode { id: "a".into() });
        let inverse = mutation.inverse(&before);
        assert_eq!(inverse.len(), 4);
        assert!(matches!(&inverse[0], DagMutation::ConnectNodes(value) if value.id == "loop" && value.index == 3));
        assert!(matches!(&inverse[1], DagMutation::ConnectNodes(value) if value.id == "last" && value.index == 2));
        assert!(matches!(&inverse[2], DagMutation::ConnectNodes(value) if value.id == "e" && value.index == 0));
        assert!(matches!(&inverse[3], DagMutation::CreateNode(value) if value.node.id == "a" && value.index == 0));
        let mut restored = apply(&before, &mutation);
        for inverse in inverse.into_iter().rev() {
            restored = apply(&restored, &inverse);
        }
        assert_eq!(restored, before);
    }

    #[semio_framework_async_macros::async_test]
    async fn direct_store_undo_restores_incident_edge_order() {
        let before = base();
        let mut store = DagStore::new(create_document_envelope(DAG_DOCUMENT_SCHEMA, "dag", before.clone(), None)).await.expect("store");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DagMutation::DeleteNode(DeleteNode { id: "a".into() })], description: None }).await.expect("delete");
        store.dispatch(ArtifactCommand::Undo).await.expect("undo");
        assert_eq!(store.snapshot().expect("restored projection"), before);
        store.dispatch(ArtifactCommand::Redo).await.expect("redo");
        assert_eq!(store.snapshot().expect("deleted projection"), apply(&before, &DagMutation::DeleteNode(DeleteNode { id: "a".into() })));
    }

    #[test]
    fn direct_disconnect_inverse_preserves_nonfinal_position() {
        let before = base();
        let mutation = DagMutation::DisconnectNodes(DisconnectNodes { id: "keep".into() });
        let inverse = mutation.inverse(&before);
        assert!(matches!(&inverse[0], DagMutation::ConnectNodes(value) if value.id == "keep" && value.index == 1));
        assert_eq!(apply(&apply(&before, &mutation), &inverse[0]), before);
    }

    #[test]
    fn direct_rename_preserves_exact_endpoint_suffix() {
        let fixture = fixture();
        for row in fixture["endpointRenames"].as_array().expect("endpoint vectors") {
            let id = row["id"].as_str().expect("id");
            let new_id = row["newId"].as_str().expect("new ID");
            let source = row["source"].as_str().expect("source");
            let expected = row["expected"].as_str().expect("expected endpoint");
            let before = DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), nodes: vec![node(id), node("b")], edges: vec![DagFixtureEdge { id: "edge".into(), source: source.into(), target: "b@in".into(), ..Default::default() }] };
            let mutation = DagMutation::RenameNode(RenameNode { id: id.into(), new_id: new_id.into() });
            let after = apply(&before, &mutation);
            assert_eq!(after.edges[0].source, expected);
            assert_eq!(apply(&after, &mutation.inverse(&before)[0]), before);
        }
    }

    #[test]
    fn direct_structural_absorb_is_associative_and_preserves_rejection() {
        let fixture = fixture();
        for row in fixture["algebra"].as_array().expect("algebra vectors") {
            let before = base();
            let mut after = before.clone();
            let mut diffs = Vec::new();
            for value in row["mutations"].as_array().expect("mutation sequence") {
                let mutation = from_pack_value::<DagMutation>(value.clone()).expect("sequence mutation");
                let (diff, _) = mutation.diff(&after).into_parts();
                after = diff.apply(&after).expect("sequential diff");
                diffs.push(diff);
            }
            let mut left = DagDiff::default();
            for diff in &diffs {
                left.absorb(diff.clone());
            }
            let mut right = DagDiff::default();
            for mut diff in diffs.into_iter().rev() {
                diff.absorb(right);
                right = diff;
            }
            assert_eq!(left, right, "{}", row["name"]);
            assert_eq!(left.apply(&before).expect("absorbed diff"), after, "{}", row["name"]);
            assert_eq!(to_pack_value(&after.nodes.iter().map(|node| node.id.clone()).collect::<Vec<_>>()), row["nodeOrder"]);
            assert_eq!(to_pack_value(&after.edges.iter().map(|edge| edge.id.clone()).collect::<Vec<_>>()), row["edgeOrder"]);
            for (id, x) in row["x"].as_object().expect("expected positions") {
                assert_eq!(after.nodes.iter().find(|node| node.id == id).expect("position target").x, x.as_f64().expect("x"));
            }
            assert_eq!(dsl::os_pack::json::from_json_str::<DagDiff>(&dsl::os_pack::json::to_json_string(&left)).expect("diff decode"), left);
        }
        let before = base();
        let mut rejected = DagDiff::from(DagDelta { created_node: Some(node("x")), created_node_at: Some(u64::MAX), ..Default::default() });
        rejected.absorb(DagMutation::MoveNode(MoveNode { id: "a".into(), x: 3.0, y: 4.0 }).diff(&before).into_parts().0);
        assert_eq!(rejected.apply(&before).expect_err("rejection survives composition").code, "mutation.apply.invalid-index");
        assert_eq!(before, base());
        assert_eq!(DagDiff::from(DagDelta { connected_edge: Some(before.edges[0].clone()), ..Default::default() }).apply(&before).expect_err("unpaired edge index").code, "mutation.apply.incomplete-diff");
    }

    #[test]
    fn direct_wire_indices_are_exact_and_apply_rejects_out_of_range() {
        let before = base();
        assert_eq!(dag_index_to_wire(usize::MAX), u64::try_from(usize::MAX).expect("guarded native width"));
        if usize::BITS < u64::BITS {
            assert_eq!(dag_index_from_wire(u64::MAX).expect_err("narrow native width").code, "mutation.apply.invalid-index");
        }
        for mutation in [
            DagMutation::CreateNode(CreateNode { node: node("x"), index: u64::MAX }),
            DagMutation::ConnectNodes(ConnectNodes { id: "x".into(), source: "a@out".into(), target: "b@in".into(), route_style: EdgeRouteStyle::Bezier, properties: PropertyBag::new(), index: u64::MAX }),
        ] {
            assert_codecs(&mutation);
            assert!(dsl::os_pack::json::to_json_string(&mutation).contains("18446744073709551615"));
            assert!(mutation.print_op().contains("18446744073709551615"));
            assert_eq!(mutation.diff(&before).diff().apply(&before).expect_err("out of range").code, "mutation.apply.invalid-index");
            let json = dsl::os_pack::json::to_json_string(&mutation);
            for invalid in ["18446744073709551616", "-1", "0.5", "1e21", "null", "\"1\""] {
                assert!(dsl::os_pack::json::from_json_str::<DagMutation>(&json.replace("18446744073709551615", invalid)).is_err(), "{invalid}");
            }
        }
    }

    #[test]
    fn direct_intrinsic_serde_and_selection_are_lossless() {
        let fixture = fixture();
        for row in fixture["nodeKinds"].as_array().expect("node kind vectors") {
            let value = row["value"].clone();
            assert_eq!(from_pack_value::<DagNodeKind>(value.clone()).is_ok(), row["valid"].as_bool().expect("expected validity"), "{}", row["name"]);
            if row["valid"] == true {
                let kind = from_pack_value::<DagNodeKind>(value).expect("kind");
                assert_codecs(&DagMutation::ReplaceNodeKind(ReplaceNodeKind { id: "a".into(), new_kind: kind }));
            }
        }
        let mut selected = node("select");
        selected.kind = DagNodeKind::Select { options: vec!["a".into(), "b".into()], selected: u64::MAX, output: IoPortSpec::simple("out", "Output") };
        assert_codecs(&DagMutation::CreateNode(CreateNode { node: selected.clone(), index: 0 }));
        assert_eq!(advance_select_option(&mut selected).as_deref(), Some("a"));
        selected.kind = DagNodeKind::Select { options: vec![], selected: u64::MAX, output: IoPortSpec::simple("out", "Output") };
        assert_eq!(advance_select_option(&mut selected), None);
        let mut app = node("app");
        app.icon = "node-icon".into();
        app.kind = DagNodeKind::AppInstance { instance_id: "instance".into(), plugin_id: "plugin".into(), app_id: "app".into(), icon: "app-icon".into(), inputs: vec![], outputs: vec![] };
        let encoded = dsl::os_pack::json::to_json_string(&app);
        assert_eq!(encoded.matches("\"icon\":").count(), 1);
        assert_eq!(encoded.matches("\"appIcon\":").count(), 1);
        assert_eq!(dsl::os_pack::json::from_json_str::<DagNodeSpec>(&encoded).expect("app round trip"), app);
        assert_codecs(&DagMutation::CreateNode(CreateNode { node: app, index: 0 }));
    }
}
