//! 🚪️ Artifact representation boundary.



//#region 🧩️ArtifactItems
pub mod artifact_items {
pub const DAG_DOCUMENT_SCHEMA: &str = "dag.dag";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
pub use semio_framework_os_infinite_directed_presentation::DagHostSnapshotEdge;
pub use semio_framework_os_infinite_directed_presentation::{DagNodeSpec, IoPortSpec};

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio@v1/graph` document — the dag's nodes and edges live in this
/// child's store; the parent document owns no content and no leaf that could read it (design §20.15).
pub type DagContentChild = store::os_store::artifact_child::ArtifactChild<SemioGraphSnapshot>;

use semio_framework_value::{DslValue, Number};
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
pub use semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation;
use semio_s_artifact_stdio_semio_graph::schema::snapshot::{GraphEdgeId as SemioGraphEdgeId, GraphNodeId as SemioGraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphPort, SemioGraphPortKind, SemioGraphSnapshot, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;

/// 🔗️ The graph edge kind every DAG connection carries.
pub const DAG_EDGE_KIND: &str = "dag-edge";
/// 🧾️ `DagNodeSpec` record fields carried by the graph node's NATIVE slots (id, label, position, size, kind tag) — every
/// other field is one typed node property keyed by its record field name, so a field edit is one `set-node-property`.
pub const DAG_NODE_NATIVE_FIELDS: [&str; 7] = ["id", "name", "x", "y", "width", "height", "kind"];
/// 🧾️ `DagHostSnapshotEdge` record fields carried by the graph edge's native slots.
pub const DAG_EDGE_NATIVE_FIELDS: [&str; 3] = ["id", "source", "target"];

/// 🔢️ The typed graph property value of one record field value (numbers keep their integer/float class).
pub fn semio_value_of(value: &DslValue) -> SemioValue {
    match value {
        DslValue::Null => SemioValue::Null,
        DslValue::Bool(value) => SemioValue::Bool { value: *value },
        DslValue::Number(Number::UInt(value)) => SemioValue::Int { lexeme: value.to_string() },
        DslValue::Number(Number::Int(value)) => SemioValue::Int { lexeme: value.to_string() },
        DslValue::Number(Number::Float(value)) => SemioValue::Float { lexeme: format!("{value:?}") },
        DslValue::String(value) => SemioValue::Str { value: value.clone() },
        DslValue::Bytes(value) => SemioValue::Bytes { value: value.clone() },
        DslValue::Array(items) => SemioValue::List { items: items.iter().map(semio_value_of).collect() },
        DslValue::Object(entries) => SemioValue::Map { entries: entries.iter().map(|(key, value)| SemioValueEntry { key: key.clone(), value: semio_value_of(value) }).collect() },
    }
}

/// 🔢️ The record field value of one typed graph property value — the exact inverse of [`semio_value_of`].
pub fn dsl_value_of(value: &SemioValue) -> DslValue {
    match value {
        SemioValue::Null => DslValue::Null,
        SemioValue::Bool { value } => DslValue::Bool(*value),
        SemioValue::Int { lexeme } => lexeme.parse::<u64>().map(DslValue::uint).or_else(|_| lexeme.parse::<i64>().map(DslValue::int)).unwrap_or_else(|_| DslValue::String(lexeme.clone())),
        SemioValue::Float { lexeme } => lexeme.parse::<f64>().map(DslValue::float).unwrap_or_else(|_| DslValue::String(lexeme.clone())),
        SemioValue::Str { value } => DslValue::String(value.clone()),
        SemioValue::Bytes { value } => DslValue::Bytes(value.clone()),
        SemioValue::List { items } => DslValue::Array(items.iter().map(dsl_value_of).collect()),
        SemioValue::Map { entries } => DslValue::Object(entries.iter().map(|entry| (entry.key.clone(), dsl_value_of(&entry.value))).collect()),
        SemioValue::Ref { id } => DslValue::String(id.value.clone()),
    }
}

/// 🧩️ The typed graph properties of a record value: every top-level field except `native`, in record order.
pub fn dag_record_properties(record: DslValue, native: &[&str]) -> Vec<SemioValueEntry> {
    match record {
        DslValue::Object(entries) => entries.into_iter().filter(|(key, _)| !native.contains(&key.as_str())).map(|(key, value)| SemioValueEntry { value: semio_value_of(&value), key }).collect(),
        _ => Vec::new(),
    }
}

/// 🌉 One `DagNodeSpec` as a graph node: id, name, position, size and kind tag on the native slots, the ports projected from
/// the kind for neutral graph tooling, every other record field one typed property.
pub fn dag_graph_node(node: &DagNodeSpec) -> SemioGraphNode {
    let port = |port: &IoPortSpec, kind| SemioGraphPort { name: port.id.clone(), kind, category: String::new(), properties: Vec::new() };
    SemioGraphNode {
        id: SemioGraphNodeId::new(node.id.clone()),
        kind: semio_framework_os_infinite_directed_presentation::dag_node_kind_tag(&node.kind).to_string(),
        label: node.name.clone(),
        position: SemioPoint2 { x: node.x, y: node.y },
        width: node.width,
        height: node.height,
        ports: node.inputs().iter().map(|input| port(input, SemioGraphPortKind::In)).chain(node.outputs().iter().map(|output| port(output, SemioGraphPortKind::Out))).collect(),
        properties: dag_record_properties(semio_framework_value::ToValue::to_value(node), &DAG_NODE_NATIVE_FIELDS),
    }
}

/// 🌉 The inverse of [`dag_graph_node`]: the native slots are authoritative, the properties carry the rest. A node authored
/// outside this plugin (no decodable record) reads as a minimal computation node built from the native slots alone.
pub fn dag_node_of_graph(node: &SemioGraphNode) -> DagNodeSpec {
    let native = [
        ("id".to_string(), DslValue::String(node.id.value.clone())),
        ("name".to_string(), DslValue::String(node.label.clone())),
        ("x".to_string(), DslValue::float(node.position.x)),
        ("y".to_string(), DslValue::float(node.position.y)),
        ("width".to_string(), DslValue::float(node.width)),
        ("height".to_string(), DslValue::float(node.height)),
        ("kind".to_string(), DslValue::String(node.kind.clone())),
    ];
    let record = DslValue::Object(native.into_iter().chain(node.properties.iter().map(|entry| (entry.key.clone(), dsl_value_of(&entry.value)))).collect());
    <DagNodeSpec as semio_framework_value::FromValue>::from_value(record).unwrap_or_else(|_| DagNodeSpec { id: node.id.value.clone(), name: node.label.clone(), x: node.position.x, y: node.position.y, width: node.width, height: node.height, ..Default::default() })
}

/// 🌉 One `DagHostSnapshotEdge` as a graph edge: endpoint node ids and ports on the native slots (a bare endpoint keeps no
/// port), every other record field one typed property.
pub fn dag_graph_edge(edge: &DagHostSnapshotEdge) -> SemioGraphEdge {
    let endpoint = |value: &str| value.split_once('@').map_or_else(|| (value.to_string(), None), |(node, port)| (node.to_string(), Some(port.to_string())));
    let ((source, source_port), (target, target_port)) = (endpoint(&edge.source), endpoint(&edge.target));
    SemioGraphEdge {
        id: SemioGraphEdgeId::new(edge.id.clone()),
        source: SemioGraphNodeId::new(source),
        target: SemioGraphNodeId::new(target),
        kind: DAG_EDGE_KIND.into(),
        label: String::new(),
        source_port,
        target_port,
        properties: dag_record_properties(semio_framework_value::ToValue::to_value(edge), &DAG_EDGE_NATIVE_FIELDS),
    }
}

/// 🌉 The inverse of [`dag_graph_edge`]; an edge authored outside this plugin reads as a bare node-to-node edge.
pub fn dag_edge_of_graph(edge: &SemioGraphEdge) -> DagHostSnapshotEdge {
    let endpoint = |node: &SemioGraphNodeId, port: &Option<String>| port.as_ref().map_or_else(|| node.value.clone(), |port| format!("{}@{port}", node.value));
    let native = [("id".to_string(), DslValue::String(edge.id.value.clone())), ("source".to_string(), DslValue::String(endpoint(&edge.source, &edge.source_port))), ("target".to_string(), DslValue::String(endpoint(&edge.target, &edge.target_port)))];
    let record = DslValue::Object(native.into_iter().chain(edge.properties.iter().map(|entry| (entry.key.clone(), dsl_value_of(&entry.value)))).collect());
    <DagHostSnapshotEdge as semio_framework_value::FromValue>::from_value(record).unwrap_or_else(|_| DagHostSnapshotEdge { id: edge.id.value.clone(), source: endpoint(&edge.source, &edge.source_port), target: endpoint(&edge.target, &edge.target_port), ..Default::default() })
}

/// 🌉 The composed graph content of a DAG scene.
pub fn dag_content_snapshot(scene: &DagScene) -> SemioGraphSnapshot {
    SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: scene.nodes.iter().map(dag_graph_node).collect(), edges: scene.edges.iter().map(dag_graph_edge).collect() }
}

/// 🌉 The DAG scene a composed graph content reads as.
pub fn dag_scene_of_content(content: &SemioGraphSnapshot) -> DagScene {
    DagScene { nodes: content.nodes.iter().map(dag_node_of_graph).collect(), edges: content.edges.iter().map(dag_edge_of_graph).collect() }
}

/// 🕸️ The deterministic content-addressed CHILD handle of a scene — same `(child_id, target)` for identical content.
pub fn dag_content_child_handle(scene: &DagScene) -> DagContentChild {
    use pack::snapshot::ArtifactPack;
    let child_id = store::content_id("dag-content", &<SemioGraphSnapshot as ArtifactPack>::encode_pack(&dag_content_snapshot(scene)));
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "graph".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}
/// 🌱 The DAG's nodes and edges as the editor reads them: composed on read from the `content` child's store, never held by
/// the parent document.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DagScene {
    pub nodes: Vec<DagNodeSpec>,
    pub edges: Vec<DagHostSnapshotEdge>,
}
//#endregion 🔖️ContentBridge
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{DagScene, dag_content_child_handle, dag_content_snapshot};
use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
pub use crate::standards::v1::subsets::any::schema::snapshot::DagSnapshot;

//#region 🔖️DerivableScene
/// 🌱️ The content a document's `content` child derives without a member store: the bundled demo graph or the empty graph;
/// any other child id is not derivable (its content lives only in its member store). Readers without a child view —
/// inference and the foreign serializers — read this until the framework hands them child head packs (design D2).
pub fn dag_derivable_scene(snapshot: &DagSnapshot) -> Option<DagScene> {
    let child_id = snapshot.content.child_id.as_str();
    if child_id == crate::examples::demo::CONTENT_CHILD_ID {
        return Some(crate::examples::demo::scene());
    }
    (child_id == dag_content_child_handle(&DagScene::default()).child_id).then(DagScene::default)
}

/// 🌱️ Packs the derivable `content` member (the react shell's `loadDocumentPair` sends `members: []`).
pub fn genesis_dag_child_pack(snapshot: &DagSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use pack::snapshot::ArtifactPack;
    (slot == "content" && child_id == snapshot.content.child_id).then(|| dag_derivable_scene(snapshot)).flatten().map(|scene| <SemioGraphSnapshot as ArtifactPack>::encode_pack(&dag_content_snapshot(&scene)))
}
//#endregion 🔖️DerivableScene
}
//#endregion 🧩️ArtifactItems6
