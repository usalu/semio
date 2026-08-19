//! 🔀️ DAG artifact — the document entity this plugin's app edits.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`reasoning/dag→C:graph`): the old inline
//! `nodes`/`edges` fields are replaced by a composed `s.stdio.semio.graph` CHILD slot
//! (`🔖️ContentBridge` below) — this plugin no longer defines its own persisted node/edge model, it
//! composes stdio's neutral `graph` subset instead. The rich live editing types
//! (`infinite_board_port_directed_dag::DagNodeSpec`/`DagNodeKind`/`DagFixtureEdge`) still flow
//! through the app exactly as before; only the PERSISTED shape changed. They now bridge through the
//! composed child + `🔖️WorkingScene` cache rather than plain struct fields.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

pub const DAG_DOCUMENT_SCHEMA: &str = "dag.dag";

/// 🪪️ This artifact's canonical dialect (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
/// contract §1) — lives at the ARTIFACT level (not under `editor`/`viewer`) specifically so a
/// viewer file can read it without ever importing through the sibling `editor` module. Matches
/// `#[artifact_schema(id = "s.dag.dag")]` on `DagArtifact`; `standard`/`subset` match this
/// artifact's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is
/// `s.dag.dag@1/*#editor` / `s.dag.dag@1/*#viewer`.
pub const DAG_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect {
    artifact_kind: "s.dag.dag",
    standard: semio_framework_plugin::app::StandardId("1"),
    subset: semio_framework_plugin::app::SubsetId::ANY,
};

pub use crate::artifacts::dag::snapshot::schema::{default_snapshot, DagSnapshot};
pub use infinite_board_port_directed_dag::{DagEdgePatch, DagFixtureEdge, DagNodeKind, DagNodePatch, DagNodeSpec, DagPreviewContent, IoPortSpec};

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio.graph` document — the dag plugin's
/// nodes/edges now live in this composed child's `nodes`/`edges` rather than inline on `DagSnapshot`.
pub type DagContentChild = store::ArtifactChild<SemioGraphSnapshot>;

use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{
    GraphEdgeId as SemioGraphEdgeId, GraphNodeId as SemioGraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphPort, SemioGraphPortKind, SemioGraphSnapshot, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA,
};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};

/// 🏷️ `dag.node` is the honest string boundary carrying the FULL `DagNodeSpec` (every field this
/// plugin's rich node-kind enum can hold — computation/slider/select/screen/note/image/preview/
/// action/export/cluster/appInstance, all with their own field sets) as JSON. `id`/`label`/
/// `position` are ALSO projected onto the composed `SemioGraphNode`'s own native fields (and `ports`
/// is a best-effort projection of `node.inputs()`/`node.outputs()`) for genuine graph-shape tooling
/// that only understands the neutral subset — but the JSON blob is the round-trip SOURCE OF TRUTH on
/// decode, since `SemioGraphNode.properties` is the only slot this subset offers wide enough to carry
/// a whole rich node kind losslessly (matches `flow`'s own "honest string boundary" precedent).
const DAG_NODE_JSON_PROPERTY: &str = "dag.node";

async fn semio_node_from_dag_node(node: &DagNodeSpec) -> SemioGraphNode {
    let ports = node.inputs().iter().map(|port| SemioGraphPort { name: port.id.clone(), kind: SemioGraphPortKind::In }).chain(node.outputs().iter().map(|port| SemioGraphPort { name: port.id.clone(), kind: SemioGraphPortKind::Out })).collect();
    SemioGraphNode {
        id: SemioGraphNodeId::new(node.id.clone()),
        kind: infinite_board_port_directed_dag::dag_node_kind_tag(&node.kind).to_string(),
        label: node.name.clone(),
        position: SemioPoint2 { x: node.x, y: node.y },
        ports,
        properties: vec![SemioValueEntry { key: DAG_NODE_JSON_PROPERTY.into(), value: SemioValue::Str { value: serde_json::to_string(node).unwrap_or_default() } }],
    }
}

/// 🌉 Inverse of [`semio_node_from_dag_node`] — reconstructs the exact `DagNodeSpec` from its
/// `dag.node` JSON property. Falls back to a minimal computation node built from the graph-native
/// `id`/`label`/`position` fields only if the property is missing (content authored outside this
/// plugin, e.g. by a hand-written `graph` doc) — never panics.
async fn dag_node_from_semio_node(node: &SemioGraphNode) -> DagNodeSpec {
    for property in &node.properties {
        if property.key == DAG_NODE_JSON_PROPERTY {
            if let SemioValue::Str { value } = &property.value {
                if let Ok(parsed) = serde_json::from_str::<DagNodeSpec>(value) {
                    return parsed;
                }
            }
        }
    }
    DagNodeSpec { id: node.id.value.clone(), name: node.label.clone(), x: node.position.x, y: node.position.y, ..Default::default() }
}

/// 🏷️ `SemioGraphEdge` has no `properties` slot (unlike `SemioGraphNode`) — its `label` field (which
/// this plugin's own `DagFixtureEdge` never populates on its own behalf) is repurposed to carry the
/// FULL `DagFixtureEdge` (port-qualified `source`/`target` endpoint strings, `route_style`,
/// `properties`) as JSON, the round-trip source of truth on decode. `source`/`target`/`kind` are also
/// projected onto their native fields (node-id-only, port suffix stripped) for genuine graph-shape
/// tooling.
async fn semio_edge_from_dag_edge(edge: &DagFixtureEdge) -> SemioGraphEdge {
    let (source_node, _) = split_endpoint(&edge.source);
    let (target_node, _) = split_endpoint(&edge.target);
    SemioGraphEdge { id: SemioGraphEdgeId::new(edge.id.clone()), source: SemioGraphNodeId::new(source_node), target: SemioGraphNodeId::new(target_node), kind: "dag-edge".into(), label: serde_json::to_string(edge).unwrap_or_default() }
}

/// 🌉 Inverse of [`semio_edge_from_dag_edge`] — falls back to a bare node-id edge (no route
/// style/properties) if `label` isn't valid `DagFixtureEdge` JSON (content authored outside this
/// plugin) — never panics.
async fn dag_edge_from_semio_edge(edge: &SemioGraphEdge) -> DagFixtureEdge {
    serde_json::from_str::<DagFixtureEdge>(&edge.label).unwrap_or_else(|_| DagFixtureEdge { id: edge.id.value.clone(), source: edge.source.value.clone(), target: edge.target.value.clone(), ..Default::default() })
}

async fn split_endpoint(endpoint: &str) -> (String, String) {
    crate::artifacts::dag::schema::split_endpoint(endpoint)
}

/// 🌉 REAL bidirectional converter between the app's live `DagNodeSpec`/`DagFixtureEdge` editing
/// state and the composed child's own `SemioGraphSnapshot` node/edge graph (the
/// "ModelBridge"/"DocumentBridge" pattern from `📓️wave3-reports/cad-report.md` and
/// `📓️wave4-reports/flow-report.md`).
pub async fn dag_content_snapshot_from_working(nodes: &[DagNodeSpec], edges: &[DagFixtureEdge]) -> SemioGraphSnapshot {
    SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: nodes.iter().map(semio_node_from_dag_node).collect(), edges: edges.iter().map(semio_edge_from_dag_edge).collect() }
}

/// 🌉 Inverse of [`dag_content_snapshot_from_working`].
pub async fn working_from_dag_content_snapshot(content: &SemioGraphSnapshot) -> (Vec<DagNodeSpec>, Vec<DagFixtureEdge>) {
    (content.nodes.iter().map(dag_node_from_semio_node).collect(), content.edges.iter().map(dag_edge_from_semio_edge).collect())
}

/// 🕸️ Deterministic content-addressed CHILD handle for the dag content — same `(child_id, target)`
/// for identical `(nodes, edges)`, a different pair once the content actually changes; mirrors
/// flow's `flow_content_child_handle`/writer's `document_child_handle`.
pub async fn dag_content_child_handle(nodes: &[DagNodeSpec], edges: &[DagFixtureEdge]) -> DagContentChild {
    use std::hash::{Hash, Hasher};
    let snapshot = dag_content_snapshot_from_working(nodes, edges);
    let content_json = serde_json::to_string(&snapshot).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("dag-content-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "graph".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "dag-content".into(), dialect };
    store::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️ContentBridge

//#region 🔖️WorkingScene
/// 🌱 Ephemeral, session-side working representation of the composed content child's live
/// nodes/edges — NEVER persisted, NEVER a durable field on `DagSnapshot` itself (matches the
/// `EngineRep` contract: wholly derived, droppable at any instant, rebuilt from base). Exists
/// because no `LinkResolver`/child-dispatch seam is wired into `ArtifactApp::handle` yet (checked
/// directly against `🔌️plugin/🦀️component.rs` — same standing gap cad/lowpoly/writer/flow's reports
/// all document); until one exists, the only way a persisted content-addressed HANDLE can round-trip
/// to real nodes/edges within one process is this cache, keyed by `DagContentChild::child_id` —
/// mirrors `FlowWorkingScene`/`WriterWorkingScene`/`LowpolyScratch.mesh_workspace`.
///
/// ⚠️ Same documented gap as flow/lowpoly/writer: store-level undo/redo bypasses
/// `ArtifactApp::handle` entirely, and a bare `parse_dsl`/`decode_pack` of persisted bytes recovers
/// only the opaque handle, never the content (the child's real payload lives in its own,
/// not-yet-resolvable, child store). `dag_working_scene`/`dag_working_scene_for_handle` fail soft
/// (an empty scene) rather than panicking. A real fix needs child-document resolution, which no
/// WASM-guest plugin in this repo has yet.
#[derive(Clone, Debug, Default)]
pub struct DagWorkingScene {
    pub nodes: Vec<DagNodeSpec>,
    pub edges: Vec<DagFixtureEdge>,
}

thread_local! {
    static DAG_SCRATCH: std::cell::RefCell<std::collections::HashMap<String, DagWorkingScene>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

/// 📝 Seeds the scratch cache for a handle — call whenever new nodes/edges content is about to
/// become a document's `content` field (every mutation-diff/fixture builder in this plugin does, via
/// [`dag_content_child_handle_and_cache`]).
pub async fn cache_dag_content(child_id: &str, nodes: Vec<DagNodeSpec>, edges: Vec<DagFixtureEdge>) {
    DAG_SCRATCH.with(|cache| cache.borrow_mut().insert(child_id.to_string(), DagWorkingScene { nodes, edges }));
}

/// 🔎 Reads the cached live scene for a content child handle — an empty scene (never a panic) when
/// nothing has cached it yet (see this region's module doc comment for why that can happen).
pub async fn dag_working_scene_for_handle(handle: &DagContentChild) -> DagWorkingScene {
    DAG_SCRATCH.with(|cache| cache.borrow().get(&handle.child_id).cloned()).unwrap_or_default()
}

/// 🔎 Reads the current document's live nodes/edges off its `content` child handle — the single read
/// call site every mutation diff/inverse/app command in this plugin uses instead of the old
/// `snapshot.nodes`/`.edges` field access.
pub async fn dag_working_scene(snapshot: &DagSnapshot) -> DagWorkingScene {
    dag_working_scene_for_handle(&snapshot.content)
}

/// 🏗️ Mints a new content-addressed handle AND seeds the scratch cache with its scene in one call —
/// the standard way every mutation-diff/fixture builder in this plugin creates a `content` field
/// value; never construct a handle without also caching, or [`dag_working_scene`] will read back
/// empty.
pub async fn dag_content_child_handle_and_cache(nodes: Vec<DagNodeSpec>, edges: Vec<DagFixtureEdge>) -> DagContentChild {
    let handle = dag_content_child_handle(&nodes, &edges);
    cache_dag_content(&handle.child_id, nodes, edges);
    handle
}
//#endregion 🔖️WorkingScene

//#region 🔖️Domain
/// 🎥️ Viewport camera for the DAG canvas (plugin-owned; distinct from framework `dag` kernel helpers).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DagCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for DagCamera {
    async fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

impl From<DagCamera> for infinite_board_port_directed_dag::DagCamera {
    async fn from(value: DagCamera) -> Self {
        Self { x: value.x, y: value.y, zoom: value.zoom }
    }
}

impl From<infinite_board_port_directed_dag::DagCamera> for DagCamera {
    async fn from(value: infinite_board_port_directed_dag::DagCamera) -> Self {
        Self { x: value.x, y: value.y, zoom: value.zoom }
    }
}
//#endregion 🔖️Domain

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest.
pub async fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "graph.dag".into(),
        name: "DAG".into(),
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
/// exception, still called from `🕸️dag/🦀️component.rs`'s own `.setup()`: it registers the `DagPlayApp`
/// CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has no field for
/// (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's artifact-scoped
/// function set. Relocated from `⚙️engine` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// reloc-g2): `declaration()` describes the artifact (kind, schema, io ports, ownership), which is not
/// engine behaviour.
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.dag.standard.v1", "standard", "1", &[], None),
        ("s.dag.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.dag.schema.artifact", "schema", "s.dag.dag", &[("schema", "s.dag.dag")], None),
        ("s.dag.inference.artifact", "inference", "s.dag.dag.inference", &[("schema", "s.dag.dag.inference")], None),
        ("s.dag.composer.native", "composer", "s.dag@1/*", &[("dialect", "s.dag@1/*")], None),
        ("s.dag.composer.format-1", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.dag.composer.format-2", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.dag.grammar.1", "grammar", "dag.document", &[("grammar", "dag.document")], None),
        ("s.dag.grammar.2", "grammar", "dag.op", &[("grammar", "dag.op")], None),
        ("s.dag.grammar.3", "grammar", "dag.diff", &[("grammar", "dag.diff")], None),
        ("s.dag.grammar.4", "grammar", "dag.pack", &[("grammar", "dag.pack")], None),
        ("s.dag.grammar.5", "grammar", "dag.spr", &[("grammar", "dag.spr")], None),
        ("s.dag.codec.document-1", "codec", "dag.dag:dag", &[("codec", "dag.dag"), ("extension", "dag")], None),
        ("s.dag.localization.en", "localization", "DAG", &[], Some(("en", "DAG"))),
        ("s.dag.localization.de", "localization", "Gerichteter azyklischer Graph", &[], Some(("de", "Gerichteter azyklischer Graph"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.dag")?);
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
pub async fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.dag.dag").expect("canonical dag kind"), localization: &[], standards: vec![crate::artifacts::dag::standards::v1::standard()] }
}
//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn artifact_kind_declares_the_graph_dag_component_kind() {
        assert_eq!(artifact_kind().id, "graph.dag");
        assert_eq!(artifact_kind().schema, DAG_DOCUMENT_SCHEMA);
    }

    #[test]
    async fn default_snapshot_matches_document_schema() {
        assert_eq!(default_snapshot().schema, DAG_DOCUMENT_SCHEMA);
    }

    #[test]
    async fn node_edge_content_round_trips_through_the_composed_child_snapshot() {
        let document = default_snapshot();
        let scene = dag_working_scene(&document);
        let content = dag_content_snapshot_from_working(&scene.nodes, &scene.edges);
        let (nodes, edges) = working_from_dag_content_snapshot(&content);
        assert_eq!(nodes, scene.nodes);
        assert_eq!(edges, scene.edges);
    }
}
//#endregion 🧪️Tests
