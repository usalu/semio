//! ⚙️ S Studio app — headless compute (constitutional: engine).
//!
//! 🕳️ Deviation from the usual "engine" content: every function here computes over
//! `semio_framework_os::{WorkflowDocument, WorkflowOperation}` (a foreign type owned by the kernel
//! `workflow` crate via os-core's re-export, not a local document this plugin defines — the dissolved
//! `OsProjection`/`OsOperation`'s successors, see `## The inversion`) — building `WorkflowOperation`
//! values from arguments is still pure compute, not an `apply_X_operation` match on a LOCALLY-owned
//! enum, so it carries no circular-dependency risk against a local `op` crate (this app has none — see
//! `space_op`'s own doc comment).

use infinite_board_port_directed_dag::{dag_fixture_to_wire_literal, DagCamera, DagFixture, DagFixtureEdge, DagNodeKind, DagNodeSpec, IoPortSpec};
use semio_framework_os::{
    create_default_workflow_parameter, create_os_id, media_port_spec_id, negotiate_media_contract, os_app_registration, patch_workflow_parameter, resolve_os_app_definition, workflow_node_for_app, workflow_parameter_id_from_port_id, MediaContract,
    WorkflowDocument, WorkflowOperation, WorkflowParameter, WorkflowParameterBinding, WorkflowParameterType, WorkflowPosition,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use space::SpaceWindowCamera;
use std::collections::{BTreeMap, HashMap};

//#region 🔖️Config
/// 🧮️ B1: space's real `DocumentApp::Config` — the studio app's config artifact. Absorbs every field
/// `StudioRuntimeState` (formerly held in a `RefCell` on the `SpaceApp` struct) and the deleted
/// `ViewState.panel_json`/`SpacePanelState.active_panel_tab`/`ViewState.locale` used to carry —
/// selection/hover/clipboard/camera/engagement inputs/pending-import/presence identity/active panel
/// tab/locale all round-trip through the config `DocumentStore` now, each with a real `backwards` via
/// `space_op::SpaceConfigOperation`, instead of never being VCS'd at all. A node IS the app instance now
/// (see the kernel `🔁️workflow` crate's `🔖️InstanceIdentity` doc), so the old disjoint
/// `selected_media_node_ids`/`selected_app_instance_ids`/`clipboard_instance_ids` pairs collapse into
/// one `*_node_ids` field apiece. `camera`/per-window options are keyed by window id (`BTreeMap<String,
/// _>`, per the Configured Node Apps recipe) — today that's always `space::S_PLAY_WINDOW_WORKFLOW`,
/// since split-pane window *instances* aren't a thing anywhere in this codebase yet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "spacecfg")]
#[dsl(layout = "lines")]
pub struct SpaceConfig {
    /// 🎥️ Workflow-canvas camera, keyed by window id — was `StudioRuntimeState::workflow_camera`.
    pub camera: BTreeMap<String, SpaceWindowCamera>,
    /// 👁️ Selected workflow-node ids — was `selected_media_node_ids`/`selected_app_instance_ids`.
    pub selected_node_ids: Vec<String>,
    /// 👁️ Hovered workflow-node id — was `hovered_media_node_id`.
    pub hovered_node_id: Option<String>,
    /// 🗂️ Collapsed workflow nodes — node-preview UI state (Wave 3/4), not yet driven by any command.
    pub collapsed_node_ids: Vec<String>,
    /// 🖼️ Workflow nodes with their live preview thumbnail turned off — node-preview UI state (Wave 4),
    /// not yet driven by any command.
    pub preview_off_node_ids: Vec<String>,
    /// 👁️ The "active app" measure selection — was `active_instance_id`.
    pub active_node_id: Option<String>,
    /// 👁️ The node currently open in its own plugin window — was `focused_instance_id`.
    pub focused_node_id: Option<String>,
    /// 📋️ Copied node ids, pasted by `duplicateAppInstance`/`pasteAppInstance` — was
    /// `clipboard_instance_ids`.
    pub clipboard_node_ids: Vec<String>,
    pub workflow_engagement_input: String,
    pub compiled_dag_engagement_input: String,
    /// 📥️ In-flight media-import target — was `pending_import_instance_id`.
    pub pending_import_node_id: Option<String>,
    pub pending_import_format: Option<String>,
    /// 👁️ Active studio panel tab — was host-round-tripped through the deleted `ViewState.panel_json`
    /// (`SpacePanelState.active_panel_tab`); a real config field now.
    pub active_panel_tab: String,
    /// 🌱️ The currently open studio document's catalog id — was `StudioRuntimeState::space_id`.
    pub space_id: Option<String>,
    /// 🫀️ This session's local presence identity — was `client_id`/`client_name`.
    pub client_id: Option<String>,
    pub client_name: Option<String>,
    /// 🗣️ BCP-47 locale tag — was read off the deleted `ViewState.locale`.
    pub locale: String,
}

impl Default for SpaceConfig {
    fn default() -> Self {
        Self {
            camera: BTreeMap::new(),
            selected_node_ids: Vec::new(),
            hovered_node_id: None,
            collapsed_node_ids: Vec::new(),
            preview_off_node_ids: Vec::new(),
            active_node_id: None,
            focused_node_id: None,
            clipboard_node_ids: Vec::new(),
            workflow_engagement_input: String::new(),
            compiled_dag_engagement_input: String::new(),
            pending_import_node_id: None,
            pending_import_format: None,
            active_panel_tab: space::S_PLAY_CATALOGUE_TAB_ID.into(),
            space_id: None,
            client_id: None,
            client_name: None,
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(SpaceConfig);

//#endregion 🔖️Config

//#region 🔖️Parameters
pub fn parameter_entity_id(parameter: &WorkflowParameter) -> &str {
    match parameter {
        WorkflowParameter::Numeric { id, .. } | WorkflowParameter::Categorical { id, .. } | WorkflowParameter::Toggle { id, .. } | WorkflowParameter::Text { id, .. } => id,
    }
}

pub fn parameter_name(parameter: &WorkflowParameter) -> &str {
    match parameter {
        WorkflowParameter::Numeric { name, .. } | WorkflowParameter::Categorical { name, .. } | WorkflowParameter::Toggle { name, .. } | WorkflowParameter::Text { name, .. } => name,
    }
}

pub trait OsParameterId {
    fn id(&self) -> &str;
}

impl OsParameterId for WorkflowParameter {
    fn id(&self) -> &str {
        parameter_entity_id(self)
    }
}

/// @emoji ➕️ Builds an `AddParameter` operation with a fresh default parameter of the requested type.
pub fn add_parameter_operation(parameter_type: &WorkflowParameterType, name: &str) -> WorkflowOperation {
    WorkflowOperation::AddParameter { parameter: create_default_workflow_parameter(parameter_type, name, None) }
}

/// @emoji 🩹️ Builds a `PatchParameter` operation by folding `patch` (a `{field: value}` object) into the
/// current parameter — the store-free operation-builder used in place of os-core's
/// `OsWorkflowStore::patch_parameter`.
pub fn patch_parameter_operation(projection: &WorkflowDocument, parameter_id: &str, patch: &Value) -> Option<WorkflowOperation> {
    let current = projection.parameters.iter().find(|parameter| parameter_entity_id(parameter) == parameter_id)?;
    Some(WorkflowOperation::PatchParameter { parameter_id: parameter_id.into(), parameter: patch_workflow_parameter(current, patch) })
}
//#endregion 🔖️Parameters

//#region 🔖️WorkflowNodes
/// @emoji ✨️ Builds the `AddNode` operation (minting a fresh node — id, ports, document/config refs,
/// everything — via `workflow_node_for_app`, so replay never re-derives it) plus the new node's id for
/// the caller to focus. The store-free operation-builder the plugin uses in place of os-core's
/// `OsWorkflowStore::add_workflow_node` (a `DocumentApp` owns no store — its wrapper does; this
/// operation-builder form also can't dispatch the `space::SpaceOperation::InstallProgram` sibling op
/// `OsWorkflowStore::add_workflow_node` now does — that only applies to the live-store path, tracked as
/// a gap for whichever wave wires this plugin's spawn command through a real `OsWorkflowStore`).
/// Renamed from the pre-merge `spawn_app_instance_operation`: a node IS the app instance now, there is
/// no separate `OsAppInstance` to mint alongside it.
pub fn add_workflow_node_operation(plugin_id: &str, app_id: &str, label: Option<&str>, x: f64, y: f64) -> Option<(WorkflowOperation, String)> {
    let app = resolve_os_app_definition(plugin_id, app_id)?;
    let node_id = create_os_id("node");
    let position = WorkflowPosition { x, y, width: 0.0, height: 0.0 };
    let mut node = workflow_node_for_app(&app, plugin_id, &node_id, &position);
    if let Some(label) = label {
        node.label = label.into();
    }
    Some((WorkflowOperation::AddNode { node }, node_id))
}
//#endregion 🔖️WorkflowNodes

//#region 🔖️MediaContractConnect
/// @emoji 🤝️ Resolves the source/target `WorkflowMediaPort`s for a proposed connect from the live
/// projection and negotiates their wire contract — shared by both connect entry points
/// (`"connectMediaPorts"` and the `nodeGraphEdit`/`"connect"` fixture edit) so neither can push a
/// `WorkflowOperation::ConnectPorts` for an incompatible or unresolved pair of ports. Operates directly
/// on `WorkflowNode`/`WorkflowMediaPort` — a node's ports are typed `MediaPortSpec`s now, no more
/// string `artifact_kind` join through a separate `OsMediaPort`.
pub fn negotiate_media_connect(projection: &WorkflowDocument, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> Result<MediaContract, String> {
    let source_port =
        projection.graph.nodes.iter().find(|node| node.id == source_node_id).and_then(|node| node.outputs.iter().find(|port| port.id == source_port_id)).ok_or_else(|| format!("unknown source port {source_node_id}:{source_port_id}"))?;
    let target_port =
        projection.graph.nodes.iter().find(|node| node.id == target_node_id).and_then(|node| node.inputs.iter().find(|port| port.id == target_port_id)).ok_or_else(|| format!("unknown target port {target_node_id}:{target_port_id}"))?;
    negotiate_media_contract(source_port, target_port)
}
//#endregion 🔖️MediaContractConnect

//#region 🔖️MediaVfs
// 🚧️ `list_os_workflow_vfs_children`/`OsWorkflowVfsNodeRecord` were deleted with os-core's
// `🔖️WorkflowVfs` region (the os-core dissolve, step 6) — a full collection-browser UI replaces the
// workflow-instance VFS in a later wave (see the plan's `Addressing`/collection design rulings).
// `flatten_media_vfs_rows` is kept as a minimal compiling stub (always empty) so this crate's callers
// (`space_ui`) still compile; `vfs_node_to_row`/`media_port_label` still have real bodies since they
// don't depend on the deleted types.
pub fn flatten_media_vfs_rows(_parent_id: &str, _graph: &semio_framework_os::Workflow, _bindings: &[WorkflowParameterBinding], _parameters: &[WorkflowParameter], _rows: &mut Vec<Value>) {}

pub fn media_port_label(port_id: &str, parameter_by_id: &HashMap<String, &WorkflowParameter>) -> String {
    workflow_parameter_id_from_port_id(port_id).and_then(|id| parameter_by_id.get(&id).map(|row| parameter_name(row).to_string())).or_else(|| media_port_spec_id(port_id)).unwrap_or_else(|| port_id.to_string())
}
//#endregion 🔖️MediaVfs

//#region 🔖️CompiledDag
/// @emoji 🕸️ Projects the workflow onto the generic port-directed-DAG fixture the Compiled DAG window
/// renders — every `WorkflowNode` becomes one `DagNodeKind::AppInstance` directly (node IS instance
/// now; no separate join through `OsAppInstance`).
pub fn workflow_to_dag_fixture(projection: &WorkflowDocument) -> DagFixture {
    let parameter_by_id: HashMap<_, _> = projection.parameters.iter().map(|row| (parameter_entity_id(row).to_string(), row)).collect();
    let nodes = projection
        .graph
        .nodes
        .iter()
        .map(|node| {
            let registration = os_app_registration(&node.plugin_id, &node.app_id);
            let icon = format!("emoji:{}", registration.map(|row| row.component_kind.clone()).unwrap_or_else(|| "s".into()));
            DagNodeSpec {
                id: node.id.clone(),
                name: node.label.clone(),
                abbreviation: if node.app_id.chars().count() <= 3 { node.app_id.clone() } else { node.app_id.chars().take(3).collect() },
                icon: icon.clone(),
                x: node.x + node.width / 2.0,
                y: node.y + node.height / 2.0,
                width: node.width,
                height: node.height,
                operator_kind: Some(node.plugin_id.clone()),
                kind: DagNodeKind::AppInstance {
                    instance_id: node.id.clone(),
                    plugin_id: node.plugin_id.clone(),
                    app_id: node.app_id.clone(),
                    icon,
                    inputs: node
                        .inputs
                        .iter()
                        .map(|port| {
                            let mut spec = IoPortSpec::simple(&port.id, media_port_label(&port.id, &parameter_by_id));
                            spec.artifact_kind = port.spec.kind_id.clone();
                            spec
                        })
                        .collect(),
                    outputs: node
                        .outputs
                        .iter()
                        .map(|port| {
                            let mut spec = IoPortSpec::simple(&port.id, media_port_label(&port.id, &parameter_by_id));
                            spec.artifact_kind = port.spec.kind_id.clone();
                            spec
                        })
                        .collect(),
                },
                ..Default::default()
            }
        })
        .collect();
    let edges = projection
        .graph
        .edges
        .iter()
        .map(|edge| DagFixtureEdge { id: edge.id.clone(), source: format!("{}@{}", edge.source_node_id, edge.source_port_id), target: format!("{}@{}", edge.target_node_id, edge.target_port_id), ..Default::default() })
        .collect();
    DagFixture { schema: "dag.fixture".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes, edges }
}

pub fn compiled_dag_wire_literal(projection: &WorkflowDocument) -> String {
    let fixture = workflow_to_dag_fixture(projection);
    dag_fixture_to_wire_literal(&fixture)
}
//#endregion 🔖️CompiledDag

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use space_shared::demo_space_projection;

    #[test]
    fn patch_parameter_op_updates_numeric_value() {
        let projection = demo_space_projection();
        let operation = patch_parameter_operation(&projection, "param-brush-size", &json!({ "value": 48.0 })).expect("operation");
        match operation {
            WorkflowOperation::PatchParameter { parameter, .. } => match parameter {
                WorkflowParameter::Numeric { value, .. } => assert_eq!(value, 48.0),
                _ => panic!("expected numeric"),
            },
            _ => panic!("expected patch parameter operation"),
        }
    }

    #[test]
    fn compiled_dag_wire_literal_mentions_app_instances() {
        let wire = compiled_dag_wire_literal(&demo_space_projection());
        assert!(wire.contains("appInstance") || wire.contains("draw"));
    }

    #[test]
    fn space_config_default_matches_the_expected_sticky_defaults() {
        let config = SpaceConfig::default();
        assert_eq!(config.active_panel_tab, space::S_PLAY_CATALOGUE_TAB_ID);
        assert_eq!(config.locale, "en-US");
        assert!(config.camera.is_empty());
        assert!(config.selected_node_ids.is_empty());
    }
}
//#endregion 🧪️Tests
