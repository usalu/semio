//! 🔀️ DAG artifact — the document entity this plugin's app edits.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`reasoning/dag→C:graph`): the old inline
//! `nodes`/`edges` fields are replaced by a composed `s.stdio.semio.graph` CHILD slot
//! (`🔖️ContentBridge` below) — this plugin no longer defines its own persisted node/edge model, it
//! composes stdio's neutral `graph` subset instead. The rich live editing types
//! (`semio_framework_artifact_infinite_dag::DagNodeSpec`/`DagNodeKind`/`DagHostSnapshotEdge`) still flow
//! through the app exactly as before; only the PERSISTED shape changed. They now bridge through the
//! composed child's exact local owner rather than plain struct fields.

extern crate infinite_canvas as infinite_board_port_directed_dag;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_dag_demo_tests;
extern crate semio_framework_schema as framework_schema;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
#[cfg(test)]
use serde::{Deserialize, Serialize};

pub const DAG_DOCUMENT_SCHEMA: &str = "dag.dag";

/// 🪪️ This artifact's canonical dialect (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
/// contract §1) — lives at the ARTIFACT level (not under `editor`/`viewer`) specifically so a
/// viewer file can read it without ever importing through the sibling `editor` module. Matches
/// `#[artifact_schema(id = "s.dag.dag")]` on `DagArtifact`; `standard`/`subset` match this
/// artifact's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is
/// `s.dag.dag@1/*#editor` / `s.dag.dag@1/*#viewer`.
pub const DAG_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.dag.dag", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub use crate::snapshot::schema::{default_snapshot, empty_snapshot};
pub use semio_framework_artifact_infinite_dag::{DagEdgePatch, DagExpandedPaths, DagHostSnapshotEdge, DagNodeKind, DagNodePatch, DagNodeSpec, DagPreviewContent, IoPortSpec};

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio@v1/graph` document — the dag's nodes and edges live in this
/// child's store; the parent document owns no content and no leaf that could read it (design §20.15).
pub type DagContentChild = store::ArtifactChild<SemioGraphSnapshot>;

use semio_framework_value::{DslValue, Number};
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
pub use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::{
    GraphEdgeId as SemioGraphEdgeId, GraphNodeId as SemioGraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphPort, SemioGraphPortKind, SemioGraphSnapshot, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA,
};
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};

/// 🔗️ The graph edge kind every DAG connection carries.
const DAG_EDGE_KIND: &str = "dag-edge";
/// 🧾️ `DagNodeSpec` record fields carried by the graph node's NATIVE slots (id, label, position, size, kind tag) — every
/// other field is one typed node property keyed by its record field name, so a field edit is one `set-node-property`.
const DAG_NODE_NATIVE_FIELDS: [&str; 7] = ["id", "name", "x", "y", "width", "height", "kind"];
/// 🧾️ `DagHostSnapshotEdge` record fields carried by the graph edge's native slots.
const DAG_EDGE_NATIVE_FIELDS: [&str; 3] = ["id", "source", "target"];

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
fn dag_record_properties(record: DslValue, native: &[&str]) -> Vec<SemioValueEntry> {
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
        kind: semio_framework_artifact_infinite_dag::dag_node_kind_tag(&node.kind).to_string(),
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
    use store::ArtifactPack;
    let child_id = store::content_id("dag-content", &<SemioGraphSnapshot as ArtifactPack>::encode_pack(&dag_content_snapshot(scene)));
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "graph".into() };
    let target = store::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️ContentBridge

//#region 🔖️Scene
/// 🌱 The DAG's nodes and edges as the editor reads them: composed on read from the `content` child's store, never held by
/// the parent document.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DagScene {
    pub nodes: Vec<DagNodeSpec>,
    pub edges: Vec<DagHostSnapshotEdge>,
}

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

/// 🧬️ Publishes child `leaves` as ONE edit of the exact composed `content` child; no leaf is the empty emit.
pub fn dag_child_emit<C, D>(snapshot: &DagSnapshot, leaves: &[SemioGraphMutation]) -> semio_framework_plugin::Emit<DagMutation, C, D> {
    if leaves.is_empty() {
        return semio_framework_plugin::Emit::default();
    }
    semio_framework_plugin::Emit { child_emits: vec![semio_framework_plugin::app::ChildEmit::of::<SemioGraphSnapshot, _>("content", &snapshot.content.child_id, leaves)], ..Default::default() }
}

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
    use store::ArtifactPack;
    (slot == "content" && child_id == snapshot.content.child_id).then(|| dag_derivable_scene(snapshot)).flatten().map(|scene| <SemioGraphSnapshot as ArtifactPack>::encode_pack(&dag_content_snapshot(&scene)))
}
//#endregion 🔖️Scene

//#region 🔖️ChildLeaves
/// 🌱 The graph child leaf that creates one DAG node (appended).
pub fn create_node_leaf(node: &DagNodeSpec) -> SemioGraphMutation {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::create_node::CreateNode;
    let SemioGraphNode { id, kind, label, position, width, height, ports, properties } = dag_graph_node(node);
    SemioGraphMutation::CreateNode(CreateNode { id, kind, label, position, width, height, ports, properties, at: None })
}

/// 🤝️ The graph child leaf that creates one DAG edge (appended).
pub fn create_edge_leaf(edge: &DagHostSnapshotEdge) -> SemioGraphMutation {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::create_edge::CreateEdge;
    let SemioGraphEdge { id, source, target, kind, label, source_port, target_port, properties } = dag_graph_edge(edge);
    SemioGraphMutation::CreateEdge(CreateEdge { id, source, target, kind, label, source_port, target_port, properties, at: None })
}

/// ✂️ The graph child leaf that deletes one DAG edge.
pub fn delete_edge_leaf(edge_id: &str) -> SemioGraphMutation {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::delete_edge::DeleteEdge;
    SemioGraphMutation::DeleteEdge(DeleteEdge { id: SemioGraphEdgeId::new(edge_id) })
}

/// ✋️ The relative graph child leaf that drags DAG nodes by one offset.
pub fn drag_nodes_leaf(node_ids: Vec<String>, dx: f64, dy: f64) -> SemioGraphMutation {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::drag_nodes::DragNodes;
    SemioGraphMutation::DragNodes(DragNodes { targets: node_ids.into_iter().map(SemioGraphNodeId::new).collect(), dx, dy })
}

/// 🏷️ The graph child leaf that changes a DAG node's identity key (its edges follow).
pub fn rename_node_leaf(id: &str, new_id: &str) -> SemioGraphMutation {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::rename_node::RenameNode;
    SemioGraphMutation::RenameNode(RenameNode { id: SemioGraphNodeId::new(id), new_id: SemioGraphNodeId::new(new_id) })
}
//#endregion 🔖️ChildLeaves

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

impl From<DagCamera> for semio_framework_artifact_infinite_dag::DagCamera {
    fn from(value: DagCamera) -> Self {
        Self { x: value.x, y: value.y, zoom: value.zoom }
    }
}

impl From<semio_framework_artifact_infinite_dag::DagCamera> for DagCamera {
    fn from(value: semio_framework_artifact_infinite_dag::DagCamera) -> Self {
        Self { x: value.x, y: value.y, zoom: value.zoom }
    }
}
//#endregion 🔖️Domain

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "graph.dag".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("DAG", "DAG"),
        source_format: DAG_DOCUMENT_SCHEMA.into(),
        component_kind: "dag".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Dag },
        schema: DAG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from a
/// plugin `.setup()` callback. `crate::editor::dag::config::schema::register_app_schema()` is the one
/// exception, still called from `🕸️dag/🦀️.rs`'s own `.setup()`: it registers the `DagPlayApp`
/// CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has no field for
/// (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's artifact-scoped
/// function set. Relocated from `⚙️engine` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// reloc-g2): `declaration()` describes the artifact (kind, schema, io ports, ownership), which is not
/// engine behaviour.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.dag.dag.standard.v1", "standard", "1", &[], None),
        ("s.dag.dag.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.dag.dag.schema.artifact", "schema", "s.dag.dag", &[("schema", "s.dag.dag")], None),
        ("s.dag.dag.inference.artifact", "inference", "s.dag.dag.inference", &[("schema", "s.dag.dag.inference")], None),
        ("s.dag.dag.composer.native", "composer", "s.dag.dag@1/*", &[("dialect", "s.dag.dag@1/*")], None),
        ("s.dag.dag.composer.format-2", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.dag.dag.grammar.1", "grammar", "dag.document", &[("grammar", "dag.document")], None),
        ("s.dag.dag.grammar.2", "grammar", "dag.op", &[("grammar", "dag.op")], None),
        ("s.dag.dag.grammar.3", "grammar", "dag.diff", &[("grammar", "dag.diff")], None),
        ("s.dag.dag.grammar.4", "grammar", "dag.pack", &[("grammar", "dag.pack")], None),
        ("s.dag.dag.grammar.5", "grammar", "dag.spr", &[("grammar", "dag.spr")], None),
        ("s.dag.dag.codec.document-1", "codec", "dag.dag:dag", &[("codec", "dag.dag"), ("codec-extension", "7:dag.dag:dag")], None),
        ("s.dag.dag.localization.en", "localization", "DAG", &[], Some(("en", "DAG"))),
        ("s.dag.dag.localization.de", "localization", "Gerichteter azyklischer Graph", &[], Some(("de", "Gerichteter azyklischer Graph"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.dag.dag")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

/// 🔖️ New declaration tree root (design.md §1/§2) — replaces `declaration()` (the old
/// `ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...).languages(...)
/// .document_codec::<...>()` chain) outright. No dual registration: the plugin root's
/// `.declare_artifact(artifact())` call is the ONLY registration channel for this artifact.
/// `definition()` (old `ArtifactDefinition`/capability rows) is KEPT per debt D1 — not deleted
/// repo-wide until W6 — but has zero callers left from this file.
pub fn artifact<A: DagApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.dag.dag").expect("canonical dag kind"), localization: &[], standards: vec![standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait DagApplication:
    semio_framework_plugin::PluginApp + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::dag::DagPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>> + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::dag::DagViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

impl<A> DagApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::dag::DagPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::dag::DagViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod topology {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod io {
    pub use super::standards::v1::subsets::any::io::*;
}
pub mod document_dsl {
    pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
}
pub mod pack {
    pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::io::diff::text::*;
    }
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod snapshot {
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
    }
}
pub use crate::standards::v1::subsets::any::schema::diff::DagDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::DagMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::DagSnapshot;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

/// ✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the mutation-capable surface, migrated
/// wholesale from the retired `🎛️apps/🕸️dag/` app tree into the owned subset's `✏️editor/` facet.
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod dag {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-node/🦀️.rs"]
            pub mod add_node;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔌️connect-media-ports/🦀️.rs"]
            pub mod connect_media_ports;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✂️disconnect/🦀️.rs"]
            pub mod disconnect;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👇️graph-pointer-down/🦀️.rs"]
            pub mod graph_pointer_down;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-media-node/🦀️.rs"]
            pub mod move_media_node;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs"]
            pub mod node_graph_edit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs"]
            pub mod node_graph_viewport;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-dag-nodes/🦀️.rs"]
            pub mod patch_dag_nodes;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-node/🦀️.rs"]
            pub mod remove_node;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️rename-dag-node/🦀️.rs"]
            pub mod rename_dag_node;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod tools {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs"]
                    pub mod reorganize;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️compiled/🦀️.rs"]
                    pub mod compiled;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

/// 👁️ The read-only surface (contract §2.2/§2.6) — a genuinely independent module tree from
/// `editor` above, never `#[path]`-mounting anything under `✏️editor/`.
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod dag {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🕸️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
}

//#region 🧬️ChildRestoreProjection
/// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields — the ONE
/// definition the editor and the viewer both declare. The `ArtifactEditor`/`ArtifactViewer` trait default
/// refuses it (`… did not declare a loaded-parent child projection`), and since PX1 the live envelope load
/// asks for it before the decoded document may replace the store, so an undeclared app fails every live load.
pub fn dag_child_restore_projection(snapshot: &crate::DagSnapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("dag.child-projection"), error.to_string()))
}
//#endregion 🧬️ChildRestoreProjection
