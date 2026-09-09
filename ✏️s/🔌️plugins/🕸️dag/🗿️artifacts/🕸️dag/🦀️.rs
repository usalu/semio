//! 🔀️ DAG artifact — the document entity this plugin's app edits.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`reasoning/dag→C:graph`): the old inline
//! `nodes`/`edges` fields are replaced by a composed `s.stdio.semio.graph` CHILD slot
//! (`🔖️ContentBridge` below) — this plugin no longer defines its own persisted node/edge model, it
//! composes stdio's neutral `graph` subset instead. The rich live editing types
//! (`semio_framework_artifact_infinite_dag::DagNodeSpec`/`DagNodeKind`/`DagFixtureEdge`) still flow
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

pub use crate::snapshot::schema::default_snapshot;
pub use semio_framework_artifact_infinite_dag::{DagEdgePatch, DagFixtureEdge, DagNodeKind, DagNodePatch, DagNodeSpec, DagPreviewContent, IoPortSpec};

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio.graph` document — the dag plugin's
/// nodes/edges now live in this composed child's `nodes`/`edges` rather than inline on `DagSnapshot`.
pub type DagContentChild = store::ArtifactChild<SemioGraphSnapshot>;

use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::{
    GraphEdgeId as SemioGraphEdgeId, GraphNodeId as SemioGraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphPort, SemioGraphPortKind, SemioGraphSnapshot, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA,
};
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};

/// 🏷️ `dag.node` is the honest string boundary carrying the FULL `DagNodeSpec` (every field this
/// plugin's rich node-kind enum can hold — computation/slider/select/screen/note/image/preview/
/// action/export/cluster/appInstance, all with their own field sets) as JSON. `id`/`label`/
/// `position` are ALSO projected onto the composed `SemioGraphNode`'s own native fields (and `ports`
/// is a best-effort projection of `node.inputs()`/`node.outputs()`) for genuine graph-shape tooling
/// that only understands the neutral subset — but the JSON blob is the round-trip SOURCE OF TRUTH on
/// decode, since `SemioGraphNode.properties` is the only slot this subset offers wide enough to carry
/// a whole rich node kind losslessly (matches `flow`'s own "honest string boundary" precedent).
const DAG_NODE_JSON_PROPERTY: &str = "dag.node";

fn semio_node_from_dag_node(node: &DagNodeSpec) -> SemioGraphNode {
    let ports = node.inputs().iter().map(|port| SemioGraphPort { name: port.id.clone(), kind: SemioGraphPortKind::In }).chain(node.outputs().iter().map(|port| SemioGraphPort { name: port.id.clone(), kind: SemioGraphPortKind::Out })).collect();
    SemioGraphNode {
        id: SemioGraphNodeId::new(node.id.clone()),
        kind: semio_framework_artifact_infinite_dag::dag_node_kind_tag(&node.kind).to_string(),
        label: node.name.clone(),
        position: SemioPoint2 { x: node.x, y: node.y },
        ports,
        properties: vec![SemioValueEntry { key: DAG_NODE_JSON_PROPERTY.into(), value: SemioValue::Str { value: dsl::json::to_json_string(node) } }],
    }
}

/// 🌉 Inverse of [`semio_node_from_dag_node`] — reconstructs the exact `DagNodeSpec` from its
/// `dag.node` JSON property. Falls back to a minimal computation node built from the graph-native
/// `id`/`label`/`position` fields only if the property is missing (content authored outside this
/// plugin, e.g. by a hand-written `graph` doc) — never panics.
fn dag_node_from_semio_node(node: &SemioGraphNode) -> DagNodeSpec {
    for property in &node.properties {
        if property.key == DAG_NODE_JSON_PROPERTY {
            if let SemioValue::Str { value } = &property.value {
                if let Ok(parsed) = dsl::json::from_json_str::<DagNodeSpec>(value) {
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
fn semio_edge_from_dag_edge(edge: &DagFixtureEdge) -> SemioGraphEdge {
    let (source_node, _) = split_endpoint(&edge.source);
    let (target_node, _) = split_endpoint(&edge.target);
    SemioGraphEdge { id: SemioGraphEdgeId::new(edge.id.clone()), source: SemioGraphNodeId::new(source_node), target: SemioGraphNodeId::new(target_node), kind: "dag-edge".into(), label: dsl::json::to_json_string(edge) }
}

/// 🌉 Inverse of [`semio_edge_from_dag_edge`] — falls back to a bare node-id edge (no route
/// style/properties) if `label` isn't valid `DagFixtureEdge` JSON (content authored outside this
/// plugin) — never panics.
fn dag_edge_from_semio_edge(edge: &SemioGraphEdge) -> DagFixtureEdge {
    dsl::json::from_json_str::<DagFixtureEdge>(&edge.label).unwrap_or_else(|_| DagFixtureEdge { id: edge.id.value.clone(), source: edge.source.value.clone(), target: edge.target.value.clone(), ..Default::default() })
}

fn split_endpoint(endpoint: &str) -> (String, String) {
    schema::split_endpoint(endpoint)
}

/// 🌉 REAL bidirectional converter between the app's live `DagNodeSpec`/`DagFixtureEdge` editing
/// state and the composed child's own `SemioGraphSnapshot` node/edge graph (the
/// "ModelBridge"/"DocumentBridge" pattern from `📓️wave3-reports/cad-report.md` and
/// `📓️wave4-reports/flow-report.md`).
pub fn dag_content_snapshot_from_working(nodes: &[DagNodeSpec], edges: &[DagFixtureEdge]) -> SemioGraphSnapshot {
    SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: nodes.iter().map(semio_node_from_dag_node).collect(), edges: edges.iter().map(semio_edge_from_dag_edge).collect() }
}

/// 🌉 Inverse of [`dag_content_snapshot_from_working`].
pub fn working_from_dag_content_snapshot(content: &SemioGraphSnapshot) -> (Vec<DagNodeSpec>, Vec<DagFixtureEdge>) {
    (content.nodes.iter().map(dag_node_from_semio_node).collect(), content.edges.iter().map(dag_edge_from_semio_edge).collect())
}

/// 🕸️ Deterministic content-addressed CHILD handle for the dag content — same `(child_id, target)`
/// for identical `(nodes, edges)`, a different pair once the content actually changes; mirrors
/// flow's `flow_content_child_handle`/writer's `document_child_handle`.
pub fn dag_content_child_handle(nodes: &[DagNodeSpec], edges: &[DagFixtureEdge]) -> DagContentChild {
    use std::hash::{Hash, Hasher};
    let snapshot = dag_content_snapshot_from_working(nodes, edges);
    let content_json = dsl::json::to_json_string(&snapshot);
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
/// 🌱 Ephemeral representation of one composed child's live nodes and edges. The value is attached
/// to the exact `ArtifactChild`; it is never persisted, never global, and is retired with that owner.
#[derive(Clone, Debug, Default)]
pub struct DagWorkingScene {
    pub nodes: Vec<DagNodeSpec>,
    pub edges: Vec<DagFixtureEdge>,
}

/// 🔎 Retains this exact child's typed working owner. A wire-only handle fails soft until the host
/// materializes its child document.
pub fn dag_working_scene_for_handle(handle: &DagContentChild) -> DagWorkingScene {
    handle.local_owner::<DagWorkingScene>().map(|scene| scene.as_ref().clone()).unwrap_or_default()
}

/// 🔎 Reads the current document's live nodes/edges off its `content` child handle — the single read
/// call site every mutation diff/inverse/app command in this plugin uses instead of the old
/// `snapshot.nodes`/`.edges` field access.
pub fn dag_working_scene(snapshot: &DagSnapshot) -> DagWorkingScene {
    dag_working_scene_for_handle(&snapshot.content)
}

/// 🏗️ Mints one content-addressed child and transfers its immutable working scene into that exact
/// local owner. No matching identity in another snapshot can observe the payload.
pub fn dag_content_child_with_owner(nodes: Vec<DagNodeSpec>, edges: Vec<DagFixtureEdge>) -> DagContentChild {
    let handle = dag_content_child_handle(&nodes, &edges);
    handle.with_local_owner(std::sync::Arc::new(DagWorkingScene { nodes, edges }))
}
//#endregion 🔖️WorkingScene

//#region 🔖️Domain
/// 🎥️ Viewport camera for the DAG canvas (plugin-owned; distinct from framework `dag` kernel helpers).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
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
        ("s.dag.dag.composer.format-1", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
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
    semio_framework_plugin::PluginApp + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::dag::DagPlayApp>>> + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::dag::DagViewer>>>
{
}

impl<A> DagApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::dag::DagPlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::dag::DagViewer>>>
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
                        #[path = "."]
                        pub mod create_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🧪️tests/🚫️rejects-a-duplicate-node-id/🦀️.rs"]
                            mod tests_rejects_a_duplicate_node_id;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/🚫️rejects-deleting-a-missing-node/🦀️.rs"]
                            mod tests_rejects_deleting_a_missing_node;
                        }
                        #[path = "."]
                        pub mod rename_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-node/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-node/🧪️tests/🚫️rejects-renaming-a-missing-node/🦀️.rs"]
                            mod tests_rejects_renaming_a_missing_node;
                        }
                        #[path = "."]
                        pub mod change_node_name {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-node-name/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-node-name/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-node-name/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-node-name/🧪️tests/🚫️rejects-renaming-the-f90723/🦀️.rs"]
                            mod tests_rejects_renaming_the_label_of_a_missing_node;
                        }
                        #[path = "."]
                        pub mod move_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-node/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-node/🧪️tests/🚫️rejects-moving-a-missing-node/🦀️.rs"]
                            mod tests_rejects_moving_a_missing_node;
                        }
                        #[path = "."]
                        pub mod resize_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-node/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-node/🧪️tests/🚫️rejects-resizing-a-missing-node/🦀️.rs"]
                            mod tests_rejects_resizing_a_missing_node;
                        }
                        #[path = "."]
                        pub mod change_node_icon {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-node-icon/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-node-icon/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-node-icon/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-node-icon/🧪️tests/🚫️rejects-reiconing-a-3d1997/🦀️.rs"]
                            mod tests_rejects_reiconing_a_missing_node;
                        }
                        #[path = "."]
                        pub mod change_node_abbreviation {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡change-node-abbreviation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡change-node-abbreviation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡change-node-abbreviation/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡change-node-abbreviation/🧪️tests/🚫️rejects-reabbrevi-46aeee/🦀️.rs"]
                            mod tests_rejects_reabbreviating_a_missing_node;
                        }
                        #[path = "."]
                        pub mod change_node_operator_kind {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-node-operator-kind/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-node-operator-kind/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-node-operator-kind/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-node-operator-kind/🧪️tests/🔗️rejects-rebinding-45dad2/🦀️.rs"]
                            mod tests_rejects_rebinding_the_operator_of_a_missing_node;
                        }
                        #[path = "."]
                        pub mod replace_node_kind {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-node-kind/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-node-kind/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-node-kind/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-node-kind/🧪️tests/🚫️rejects-rekinding-a-8bbfaf/🦀️.rs"]
                            mod tests_rejects_rekinding_a_missing_node;
                        }
                        #[path = "."]
                        pub mod replace_node_properties {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️replace-node-properties/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️replace-node-properties/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️replace-node-properties/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️replace-node-properties/🧪️tests/🚫️rejects-repropert-1ab92a/🦀️.rs"]
                            mod tests_rejects_repropertying_a_missing_node;
                        }
                        #[path = "."]
                        pub mod reorder_nodes {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-nodes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-nodes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-nodes/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-nodes/🧪️tests/🚫️rejects-a-duplicate-id-c304e1/🦀️.rs"]
                            mod tests_rejects_a_duplicate_id_in_the_order;
                        }
                        #[path = "."]
                        pub mod connect_nodes {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-nodes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-nodes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-nodes/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-nodes/🧪️tests/🚫️rejects-a-missing-source-node/🦀️.rs"]
                            mod tests_rejects_a_missing_source_node;
                        }
                        #[path = "."]
                        pub mod disconnect_nodes {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🧪️tests/🚫️rejects-disconnecting-ab5dfa/🦀️.rs"]
                            mod tests_rejects_disconnecting_a_missing_edge;
                        }
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
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
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
                                pub mod svg {
                                    #[path = "."]
                                    pub mod v1_1 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
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
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                pub mod svg {
                                    #[path = "."]
                                    pub mod v1_1 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
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
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
pub mod op {
    pub use crate::standards::v1::subsets::any::io::mutations::text::*;
    pub use crate::standards::v1::subsets::any::schema::mutations::DagMutation;
}
pub mod document_dsl {
    pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
}
pub mod spr {
    pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
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
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-dag-nodes/🦀️.rs"]
            pub mod patch_dag_nodes;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-node/🦀️.rs"]
            pub mod remove_node;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️rename-dag-node/🦀️.rs"]
            pub mod rename_dag_node;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️reorganize/🦀️.rs"]
            pub mod reorganize;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

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
