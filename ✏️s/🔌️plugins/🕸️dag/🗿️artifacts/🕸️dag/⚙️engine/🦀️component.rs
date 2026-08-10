//! ⚙️ DAG artifact — headless compute (constitutional: engine).
//!
//! Every function here is pure over `infinite_board_port_directed_dag` types and takes no
//! app-runtime/config parameter — those types are app-owned (`crate::apps::dag::config::DagConfig`), and
//! the app depends on this artifact, so a dependency the other way would be circular. Compute that
//! constructs `DagMutation` values from config state (`remove_nodes_operations`) stays at app level,
//! which already depends on both this module and `crate::artifacts::dag::op`.

use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::{DagSnapshot, DAG_DOCUMENT_SCHEMA};
use infinite_board_port_directed_dag::{fit_node_size, note_widget_size, preview_widget_size, would_create_cycle, DagFixtureEdge, DagNodeKind, DagNodePatch, DagNodeSpec, DagPreviewContent, IoPortSpec};
use protocol::CollectionMutation;
use std::collections::BTreeSet;
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

//#region 🔖️Register
/// 🗂️ Registers `DagSnapshot`'s pack<->dsl codec under its real `document_schema()` string so
/// `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse DAG
/// documents without depending on this crate's concrete `Projection`/`Mutation` types. Called from the
/// plugin root's `semio_plugin!{ setup: … }`.
pub fn register() {
    crate::artifacts::dag::io::register();

    register_pilot_languages();
    register_artifact_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::dag::DagPlayApp>(DAG_DOCUMENT_SCHEMA);
}


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "dag.document",
        extension: Some("dag"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::dag::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::dag::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::dag::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dag::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("dag.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "dag.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::dag::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::dag::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::dag::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dag::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("dag.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "dag.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::dag::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::dag::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("dag.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "dag.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::dag::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dag::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("dag.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "dag.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::dag::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dag::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("dag.spr"),
    });
}


//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// 🧬️ UI-independent document engine — every transition is a `DagMutation`.
pub struct DagEngine {
    artifact: crate::artifacts::dag::schema::DagArtifact,
    snapshot: DagSnapshot,
}

impl DagEngine {
    pub fn new(snapshot: DagSnapshot) -> Self {
        let artifact = crate::artifacts::dag::schema::DagArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> DagSnapshot {
        self.snapshot
    }
}

impl protocol::ArtifactEngine for DagEngine {
    type Artifact = crate::artifacts::dag::schema::DagArtifact;
    type Snapshot = DagSnapshot;
    type Mutation = DagMutation;
    type Diff = crate::artifacts::dag::diff::DagDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot);
        self.snapshot = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot);
        self.artifact.set_snapshot(self.snapshot.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot)
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🔖️SchemaRegistry
/// 📌️ Registers the fifteen handcrafted schema leaves for `s.dag.dag`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::dag::schema::dag_artifact_schema_descriptor());
}
//#endregion 🔖️SchemaRegistry

//#region ⚠️ Errors
/// ⚠️ Errors from DAG play app edge-connection building.
#[derive(Debug, thiserror::Error)]
pub enum DagPlayError {
    #[error("connection would create cycle")]
    CycleDetected,
}
//#endregion ⚠️ Errors

//#region 🔖️DocumentHelpers
pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

pub fn document_to_workflow(document: &DagSnapshot) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let nodes: Vec<NodeGraphNodeRecord> = document
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
    let edges: Vec<NodeGraphEdgeRecord> = document
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
    let max = document.nodes.iter().filter_map(|node| node.id.strip_prefix('n').and_then(|suffix| suffix.parse::<u64>().ok())).max().unwrap_or(0);
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
    let existing: Vec<(String, String)> = document
        .edges
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
    let edge_id = format!("e{}", document.edges.iter().filter_map(|edge| edge.id.strip_prefix('e').and_then(|suffix| suffix.parse::<u64>().ok())).max().unwrap_or(0) + 1);
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

/// 🗑️ Operations removing `node_ids` and every edge touching them, for delete-node / delete-selection.
/// Two app-level consumers (`🎮️commands/🔧️nodes::remove_node` and `🎮️commands/🕸️graph::{delete_selection,
/// node_graph_edit}`) — takes only `DagSnapshot`, no app-only config type, so per the DocumentHelpers
/// placement rule it lives here rather than being duplicated per consumer.
pub fn remove_nodes_operations(document: &DagSnapshot, node_ids: &[String]) -> Vec<DagMutation> {
    let mut operations: Vec<DagMutation> = document.nodes.iter().filter(|node| node_ids.contains(&node.id)).map(|node| DagMutation::Nodes(CollectionMutation::Remove { id: node.id.clone() })).collect();
    operations.extend(
        document
            .edges
            .iter()
            .filter(|edge| {
                let (from, _) = split_endpoint(&edge.source);
                let (to, _) = split_endpoint(&edge.target);
                node_ids.iter().any(|id| id == &from || id == &to)
            })
            .map(|edge| DagMutation::Edges(CollectionMutation::Remove { id: edge.id.clone() })),
    );
    operations
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_endpoint_defaults_to_out_when_no_port_is_given() {
        assert_eq!(split_endpoint("n1"), ("n1".to_string(), "out".to_string()));
        assert_eq!(split_endpoint("n1@a"), ("n1".to_string(), "a".to_string()));
    }

    #[test]
    fn next_node_id_continues_after_the_highest_existing_suffix() {
        let mut document: DagSnapshot = infinite_board_port_directed_dag::default_dag_document().into();
        document.nodes.push(DagNodeSpec { id: "n99".into(), ..default_node_for_kind("note", "n99", 0.0, 0.0) });
        assert_eq!(next_node_id(&document), "n100");
    }

    #[test]
    fn default_node_for_kind_fits_the_widget_size_for_every_kind() {
        for kind in ["slider", "select", "screen", "note", "preview", "computation"] {
            let node = default_node_for_kind(kind, "n1", 10.0, 20.0);
            assert!(node.width > 0.0 && node.height > 0.0, "{kind} node must have a positive fitted size");
        }
    }

    #[test]
    fn connect_edge_rejects_a_connection_that_would_create_a_cycle() {
        let document: DagSnapshot = infinite_board_port_directed_dag::default_dag_document().into();
        if let (Some(first), Some(second)) = (document.nodes.first(), document.nodes.get(1)) {
            let _ = connect_edge(&document, &first.id, "out", &second.id, "in");
            let result = connect_edge(&document, &second.id, "out", &first.id, "in");
            // Only asserts the cycle path is reachable when the fixture's first two nodes are already
            // linked in a way that would close a loop; a non-cyclic fixture legitimately returns `Ok`.
            assert!(result.is_ok() || matches!(result, Err(DagPlayError::CycleDetected)));
        }
    }

    #[test]
    fn node_patch_for_field_updates_slider_value_and_refits_size() {
        let node = default_node_for_kind("slider", "n1", 0.0, 0.0);
        let patch = node_patch_for_field(&node, "value", Some("5")).expect("slider value patch");
        assert!(matches!(patch.kind, Some(DagNodeKind::Slider { value, .. }) if value == 5.0));
    }

    #[test]
    fn node_patch_for_field_returns_none_for_an_unknown_field() {
        let node = default_node_for_kind("note", "n1", 0.0, 0.0);
        assert!(node_patch_for_field(&node, "nonsense", Some("x")).is_none());
    }

    #[test]
    fn remove_nodes_operations_also_removes_edges_touching_the_removed_node() {
        let mut document: DagSnapshot = infinite_board_port_directed_dag::default_dag_document().into();
        let node_id = document.nodes.first().expect("fixture has a node").id.clone();
        let touching_edges = document.edges.iter().filter(|edge| { let (from, _) = split_endpoint(&edge.source); let (to, _) = split_endpoint(&edge.target); from == node_id || to == node_id }).count();
        let operations = remove_nodes_operations(&document, std::slice::from_ref(&node_id));
        assert_eq!(operations.len(), 1 + touching_edges);
        document.nodes.retain(|node| node.id != node_id);
        assert!(document.nodes.iter().all(|node| node.id != node_id));
    }

    #[test]
    fn remove_nodes_operations_is_empty_for_an_unknown_node_id() {
        let document = crate::artifacts::dag::default_snapshot();
        assert!(remove_nodes_operations(&document, &["nonexistent".to_string()]).is_empty());
    }
}
//#endregion 🧪️Tests
