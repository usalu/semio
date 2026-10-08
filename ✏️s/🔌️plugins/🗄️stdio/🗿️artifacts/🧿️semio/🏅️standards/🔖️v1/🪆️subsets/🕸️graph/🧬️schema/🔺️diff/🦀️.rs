//! 🔺️ SemioGraphDiff — sparse keyed-row diff over `SemioGraphSnapshot`. `graph` has exactly two mutable top-level fields
//! (`nodes`/`edges`), so the diff carries two index-keyed triples: removed base indices, modified rows (a sparse
//! [`SemioGraphNodeDiff`] / [`SemioGraphEdgeDiff`] per row) and added rows with their final position. A node rename or delete
//! that must reach the edges names exactly the touched edge rows — never a rebuilt list. No `snapshot:
//! Option<SemioGraphSnapshot>` full-replace slot anywhere — whole-document replace is `ArtifactStore::reset`, outside history.

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{absorb_indexed_slot, apply_indexed_rows, between_indexed_rows, inverse_indexed_rows, validate_indexed_triple, IndexedRow, IndexedTripleDiff, Replace};
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphPort, SemioGraphSnapshot};
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️PropertyDiff
/// 🏷️ Sparse diff of one property entry: its key is the row's identity and never changes, only its value does.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioGraphEntryDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<SemioValue>,
}

impl IndexedRow<SemioValueEntry> for SemioGraphEntryDiff {
    fn apply_row(&self, base: &SemioValueEntry) -> SemioValueEntry {
        SemioValueEntry { key: base.key.clone(), value: self.value.clone().unwrap_or_else(|| base.value.clone()) }
    }
    fn inverse_row(&self, base: &SemioValueEntry) -> Self {
        Self { value: self.value.as_ref().map(|_| base.value.clone()) }
    }
    fn absorb_row(&mut self, other: Self) {
        if other.value.is_some() {
            self.value = other.value;
        }
    }
    fn row_is_empty(&self) -> bool {
        self.value.is_none()
    }
    fn between_row(base: &SemioValueEntry, other: &SemioValueEntry) -> Self {
        Self { value: (base.value != other.value).then(|| other.value.clone()) }
    }
}

/// 🏷️ Index-keyed property rows shared by nodes and edges.
pub type SemioGraphPropertiesDiff = IndexedTripleDiff<SemioGraphEntryDiff, SemioValueEntry>;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_properties(properties: &Option<SemioGraphPropertiesDiff>, base_len: usize, target: Vec<String>) -> protocol::MutationApplyResult<()> {
    match properties {
        Some(properties) => validate_indexed_triple(properties, base_len, target),
        None => Ok(()),
    }
}
//#endregion 🔖️PropertyDiff

//#region 🔖️NodeDiff
/// 🔘 Sparse diff of one node: each present field is the new value; `ports`/`properties` nest index-keyed triples.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioGraphNodeDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GraphNodeId>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<SemioPoint2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<IndexedTripleDiff<Replace<SemioGraphPort>, SemioGraphPort>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SemioGraphPropertiesDiff>,
}

impl IndexedRow<SemioGraphNode> for SemioGraphNodeDiff {
    fn apply_row(&self, base: &SemioGraphNode) -> SemioGraphNode {
        SemioGraphNode {
            id: self.id.clone().unwrap_or_else(|| base.id.clone()),
            kind: self.kind.clone().unwrap_or_else(|| base.kind.clone()),
            label: self.label.clone().unwrap_or_else(|| base.label.clone()),
            position: self.position.unwrap_or(base.position),
            width: self.width.unwrap_or(base.width),
            height: self.height.unwrap_or(base.height),
            ports: self.ports.as_ref().map_or_else(|| base.ports.clone(), |ports| apply_indexed_rows(ports, &base.ports)),
            properties: self.properties.as_ref().map_or_else(|| base.properties.clone(), |properties| apply_indexed_rows(properties, &base.properties)),
        }
    }
    fn inverse_row(&self, base: &SemioGraphNode) -> Self {
        Self {
            id: self.id.as_ref().map(|_| base.id.clone()),
            kind: self.kind.as_ref().map(|_| base.kind.clone()),
            label: self.label.as_ref().map(|_| base.label.clone()),
            position: self.position.map(|_| base.position),
            width: self.width.map(|_| base.width),
            height: self.height.map(|_| base.height),
            ports: self.ports.as_ref().map(|ports| inverse_indexed_rows(ports, &base.ports)),
            properties: self.properties.as_ref().map(|properties| inverse_indexed_rows(properties, &base.properties)),
        }
    }
    fn absorb_row(&mut self, other: Self) {
        if other.id.is_some() {
            self.id = other.id;
        }
        if other.kind.is_some() {
            self.kind = other.kind;
        }
        if other.label.is_some() {
            self.label = other.label;
        }
        if other.position.is_some() {
            self.position = other.position;
        }
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        absorb_indexed_slot(&mut self.ports, other.ports);
        absorb_indexed_slot(&mut self.properties, other.properties);
    }
    fn row_is_empty(&self) -> bool {
        self.id.is_none()
            && self.kind.is_none()
            && self.label.is_none()
            && self.position.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.ports.as_ref().is_none_or(IndexedTripleDiff::is_unchanged)
            && self.properties.as_ref().is_none_or(IndexedTripleDiff::is_unchanged)
    }
    fn between_row(base: &SemioGraphNode, other: &SemioGraphNode) -> Self {
        let ports = between_indexed_rows(&base.ports, &other.ports);
        let properties = between_indexed_rows(&base.properties, &other.properties);
        Self {
            id: (base.id != other.id).then(|| other.id.clone()),
            kind: (base.kind != other.kind).then(|| other.kind.clone()),
            label: (base.label != other.label).then(|| other.label.clone()),
            position: (base.position != other.position).then_some(other.position),
            width: (base.width != other.width).then_some(other.width),
            height: (base.height != other.height).then_some(other.height),
            ports: (!ports.is_unchanged()).then_some(ports),
            properties: (!properties.is_unchanged()).then_some(properties),
        }
    }
}
//#endregion 🔖️NodeDiff

//#region 🔖️EdgeDiff
/// 🔗️ Sparse diff of one edge: each present field is the new value; `properties` nests an index-keyed triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioGraphEdgeDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<GraphNodeId>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<GraphNodeId>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source_port: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target_port: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SemioGraphPropertiesDiff>,
}

impl IndexedRow<SemioGraphEdge> for SemioGraphEdgeDiff {
    fn apply_row(&self, base: &SemioGraphEdge) -> SemioGraphEdge {
        SemioGraphEdge {
            id: base.id.clone(),
            source: self.source.clone().unwrap_or_else(|| base.source.clone()),
            target: self.target.clone().unwrap_or_else(|| base.target.clone()),
            kind: self.kind.clone().unwrap_or_else(|| base.kind.clone()),
            label: self.label.clone().unwrap_or_else(|| base.label.clone()),
            source_port: self.source_port.clone().unwrap_or_else(|| base.source_port.clone()),
            target_port: self.target_port.clone().unwrap_or_else(|| base.target_port.clone()),
            properties: self.properties.as_ref().map_or_else(|| base.properties.clone(), |properties| apply_indexed_rows(properties, &base.properties)),
        }
    }
    fn inverse_row(&self, base: &SemioGraphEdge) -> Self {
        Self {
            source: self.source.as_ref().map(|_| base.source.clone()),
            target: self.target.as_ref().map(|_| base.target.clone()),
            kind: self.kind.as_ref().map(|_| base.kind.clone()),
            label: self.label.as_ref().map(|_| base.label.clone()),
            source_port: self.source_port.as_ref().map(|_| base.source_port.clone()),
            target_port: self.target_port.as_ref().map(|_| base.target_port.clone()),
            properties: self.properties.as_ref().map(|properties| inverse_indexed_rows(properties, &base.properties)),
        }
    }
    fn absorb_row(&mut self, other: Self) {
        if other.source.is_some() {
            self.source = other.source;
        }
        if other.target.is_some() {
            self.target = other.target;
        }
        if other.kind.is_some() {
            self.kind = other.kind;
        }
        if other.label.is_some() {
            self.label = other.label;
        }
        if other.source_port.is_some() {
            self.source_port = other.source_port;
        }
        if other.target_port.is_some() {
            self.target_port = other.target_port;
        }
        absorb_indexed_slot(&mut self.properties, other.properties);
    }
    fn row_is_empty(&self) -> bool {
        self.source.is_none()
            && self.target.is_none()
            && self.kind.is_none()
            && self.label.is_none()
            && self.source_port.is_none()
            && self.target_port.is_none()
            && self.properties.as_ref().is_none_or(IndexedTripleDiff::is_unchanged)
    }
    fn between_row(base: &SemioGraphEdge, other: &SemioGraphEdge) -> Self {
        let properties = between_indexed_rows(&base.properties, &other.properties);
        Self {
            source: (base.source != other.source).then(|| other.source.clone()),
            target: (base.target != other.target).then(|| other.target.clone()),
            kind: (base.kind != other.kind).then(|| other.kind.clone()),
            label: (base.label != other.label).then(|| other.label.clone()),
            source_port: (base.source_port != other.source_port).then(|| other.source_port.clone()),
            target_port: (base.target_port != other.target_port).then(|| other.target_port.clone()),
            properties: (!properties.is_unchanged()).then_some(properties),
        }
    }
}
//#endregion 🔖️EdgeDiff

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.graph.diff")]
pub struct SemioGraphDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<IndexedTripleDiff<SemioGraphNodeDiff, SemioGraphNode>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edges: Option<IndexedTripleDiff<SemioGraphEdgeDiff, SemioGraphEdge>>,
}

impl SemioGraphDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.nodes.as_ref().is_none_or(IndexedTripleDiff::is_unchanged) && self.edges.as_ref().is_none_or(IndexedTripleDiff::is_unchanged)
    }
}

impl MutationDiff<SemioGraphSnapshot> for SemioGraphDiff {
    fn apply(&self, base: &SemioGraphSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioGraphSnapshot> {
        let mut next = base.clone();
        if let Some(nodes) = &self.nodes {
            validate_indexed_triple(nodes, base.nodes.len(), ["nodes"])?;
            for modified in &nodes.modified {
                let node = &base.nodes[modified.index];
                let at = |field: &str| vec!["nodes".to_string(), modified.index.to_string(), field.to_string()];
                if let Some(ports) = &modified.diff.ports {
                    validate_indexed_triple(ports, node.ports.len(), at("ports"))?;
                }
                validate_properties(&modified.diff.properties, node.properties.len(), at("properties"))?;
            }
            next.nodes = apply_indexed_rows(nodes, &base.nodes);
        }
        if let Some(edges) = &self.edges {
            validate_indexed_triple(edges, base.edges.len(), ["edges"])?;
            for modified in &edges.modified {
                validate_properties(&modified.diff.properties, base.edges[modified.index].properties.len(), vec!["edges".to_string(), modified.index.to_string(), "properties".to_string()])?;
            }
            next.edges = apply_indexed_rows(edges, &base.edges);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        absorb_indexed_slot(&mut self.nodes, other.nodes);
        absorb_indexed_slot(&mut self.edges, other.edges);
    }
}

/// 🧮️ `graph`'s own `DiffAlgebra` — required by the `✉️base` envelope's own dispatch. `inverse` is the concrete negative diff of
/// the keyed rows; `between` is the positional sync/import delta, never used by mutation leaves.
impl protocol::command::DiffAlgebra<SemioGraphSnapshot> for SemioGraphDiff {
    fn between(base: &SemioGraphSnapshot, other: &SemioGraphSnapshot) -> Self {
        let nodes = between_indexed_rows(&base.nodes, &other.nodes);
        let edges = between_indexed_rows(&base.edges, &other.edges);
        SemioGraphDiff { nodes: (!nodes.is_unchanged()).then_some(nodes), edges: (!edges.is_unchanged()).then_some(edges) }
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Self {
        SemioGraphDiff { nodes: self.nodes.as_ref().map(|nodes| inverse_indexed_rows(nodes, &base.nodes)), edges: self.edges.as_ref().map(|edges| inverse_indexed_rows(edges, &base.edges)) }
    }
    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
















//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioGraphDiff` cases — single source of truth for `diff_grammar_conformance_
/// law`/`protocol_walk_law` in `🚪️io/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioGraphDiff> {
    use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified};
    use crate::standards::v1::subsets::graph::schema::snapshot::demo_graph_snapshot;
    let demo = demo_graph_snapshot();
    let added = |items: Vec<_>| items.into_iter().enumerate().map(|(index, item)| IndexAdded { index, item }).collect::<Vec<_>>();
    vec![
        SemioGraphDiff::default(),
        SemioGraphDiff { nodes: Some(IndexedTripleDiff { added: added(demo.nodes.clone()), ..Default::default() }), edges: None },
        SemioGraphDiff { nodes: None, edges: Some(IndexedTripleDiff { added: added(demo.edges.clone()), ..Default::default() }) },
        SemioGraphDiff {
            nodes: Some(IndexedTripleDiff {
                removed: vec![1],
                modified: vec![IndexModified {
                    index: 0,
                    diff: SemioGraphNodeDiff {
                        id: Some(GraphNodeId::new("renamed")),
                        label: Some("relabelled".into()),
                        position: Some(SemioPoint2 { x: 1.5, y: -2.0 }),
                        width: Some(40.0),
                        properties: Some(IndexedTripleDiff { modified: vec![IndexModified { index: 0, diff: SemioGraphEntryDiff { value: Some(SemioValue::Null) } }], ..Default::default() }),
                        ..Default::default()
                    },
                }],
                ..Default::default()
            }),
            edges: Some(IndexedTripleDiff {
                removed: vec![0],
                modified: vec![IndexModified { index: 1, diff: SemioGraphEdgeDiff { source: Some(GraphNodeId::new("renamed")), source_port: Some(None), ..Default::default() } }],
                ..Default::default()
            }),
        },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
