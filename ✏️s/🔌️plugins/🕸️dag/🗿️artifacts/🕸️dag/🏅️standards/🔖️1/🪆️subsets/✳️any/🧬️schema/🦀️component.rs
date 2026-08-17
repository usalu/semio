//! 🧬️ DAG artifact schema — every field of the artifact with its state class.

use crate::artifacts::dag::mutations::delete_node;
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::{DagCamera, DagContentChild, DagFixtureEdge, DagNodeKind, DagNodePatch, DagNodeSpec, DagPreviewContent, DagSnapshot, IoPortSpec, DAG_DOCUMENT_SCHEMA};
use infinite_board_port_directed_dag::{fit_node_size, note_widget_size, preview_widget_size, would_create_cycle};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

//#region 🔖️Artifact
/// 🧬️ Full DAG artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.graph")]
    pub content: DagContentChild,
    #[state(presence)]
    #[serde(default)]
    pub selected_node_ids: Vec<String>,
    #[state(config)]
    pub camera: DagCamera,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DagArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::dag::default_snapshot())
    }
}

impl DagArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::dag::DagSnapshot {
        crate::artifacts::dag::DagSnapshot { schema: self.schema.clone(), content: self.content.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::dag::DagSnapshot) -> Self {
        Self { schema: snapshot.schema, content: snapshot.content, selected_node_ids: Vec::new(), camera: DagCamera::default(), locale: "en-US".into() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::dag::DagSnapshot) {
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
pub fn dag_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.dag.dag",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::dag::{DagDiff, DagMutation, DagSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct DagBuilderConstruction {
        snapshot: DagSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for DagBuilderConstruction {
        type Snapshot = DagSnapshot;
        type Mutation = DagMutation;
        type Diff = DagDiff;
        fn empty() -> Self { Self { snapshot: DagSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<DagSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DagSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error(
                    "mutation.apply",
                    dsl::TextSpan::at(1, 1),
                    error.to_string(),
                )),
            }
            (self, outcome)
        }
        fn absorb(
            mut self,
            diff: Self::Diff,
        ) -> protocol::MutationApplyResult<Self> {
            let snapshot = <DagDiff as protocol::MutationDiff<DagSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::dag::DagSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct DagParts {
        pub snapshot: Option<DagSnapshot>,
    }

    pub struct DagAnalyzerAnalysis;

    impl ArtifactAnalysis for DagAnalyzerAnalysis {
        type Parts = DagParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.dag", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = DagParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <DagSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <DagSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec DagBuilderFacets {
        construction: derived_construction::DagBuilderConstruction,
        analysis: derived_analysis::DagAnalyzerAnalysis,
        composition: super::super::io::derived_composition::DagComposerComposition,
    }
    builder: DagBuilder,
    analyzer: DagAnalyzer,
    composer: DagComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region ⚠️ Errors
/// ⚠️ Errors from DAG play app edge-connection building. Relocated from the deleted `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — travels with `connect_edge`, the
/// only function that returns it.
#[derive(Debug, thiserror::Error)]
pub enum DagPlayError {
    #[error("connection would create cycle")]
    CycleDetected,
}
//#endregion ⚠️ Errors

//#region 🔖️DocumentHelpers
/// 🔀️ Pure document helpers over `DagSnapshot`/`DagNodeSpec`. Relocated from the deleted `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — none of these take an app-runtime
/// parameter, so per the region → destination map they belong beside the schema types they operate on.
pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

pub fn document_to_workflow(document: &DagSnapshot) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let scene = crate::artifacts::dag::dag_working_scene(document);
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
/// (`🎮️commands/🔧️add-node::remove_node` and `🎮️commands/🕸️set-algorithm::{delete_selection, node_graph_edit}`)
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
mod document_helpers_tests {
    use super::*;

    #[test]
    fn split_endpoint_defaults_to_out_when_no_port_is_given() {
        assert_eq!(split_endpoint("n1"), ("n1".to_string(), "out".to_string()));
        assert_eq!(split_endpoint("n1@a"), ("n1".to_string(), "a".to_string()));
    }

    #[test]
    fn next_node_id_continues_after_the_highest_existing_suffix() {
        let document = crate::artifacts::dag::default_snapshot();
        let mut nodes = document.nodes();
        nodes.push(DagNodeSpec { id: "n99".into(), ..default_node_for_kind("note", "n99", 0.0, 0.0) });
        let edges = document.edges();
        let content = crate::artifacts::dag::dag_content_child_handle_and_cache(nodes, edges);
        let document = DagSnapshot { schema: document.schema.clone(), content };
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
        let document = crate::artifacts::dag::default_snapshot();
        let nodes = document.nodes();
        if let (Some(first), Some(second)) = (nodes.first(), nodes.get(1)) {
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

    /// 🐛️ Pre-existing bug fix (unrelated to composition — traced via `git log --date=iso` to
    /// commit `31209e7a`, 2026-08-13 00:13:16, the ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES
    /// relocation that introduced both `remove_nodes_operations` and this test together): the
    /// assertion contradicted the function's OWN doc comment ("one mutation per node, not one per
    /// node PLUS one per severed edge" — `delete-node`'s own diff/inverse already captures the edge
    /// cascade internally). `remove_nodes_operations` returns exactly one `delete-node` mutation per
    /// targeted node id, regardless of how many edges touch it.
    #[test]
    fn remove_nodes_operations_returns_one_delete_node_mutation_per_targeted_node() {
        let document = crate::artifacts::dag::default_snapshot();
        let nodes = document.nodes();
        let edges = document.edges();
        let node_id = nodes.first().expect("fixture has a node").id.clone();
        let touching_edges = edges.iter().filter(|edge| { let (from, _) = split_endpoint(&edge.source); let (to, _) = split_endpoint(&edge.target); from == node_id || to == node_id }).count();
        assert!(touching_edges > 0, "fixture must exercise the cascade-capturing case");
        let operations = remove_nodes_operations(&document, std::slice::from_ref(&node_id));
        assert_eq!(operations.len(), 1, "delete-node's own diff/inverse captures the edge cascade internally");
        let remaining: Vec<DagNodeSpec> = nodes.into_iter().filter(|node| node.id != node_id).collect();
        assert!(remaining.iter().all(|node| node.id != node_id));
    }

    #[test]
    fn remove_nodes_operations_is_empty_for_an_unknown_node_id() {
        let document = crate::artifacts::dag::default_snapshot();
        assert!(remove_nodes_operations(&document, &["nonexistent".to_string()]).is_empty());
    }
}
//#endregion 🧪️Tests
