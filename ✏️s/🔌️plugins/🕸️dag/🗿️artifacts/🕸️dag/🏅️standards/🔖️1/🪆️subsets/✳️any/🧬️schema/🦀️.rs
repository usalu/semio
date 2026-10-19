//! 🧬️ DAG artifact schema — every field of the artifact with its state class.

use crate::{DagContentChild, DagMutation, DagNodeKind, DagPreviewContent, DagScene, DagSnapshot, IoPortSpec, SemioGraphMutation};
use framework_schema::ArtifactSchema;
use infinite_board_port_directed_dag::{fit_node_size, note_widget_size, preview_widget_size, would_create_cycle};
use semio_framework_ui_declaration::ui::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};
//#region 🔖️Artifact
/// 🧬️ DAG document artifact state.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: DagContentChild,
}

impl semio_framework_value::FromValue for DagArtifact {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <DagSnapshot as semio_framework_value::FromValue>::from_value(value).map(Self::from_snapshot)
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

}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.dag.dag` — twenty handcrafted schema leaves.
pub fn dag_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.dag.dag",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
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

/// 🧭️ The node-graph records a scene renders as.
pub fn document_to_workflow(scene: &DagScene) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
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
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), value_type: port.value_type.clone(), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), value_type: port.value_type.clone(), ..Default::default() }).collect(),
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

/// 🆔️ The next free `n<k>` node id of a scene.
pub fn next_node_id(scene: &DagScene) -> String {
    let max = scene.nodes.iter().filter_map(|node| node.id.strip_prefix('n').and_then(|suffix| suffix.parse::<u64>().ok())).max().unwrap_or(0);
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
            let (width, height) = preview_widget_size(&DagPreviewContent::Scalar { text: String::new() }, &crate::DagExpandedPaths::new());
            DagNodeSpec {
                id: id.into(),
                name: "Preview".into(),
                abbreviation: "Prv".into(),
                icon: "emoji:👁️".into(),
                x,
                y,
                width,
                height,
                kind: DagNodeKind::Preview { content: DagPreviewContent::Scalar { text: String::new() }, expanded: crate::DagExpandedPaths::new(), input: IoPortSpec::named("I", "In", "in", "Input") },
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

/// 🔗️ Builds the `DagHostSnapshotEdge` connecting two ports, or `Err` if it would introduce a cycle.
pub fn connect_edge(scene: &DagScene, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> Result<DagHostSnapshotEdge, DagPlayError> {
    let edges = &scene.edges;
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
    Ok(DagHostSnapshotEdge { id: edge_id, source: format!("{source_node_id}@{source_port_id}"), target: format!("{target_node_id}@{target_port_id}"), ..Default::default() })
}

/// 🩹️ The child leaves of a `patchDagNodes` field write: `name` is one `change-node-label`; a slider's `value`/`min`/`max`
/// is the ABSOLUTE `set-node-property` of that kind field, plus a `resize-node` when the widget refits to a new size.
/// `raw_value` is the typed command's UI input string verbatim (numeric fields parse it themselves).
pub fn node_field_leaves(node: &DagNodeSpec, field: &str, raw_value: &str) -> Vec<SemioGraphMutation> {
    use {semio_s_artifact_stdio_semio_graph::schema::mutations::change_node_label::ChangeNodeLabel, semio_s_artifact_stdio_semio_graph::schema::mutations::resize_node::ResizeNode, semio_s_artifact_stdio_semio_graph::schema::mutations::set_node_property::SetNodeProperty};
    use semio_s_artifact_stdio_semio_graph::schema::snapshot::GraphNodeId;
    let id = || GraphNodeId::new(node.id.clone());
    match (field, &node.kind, raw_value.trim().parse::<f64>().ok().filter(|value| value.is_finite())) {
        ("name", _, _) if node.name != raw_value => vec![SemioGraphMutation::ChangeNodeLabel(ChangeNodeLabel { id: id(), new_label: raw_value.into() })],
        ("value" | "min" | "max", DagNodeKind::Slider { value, min, max, .. }, Some(next)) if next != match field { "value" => *value, "min" => *min, _ => *max } => {
            let mut updated = node.clone();
            if let DagNodeKind::Slider { value, min, max, .. } = &mut updated.kind {
                *match field { "value" => value, "min" => min, _ => max } = next;
            }
            fit_node_size(&mut updated);
            let set = SemioGraphMutation::SetNodeProperty(SetNodeProperty { node_id: id(), key: field.into(), value: crate::semio_value_of(&semio_framework_value::DslValue::float(next)) });
            let resize = ((updated.width, updated.height) != (node.width, node.height)).then(|| SemioGraphMutation::ResizeNode(ResizeNode { id: id(), width: updated.width, height: updated.height }));
            std::iter::once(set).chain(resize).collect()
        }
        _ => Vec::new(),
    }
}

/// 🗑️ The child leaves removing `node_ids` from a scene, for remove-node / delete-selection: every incident edge is deleted
/// BEFORE its node, so each published row is point-invertible (one `delete-edge` undoes as one `create-edge`, an edge-less
/// `delete-node` as one `create-node`, each at its base index — the undo is byte-exact).
pub fn remove_nodes_leaves(scene: &DagScene, node_ids: &[String]) -> Vec<SemioGraphMutation> {
    use {semio_s_artifact_stdio_semio_graph::schema::mutations::delete_edge::DeleteEdge, semio_s_artifact_stdio_semio_graph::schema::mutations::delete_node::DeleteNode};
    use semio_s_artifact_stdio_semio_graph::schema::snapshot::{GraphEdgeId, GraphNodeId};
    let removed: Vec<&str> = scene.nodes.iter().filter(|node| node_ids.contains(&node.id)).map(|node| node.id.as_str()).collect();
    let touches = |edge: &DagHostSnapshotEdge| removed.contains(&split_endpoint(&edge.source).0.as_str()) || removed.contains(&split_endpoint(&edge.target).0.as_str());
    let edges = scene.edges.iter().filter(|edge| touches(edge)).map(|edge| SemioGraphMutation::DeleteEdge(DeleteEdge { id: GraphEdgeId::new(edge.id.clone()) }));
    edges.chain(removed.iter().map(|id| SemioGraphMutation::DeleteNode(DeleteNode { id: GraphNodeId::new(*id) }))).collect()
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔁️Re-exports
pub use crate::DagCamera;
pub use crate::DagHostSnapshotEdge;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::DagNodeSpec;
//#endregion 🔁️Re-exports



//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
/// 🪪️ This artifact's canonical dialect (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
/// contract §1) — lives at the ARTIFACT level (not under `editor`/`viewer`) specifically so a
/// viewer file can read it without ever importing through the sibling `editor` module. Matches
/// `#[artifact_schema(id = "s.dag.dag")]` on `DagArtifact`; `standard`/`subset` match this
/// artifact's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is
/// `s.dag.dag@1/*#editor` / `s.dag.dag@1/*#viewer`.
pub const DAG_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.dag.dag", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{DagScene, dag_scene_of_content};
use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
pub use crate::standards::v1::subsets::any::schema::snapshot::DagSnapshot;

//#region 🔖️Scene
/// 🧸️ Composes the scene from the document's exact published `content` child (`doc.children`, design §20.15).
pub fn dag_scene_from_children(snapshot: &DagSnapshot, children: &semio_framework_plugin::app::ChildContentView) -> Result<DagScene, semio_framework_plugin::Fault> {
    let child_id = &snapshot.content.child_id;
    let dialect = children.dialect("content", child_id).ok_or_else(|| semio_framework_plugin::Fault::from("dag-content-child-dialect-required"))?;
    if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "graph" {
        return Err(semio_framework_plugin::Fault::from("dag-content-child-dialect-mismatch"));
    }
    let content = children.typed_read::<SemioGraphSnapshot>("content", child_id)?;
    Ok(dag_scene_of_content(&content))
}

/// 🧸️ [`dag_scene_from_children`] over a document view.
pub fn dag_scene(doc: &semio_framework_plugin::ArtifactView<'_, DagSnapshot>) -> Result<DagScene, semio_framework_plugin::Fault> {
    dag_scene_from_children(doc.snapshot, &doc.children)
}
//#endregion 🔖️Scene
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{SemioGraphMutation};
use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
pub use crate::standards::v1::subsets::any::schema::mutations::DagMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::DagSnapshot;

//#region 🔖️ChildEmit
/// 🧬️ Publishes child `leaves` as ONE edit of the exact composed `content` child; no leaf is the empty emit.
pub fn dag_child_emit<C, D>(snapshot: &DagSnapshot, leaves: Vec<SemioGraphMutation>) -> semio_framework_plugin::Emit<DagMutation, C, D> {
    if leaves.is_empty() {
        return semio_framework_plugin::Emit::default();
    }
    semio_framework_plugin::Emit { child_preparations: std::collections::VecDeque::from([semio_framework_plugin::app::ChildEmitPreparation::of::<SemioGraphSnapshot, _>("content", &snapshot.content.child_id, leaves)]), ..Default::default() }
}
//#endregion 🔖️ChildEmit
}
//#endregion 🧩️ArtifactItems5

//#region 🧩️ArtifactItems7
pub mod artifact_items_7 {
use crate::{SemioGraphMutation, dag_graph_edge, dag_graph_node};
pub use semio_framework_os_infinite_directed_presentation::DagHostSnapshotEdge;
pub use semio_framework_os_infinite_directed_presentation::DagNodeSpec;
use semio_s_artifact_stdio_semio_graph::{schema::snapshot::GraphEdgeId as SemioGraphEdgeId, schema::snapshot::GraphNodeId as SemioGraphNodeId, schema::snapshot::SemioGraphEdge, schema::snapshot::SemioGraphNode};

//#region 🔖️ChildLeaves
/// 🌱 The graph child leaf that creates one DAG node (appended).
pub fn create_node_leaf(node: &DagNodeSpec) -> SemioGraphMutation {
    use semio_s_artifact_stdio_semio_graph::schema::mutations::create_node::CreateNode;
    let SemioGraphNode { id, kind, label, position, width, height, ports, properties } = dag_graph_node(node);
    SemioGraphMutation::CreateNode(CreateNode { id, kind, label, position, width, height, ports, properties, at: None })
}

/// 🤝️ The graph child leaf that creates one DAG edge (appended).
pub fn create_edge_leaf(edge: &DagHostSnapshotEdge) -> SemioGraphMutation {
    use semio_s_artifact_stdio_semio_graph::schema::mutations::create_edge::CreateEdge;
    let SemioGraphEdge { id, source, target, kind, label, source_port, target_port, properties } = dag_graph_edge(edge);
    SemioGraphMutation::CreateEdge(CreateEdge { id, source, target, kind, label, source_port, target_port, properties, at: None })
}

/// ✂️ The graph child leaf that deletes one DAG edge.
pub fn delete_edge_leaf(edge_id: &str) -> SemioGraphMutation {
    use semio_s_artifact_stdio_semio_graph::schema::mutations::delete_edge::DeleteEdge;
    SemioGraphMutation::DeleteEdge(DeleteEdge { id: SemioGraphEdgeId::new(edge_id) })
}

/// ✋️ The relative graph child leaf that drags DAG nodes by one offset.
pub fn drag_nodes_leaf(node_ids: Vec<String>, dx: f64, dy: f64) -> SemioGraphMutation {
    use semio_s_artifact_stdio_semio_graph::schema::mutations::drag_nodes::DragNodes;
    SemioGraphMutation::DragNodes(DragNodes { targets: node_ids.into_iter().map(SemioGraphNodeId::new).collect(), dx, dy })
}

/// 🏷️ The graph child leaf that changes a DAG node's identity key (its edges follow).
pub fn rename_node_leaf(id: &str, new_id: &str) -> SemioGraphMutation {
    use semio_s_artifact_stdio_semio_graph::schema::mutations::rename_node::RenameNode;
    SemioGraphMutation::RenameNode(RenameNode { id: SemioGraphNodeId::new(id), new_id: SemioGraphNodeId::new(new_id) })
}
//#endregion 🔖️ChildLeaves
}
//#endregion 🧩️ArtifactItems7

//#region 🧩️ArtifactItems8
pub mod artifact_items_8 {
use serde::{Deserialize, Serialize};

//#region 🔖️Domain
/// 🎥️ Viewport camera for the DAG canvas (plugin-owned; distinct from framework `dag` kernel helpers).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DagCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for DagCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

impl From<DagCamera> for semio_framework_os_infinite_directed_presentation::DagCamera {
    fn from(value: DagCamera) -> Self {
        Self { x: value.x, y: value.y, zoom: value.zoom }
    }
}

impl From<semio_framework_os_infinite_directed_presentation::DagCamera> for DagCamera {
    fn from(value: semio_framework_os_infinite_directed_presentation::DagCamera) -> Self {
        Self { x: value.x, y: value.y, zoom: value.zoom }
    }
}
//#endregion 🔖️Domain
}
//#endregion 🧩️ArtifactItems8
