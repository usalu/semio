//! 🧬️ DAG artifact schema — every field of the artifact with its state class.

use crate::mutations::delete_node;
use crate::op::DagMutation;
use crate::{DagContentChild, DagNodeKind, DagNodePatch, DagPreviewContent, DagSnapshot, IoPortSpec};
use framework_schema::ArtifactSchema;
use infinite_board_port_directed_dag::{fit_node_size, note_widget_size, preview_widget_size, would_create_cycle};
use std::collections::BTreeSet;
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};
//#region 🔖️Artifact
/// 🧬️ DAG document artifact state.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: DagContentChild,
}

impl dsl::FromValue for DagArtifact {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        <crate::DagSnapshot as dsl::FromValue>::from_value(value).map(Self::from_snapshot)
    }
}

//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DagArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::default_snapshot())
    }
}

impl DagArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> DagSnapshot {
        DagSnapshot { schema: self.schema.clone(), content: self.content.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: DagSnapshot) -> Self {
        Self { schema: snapshot.schema, content: snapshot.content }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: DagSnapshot) {
        self.schema = snapshot.schema;
        self.content = snapshot.content;
    }

    /// 📸️ Live nodes/edges off this artifact's `content` child — mirrors `DagSnapshot::nodes`/`edges`.
    pub fn nodes(&self) -> Vec<DagNodeSpec> {
        self.to_snapshot().nodes()
    }
    pub fn edges(&self) -> Vec<DagFixtureEdge> {
        self.to_snapshot().edges()
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.dag.dag` — twenty handcrafted schema leaves.
pub fn dag_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.dag.dag",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️Construction
/// 🏗️ This subset needs no custom build/analysis/composition logic beyond the ordinary
/// `Mutation`/`MutationDiff` algebra — the generic `SnapshotBuilder<S, M>` (W1-C task 3's
/// trivial-subset shape) replaces the old hand-rolled `DagBuilderConstruction` +
/// `DagAnalyzerAnalysis` + `derive_artifact_facets!`-generated `DagBuilder`/`DagAnalyzer`/
/// `DagComposer` outright. All io now goes exclusively through the `io_mechanism` registry
/// (`🚪️io/🦀️.rs`'s `io()`), replacing the old `DagComposerComposition`/`io_registry`.
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<DagSnapshot, DagMutation>;
//#endregion 🏗️Construction

//#region ⚠️ Errors
/// ⚠️ Errors from DAG play app edge-connection building. Relocated from the deleted `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — travels with `connect_edge`, the
/// only function that returns it.
#[derive(Debug)]
pub enum DagPlayError {
    CycleDetected,
}

impl std::fmt::Display for DagPlayError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CycleDetected => formatter.write_str("connection would create cycle"),
        }
    }
}

impl std::error::Error for DagPlayError {}
//#endregion ⚠️ Errors

//#region 🔖️DocumentHelpers
/// 🔀️ Pure document helpers over `DagSnapshot`/`DagNodeSpec`. Relocated from the deleted `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — none of these take an app-runtime
/// parameter, so per the region → destination map they belong beside the schema types they operate on.
pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

pub fn document_to_workflow(document: &DagSnapshot) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let scene = crate::dag_working_scene(document);
    let nodes: Vec<NodeGraphNodeRecord> = scene
        .nodes
        .iter()
        .map(|node| NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = scene
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    (nodes, edges)
}

pub fn next_node_id(document: &DagSnapshot) -> String {
    let max = document.nodes().iter().filter_map(|node| node.id.strip_prefix('n').and_then(|suffix| suffix.parse::<u64>().ok())).max().unwrap_or(0);
    format!("n{}", max + 1)
}

pub fn default_node_for_kind(kind: &str, id: &str, x: f64, y: f64) -> DagNodeSpec {
    let mut node = match kind {
        "slider" => DagNodeSpec {
            id: id.into(),
            name: "Slider".into(),
            abbreviation: "Sld".into(),
            icon: "emoji:🎚️".into(),
            x,
            y,
            kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.1, value: 3.0, output: IoPortSpec::named("N", "Num", "number", "Number") },
            ..Default::default()
        },
        "select" => DagNodeSpec {
            id: id.into(),
            name: "Select".into(),
            abbreviation: "Sel".into(),
            icon: "emoji:📋️".into(),
            x,
            y,
            kind: DagNodeKind::Select { options: vec!["A".into(), "B".into(), "C".into()], selected: 0, output: IoPortSpec::named("V", "Val", "value", "Value") },
            ..Default::default()
        },
        "screen" => {
            DagNodeSpec { id: id.into(), name: "Screen".into(), abbreviation: "Scr".into(), icon: "emoji:🖥️".into(), x, y, kind: DagNodeKind::Screen { media: None, input: IoPortSpec::named("I", "In", "in", "Input") }, ..Default::default() }
        }
        "note" => {
            let text = String::new();
            let (width, height) = note_widget_size(&text);
            DagNodeSpec {
                id: id.into(), name: "Note".into(), abbreviation: "Note".into(), icon: "emoji:📝️".into(), x, y, width, height, kind: DagNodeKind::Note { text, output: IoPortSpec::named("T", "Txt", "text", "Text") }, ..Default::default()
            }
        }
        "preview" => {
            let (width, height) = preview_widget_size(&DagPreviewContent::Scalar { text: String::new() }, &BTreeSet::new());
            DagNodeSpec {
                id: id.into(),
                name: "Preview".into(),
                abbreviation: "Prv".into(),
                icon: "emoji:👁️".into(),
                x,
                y,
                width,
                height,
                kind: DagNodeKind::Preview { content: DagPreviewContent::Scalar { text: String::new() }, expanded: BTreeSet::new(), input: IoPortSpec::named("I", "In", "in", "Input") },
                ..Default::default()
            }
        }
        _ => DagNodeSpec {
            id: id.into(),
            name: "Computation".into(),
            abbreviation: "Cmp".into(),
            icon: "emoji:⚙️".into(),
            x,
            y,
            operator_kind: Some("math.add".into()),
            kind: DagNodeKind::Computation {
                inputs: vec![IoPortSpec::named("A", "A", "a", "A"), IoPortSpec::named("B", "B", "b", "B")],
                outputs: vec![IoPortSpec::named("R", "R", "result", "Result")],
                variadic_inputs: false,
                variadic_outputs: false,
            },
            ..Default::default()
        },
    };
    fit_node_size(&mut node);
    node
}

/// 🔗️ Builds the `DagFixtureEdge` connecting two ports, or `Err` if it would introduce a cycle.
pub fn connect_edge(document: &DagSnapshot, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> Result<DagFixtureEdge, DagPlayError> {
    let edges = document.edges();
    let existing: Vec<(String, String)> = edges
        .iter()
        .map(|edge| {
            let (from, _) = split_endpoint(&edge.source);
            let (to, _) = split_endpoint(&edge.target);
            (from, to)
        })
        .collect();
    if would_create_cycle(&existing, source_node_id, target_node_id) {
        return Err(DagPlayError::CycleDetected);
    }
    let edge_id = format!("e{}", edges.iter().filter_map(|edge| edge.id.strip_prefix('e').and_then(|suffix| suffix.parse::<u64>().ok())).max().unwrap_or(0) + 1);
    Ok(DagFixtureEdge { id: edge_id, source: format!("{source_node_id}@{source_port_id}"), target: format!("{target_node_id}@{target_port_id}"), ..Default::default() })
}

/// 🩹️ Builds the `DagNodePatch` for a `patchDagNodes` field write (name, or a slider param that also
/// refits the widget size). `raw_value` is the typed `DagCommand::PatchDagNodes.value` field verbatim
/// (a plain `&str`, not a `serde_json::Value` — the typed command carries the raw UI input string
/// directly, so numeric fields parse it themselves instead of round-tripping through a JSON value that
/// would always classify it as a JSON string).
pub fn node_patch_for_field(node: &DagNodeSpec, field: &str, raw_value: Option<&str>) -> Option<DagNodePatch> {
    match field {
        "name" => raw_value.map(|value| DagNodePatch { name: Some(value.into()), ..Default::default() }),
        "value" | "min" | "max" if matches!(node.kind, DagNodeKind::Slider { .. }) => {
            let value = raw_value.and_then(|value| value.parse::<f64>().ok())?;
            let mut updated = node.clone();
            if let DagNodeKind::Slider { value: ref mut slider_value, min: ref mut slider_min, max: ref mut slider_max, .. } = updated.kind {
                match field {
                    "value" => *slider_value = value,
                    "min" => *slider_min = value,
                    _ => *slider_max = value,
                }
            }
            fit_node_size(&mut updated);
            Some(DagNodePatch { kind: Some(updated.kind.clone()), width: Some(updated.width), height: Some(updated.height), ..Default::default() })
        }
        _ => None,
    }
}

/// 🗑️ Operations removing `node_ids`, for delete-node / delete-selection. Two app-level consumers
/// (`🎮️commands/➕️add-node::remove_node` and `🎮️commands/🕸️set-algorithm::{delete_selection, node_graph_edit}`)
/// — takes only `DagSnapshot`, no app-only config type, so per the DocumentHelpers placement rule it
/// lives here rather than being duplicated per consumer. `delete-node`'s own diff/inverse already
/// captures the cascade (every edge touching the node), so this is one mutation per node, not one
/// per node PLUS one per severed edge.
pub fn remove_nodes_operations(document: &DagSnapshot, node_ids: &[String]) -> Vec<DagMutation> {
    document.nodes().iter().filter(|node| node_ids.contains(&node.id)).map(|node| delete_node(node.id.clone())).collect()
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧩️document-behavior/🦀️.rs"]
mod document_behavior_tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
pub use crate::DagCamera;
pub use crate::DagFixtureEdge;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::DagNodeSpec;
//#endregion 🔁️Re-exports

#[cfg(test)]
#[path = "🧪️tests/🪪️document-contract/🦀️.rs"]
mod document_contract_tests;
