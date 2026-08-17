//! ⚙️ S Studio app — headless compute over the kernel-owned `WorkflowSnapshot` (constitutional:
//! engine, kept app-level since this app owns no document-side `🗿️artifacts` node — see
//! `🦀️component.rs`'s module doc for the full rationale). Every function here computes over
//! `semio_framework_os::{WorkflowSnapshot, WorkflowMutation}` (a foreign type owned by the kernel
//! `workflow` crate via os-core's re-export) — building `WorkflowMutation` values from arguments is
//! still pure compute, not an `apply_X_mutation` match on a locally-owned enum.

use infinite_board_port_directed_dag::{dag_fixture_to_wire_literal, DagCamera, DagFixture, DagFixtureEdge, DagNodeKind, DagNodeSpec, IoPortSpec};
use semio_framework_os::{
    create_default_workflow_parameter, create_os_id, media_port_spec_id, negotiate_media_contract, os_app_registration, patch_workflow_parameter, register_app_io, resolve_os_app_definition, workflow_node_for_app, workflow_parameter_id_from_port_id, AppDefinition, MediaContract,
    WorkflowSnapshot, WorkflowMutation, WorkflowParameter, WorkflowParameterBinding, WorkflowParameterType, WorkflowPosition,
};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

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
pub fn add_parameter_operation(parameter_type: &WorkflowParameterType, name: &str) -> WorkflowMutation {
    WorkflowMutation::AddParameter { parameter: create_default_workflow_parameter(parameter_type, name, None) }
}

/// @emoji 🩹️ Builds a `PatchParameter` operation by folding `patch` (a `{field: value}` object) into the
/// current parameter — the store-free operation-builder used in place of os-core's
/// `OsWorkflowStore::patch_parameter`.
pub fn patch_parameter_operation(projection: &WorkflowSnapshot, parameter_id: &str, patch: &Value) -> Option<WorkflowMutation> {
    let current = projection.parameters.iter().find(|parameter| parameter_entity_id(parameter) == parameter_id)?;
    Some(WorkflowMutation::PatchParameter { parameter_id: parameter_id.into(), parameter: patch_workflow_parameter(current, patch) })
}
//#endregion 🔖️Parameters

//#region 🔖️WorkflowNodes
/// @emoji ✨️ Builds the `AddNode` operation (minting a fresh node — id, ports, document/config refs,
/// everything — via `workflow_node_for_app`, so replay never re-derives it) plus the new node's id for
/// the caller to focus. The store-free operation-builder the plugin uses in place of os-core's
/// `OsWorkflowStore::add_workflow_node`. A node IS the app instance now, there is no separate
/// `OsAppInstance` to mint alongside it.
pub fn add_workflow_node_operation(plugin_id: &str, app_id: &str, label: Option<&str>, x: f64, y: f64) -> Option<(WorkflowMutation, String)> {
    let app = resolve_os_app_definition(plugin_id, app_id)?;
    let node_id = create_os_id("node");
    let position = WorkflowPosition { x, y, width: 0.0, height: 0.0 };
    let mut node = workflow_node_for_app(&app, plugin_id, &node_id, &position);
    if let Some(label) = label {
        node.label = label.into();
    }
    Some((WorkflowMutation::AddNode { node }, node_id))
}
//#endregion 🔖️WorkflowNodes

//#region 🔖️MediaContractConnect
/// @emoji 🤝️ Resolves the source/target `WorkflowMediaPort`s for a proposed connect from the live
/// projection and negotiates their wire contract — shared by both connect entry points
/// (`connections::ConnectMediaPorts` and the `graph_edit::NodeGraphEdit`/`"connect"` fixture edit) so
/// neither can push a `WorkflowMutation::ConnectPorts` for an incompatible or unresolved pair of ports.
/// Operates directly on `WorkflowNode`/`WorkflowMediaPort` — a node's ports are typed `MediaPortSpec`s
/// now, no more string `artifact_kind` join through a separate `OsMediaPort`.
pub fn negotiate_media_connect(projection: &WorkflowSnapshot, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> Result<MediaContract, String> {
    let source_port =
        projection.graph.nodes.iter().find(|node| node.id == source_node_id).and_then(|node| node.outputs.iter().find(|port| port.id == source_port_id)).ok_or_else(|| format!("unknown source port {source_node_id}:{source_port_id}"))?;
    let target_port =
        projection.graph.nodes.iter().find(|node| node.id == target_node_id).and_then(|node| node.inputs.iter().find(|port| port.id == target_port_id)).ok_or_else(|| format!("unknown target port {target_node_id}:{target_port_id}"))?;
    negotiate_media_contract(source_port, target_port)
}
//#endregion 🔖️MediaContractConnect

//#region 🔖️MediaVfs
// 🚧️ `list_os_workflow_vfs_children`/`OsWorkflowVfsNodeRecord` were deleted with os-core's
// `🔖️WorkflowVfs` region — a full collection-browser UI replaces the workflow-instance VFS in a later
// wave. `flatten_media_vfs_rows` is kept as a minimal compiling stub (always empty) so this crate's
// callers still compile; `media_port_label` still has a real body since it doesn't depend on the
// deleted types.
pub fn flatten_media_vfs_rows(_parent_id: &str, _graph: &semio_framework_os::Workflow, _bindings: &[WorkflowParameterBinding], _parameters: &[WorkflowParameter], _rows: &mut Vec<Value>) {}

pub fn media_port_label(port_id: &str, parameter_by_id: &HashMap<String, &WorkflowParameter>) -> String {
    workflow_parameter_id_from_port_id(port_id).and_then(|id| parameter_by_id.get(&id).map(|row| parameter_name(row).to_string())).or_else(|| media_port_spec_id(port_id)).unwrap_or_else(|| port_id.to_string())
}
//#endregion 🔖️MediaVfs

//#region 🔖️CompiledDag
/// @emoji 🕸️ Projects the workflow onto the generic port-directed-DAG fixture the Compiled DAG window
/// renders — every `WorkflowNode` becomes one `DagNodeKind::AppInstance` directly (node IS instance
/// now; no separate join through `OsAppInstance`).
pub fn workflow_to_dag_fixture(projection: &WorkflowSnapshot) -> DagFixture {
    let parameter_by_id: HashMap<_, _> = projection.parameters.iter().map(|row| (parameter_entity_id(row).to_string(), row)).collect();
    let nodes = projection
        .graph
        .nodes
        .iter()
        .map(|node| {
            let registration = os_app_registration(&node.plugin_id, &node.app_id);
            let icon = format!("emoji:{}", registration.map_or_else(|| "s".into(), |row| row.component_kind));
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

pub fn compiled_dag_wire_literal(projection: &WorkflowSnapshot) -> String {
    let fixture = workflow_to_dag_fixture(projection);
    dag_fixture_to_wire_literal(&fixture)
}
//#endregion 🔖️CompiledDag

//#region 🔖️OsParameterBridge
// 🌉️ os-core's `instance::OsParameter`/`OsParameterFieldBinding`/`OsParameterType` (registry-facing)
// were deliberately left untouched by the os-core dissolve while `WorkflowSnapshot.parameters`/
// `.parameter_bindings` now carry the kernel `workflow` crate's own, structurally-identical
// `WorkflowParameter`/`WorkflowParameterBinding`/`WorkflowParameterType`. These two parameter
// vocabularies are a known, intentionally-deferred duplication — this bridge converts between them at
// the few call sites that still need the os-core registry-facing shape.
pub fn workflow_parameter_type_to_os(kind: &WorkflowParameterType) -> semio_framework_os::OsParameterType {
    match kind {
        WorkflowParameterType::Numeric => semio_framework_os::OsParameterType::Numeric,
        WorkflowParameterType::Categorical => semio_framework_os::OsParameterType::Categorical,
        WorkflowParameterType::Toggle => semio_framework_os::OsParameterType::Toggle,
        WorkflowParameterType::Text => semio_framework_os::OsParameterType::Text,
    }
}

pub fn workflow_parameter_to_os(parameter: &WorkflowParameter) -> semio_framework_os::OsParameter {
    match parameter {
        WorkflowParameter::Numeric { id, name, value, min, max, step } => semio_framework_os::OsParameter::Numeric { id: id.clone(), name: name.clone(), value: *value, min: *min, max: *max, step: *step },
        WorkflowParameter::Categorical { id, name, value, options } => semio_framework_os::OsParameter::Categorical { id: id.clone(), name: name.clone(), value: value.clone(), options: options.clone() },
        WorkflowParameter::Toggle { id, name, value } => semio_framework_os::OsParameter::Toggle { id: id.clone(), name: name.clone(), value: *value },
        WorkflowParameter::Text { id, name, value } => semio_framework_os::OsParameter::Text { id: id.clone(), name: name.clone(), value: value.clone() },
    }
}

pub fn workflow_parameters_to_os(parameters: &[WorkflowParameter]) -> Vec<semio_framework_os::OsParameter> {
    parameters.iter().map(workflow_parameter_to_os).collect()
}

pub fn workflow_parameter_binding_to_os(binding: &WorkflowParameterBinding) -> semio_framework_os::OsParameterFieldBinding {
    semio_framework_os::OsParameterFieldBinding { parameter_id: binding.parameter_id.clone(), node_id: binding.node_id.clone(), field_path: binding.field_path.clone() }
}

pub fn workflow_parameter_bindings_to_os(bindings: &[WorkflowParameterBinding]) -> Vec<semio_framework_os::OsParameterFieldBinding> {
    bindings.iter().map(workflow_parameter_binding_to_os).collect()
}

/// 🌉️ Whether `parameter`'s type is compatible with a registered app parameter-field's declared
/// `OsParameterType` — the inspector panel's single call site for this bridge.
pub fn os_parameter_types_compatible_shim(parameter: &WorkflowParameter, target: &semio_framework_os::OsParameterType) -> bool {
    let kind = match parameter {
        WorkflowParameter::Numeric { .. } => WorkflowParameterType::Numeric,
        WorkflowParameter::Categorical { .. } => WorkflowParameterType::Categorical,
        WorkflowParameter::Toggle { .. } => WorkflowParameterType::Toggle,
        WorkflowParameter::Text { .. } => WorkflowParameterType::Text,
    };
    semio_framework_os::os_parameter_types_compatible(&workflow_parameter_type_to_os(&kind), target)
}
//#endregion 🔖️OsParameterBridge

//#region 🔖️AppRegistrations
/// 🪐️ One `appRegistrationsJson` entry — the wire shape `os-shell.tsx`'s `SetAppRegistrations` push
/// builds from `loadedPlugins.flatMap(entry => entry.manifest.apps.map(app => ({pluginId, app})))`.
/// `app` deserializes straight off `AppDefinition`'s own `Deserialize` impl since it's the literal
/// manifest-JSON `AppDefinition` object, unmodified across the wasm boundary.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppRegistrationWireEntry {
    plugin_id: String,
    app: AppDefinition,
}

/// 🪐️ Registers every `{pluginId, app}` entry `json` carries into this wasm instance's OWN
/// `semio_framework_os::APP_REGISTRATIONS` copy — the space app is its own wasm component, so its
/// statically-linked copy of os-core's `APP_REGISTRATIONS` never sees what native/test hosts populate
/// via `PluginHost::load_plugin`/`hot_swap_plugin`; this is how it gets populated in a real
/// browser/wasm host instead. Malformed/empty `json` degrades to a silent no-op — this is a
/// best-effort host hint push, not a user-facing operation with error surfacing. Lives here (the
/// app-level compute engine, per this app's no-`🗿️artifacts`-node rationale in `🦀️component.rs`'s
/// module doc) rather than in the `🧭️navigation` command file, so the command handler stays
/// dispatch-only and the one production `register_app_io` call for this app sits beside its other
/// OS-registry bridge functions.
pub fn apply_app_registrations(json: &str) {
    let Ok(entries) = serde_json::from_str::<Vec<AppRegistrationWireEntry>>(json) else { return };
    for entry in entries {
        register_app_io(&entry.plugin_id, &entry.app);
    }
}
//#endregion 🔖️AppRegistrations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo_space_projection;
    use serde_json::json;

    #[test]
    fn patch_parameter_op_updates_numeric_value() {
        let projection = demo_space_projection();
        let operation = patch_parameter_operation(&projection, "param-brush-size", &json!({ "value": 48.0 })).expect("operation");
        match operation {
            WorkflowMutation::PatchParameter { parameter, .. } => match parameter {
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
}
//#endregion 🧪️Tests
