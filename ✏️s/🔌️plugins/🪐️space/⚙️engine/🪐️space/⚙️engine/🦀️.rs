//! ⚙️ S Studio app — headless compute over the kernel-owned `WorkflowSnapshot` (constitutional:
//! engine, kept app-level since this app owns no document-side `🗿️artifacts` node — see
//! `🦀️.rs`'s module doc for the full rationale). Every function here computes over
//! `semio_framework_os::{WorkflowSnapshot, WorkflowMutation}` (a foreign type owned by the kernel
//! `workflow` crate via os-core's re-export) — building `WorkflowMutation` values from arguments is
//! still pure compute, not an `apply_X_mutation` match on a locally-owned enum.

use infinite_board_port_directed_dag::{dag_fixture_to_wire_literal, DagCamera, DagFixture, DagFixtureEdge, DagNodeKind, DagNodeSpec, IoPortSpec};
use pack::json::Value;
use semio_framework_os::workflow::{AddNode, AddParameter, ChangeParameter};
use semio_framework_os::{
    create_default_workflow_parameter, create_os_id, media_port_spec_id, negotiate_media_contract, os_app_registration, resolve_os_app_definition, workflow_node_for_app, workflow_parameter_id, workflow_parameter_id_from_port_id, workflow_parameter_name,
    MediaContract, WorkflowMutation, WorkflowParameter, WorkflowParameterBinding, WorkflowParameterType, WorkflowPosition, WorkflowSnapshot,
};
use std::collections::HashMap;

//#region 🔖️SyncBridge
/// 🌉️ `app_commands!`'s generated `dispatch(doc, cfg)` (framework-fixed, `🧰️framework/🛍️products/
/// 💻️os/🔨️modules/🔌️plugin/🦀️.rs`) calls every command's `handle` SYNCHRONOUSLY — matches the same
/// `pub(crate) fn resolve_kernel_future` bridge `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs` uses
/// internally (not `pub`, so unreachable from a plugin crate — this is the plugin-local twin of the
/// same technique). Every `async fn` in this whole wasm component is a run-to-completion frame
/// transaction that never truly suspends (no real I/O inside an `s` command), so polling once is
/// always sound here; a genuine `Pending` would mean a real bug upstream, hence the panic.
pub(crate) fn resolve_future<F: std::future::Future>(future: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut cx = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("resolve_future: this call site is documented immediate"),
    }
}
//#endregion 🔖️SyncBridge

//#region 🔖️Parameters
pub async fn parameter_entity_id(parameter: &WorkflowParameter) -> &str {
    match parameter {
        WorkflowParameter::Numeric { id, .. } | WorkflowParameter::Categorical { id, .. } | WorkflowParameter::Toggle { id, .. } | WorkflowParameter::Text { id, .. } => id,
    }
}

pub async fn parameter_name(parameter: &WorkflowParameter) -> &str {
    match parameter {
        WorkflowParameter::Numeric { name, .. } | WorkflowParameter::Categorical { name, .. } | WorkflowParameter::Toggle { name, .. } | WorkflowParameter::Text { name, .. } => name,
    }
}

pub trait OsParameterId {
    async fn id(&self) -> &str;
}

impl OsParameterId for WorkflowParameter {
    async fn id(&self) -> &str {
        parameter_entity_id(self).await
    }
}

/// @emoji ➕️ Builds an `AddParameter` operation with a fresh default parameter of the requested type.
pub async fn add_parameter_operation(parameter_type: &WorkflowParameterType, name: &str) -> WorkflowMutation {
    WorkflowMutation::AddParameter(AddParameter { parameter: Box::new(create_default_workflow_parameter(parameter_type, name, None).await) })
}

/// 🚧️ Local reimplementation of `semio_framework_os::patch_workflow_parameter` (ported verbatim,
/// `serde_json::Value` swapped for `pack::json::Value`): FRAMEWORK GAP, report not fixed here — the
/// real `patch_workflow_parameter` still takes `&serde_json::Value`, and this crate no longer depends
/// on `serde_json` in production (only as a dev-dependency test oracle), so the framework function is
/// literally uncallable from here until it migrates to `ToValue`/`FromValue` (or `pack::json::Value`).
async fn local_patch_workflow_parameter(parameter: &WorkflowParameter, patch: &Value) -> WorkflowParameter {
    let name = patch.get("name").and_then(Value::as_str).map_or_else(|| workflow_parameter_name(parameter), str::to_string);
    let patch_type = patch.get("type").and_then(Value::as_str);
    let use_numeric = patch_type == Some("numeric") || (patch_type.is_none() && matches!(parameter, WorkflowParameter::Numeric { .. }));
    if use_numeric {
        let current = match parameter {
            WorkflowParameter::Numeric { .. } => parameter.clone(),
            _ => create_default_workflow_parameter(&WorkflowParameterType::Numeric, &name, Some(workflow_parameter_id(parameter))).await,
        };
        if let WorkflowParameter::Numeric { id, min: current_min, max: current_max, step: current_step, value: current_value, .. } = current {
            let min = patch.get("min").and_then(Value::as_f64).or(current_min);
            let max = patch.get("max").and_then(Value::as_f64).or(current_max);
            let step = patch.get("step").and_then(Value::as_f64).or(current_step);
            let raw_value = patch.get("value").and_then(Value::as_f64).unwrap_or(current_value);
            return WorkflowParameter::Numeric { id, name, min, max, step, value: local_clamp_numeric_value(raw_value, min, max, step) };
        }
    }
    let use_categorical = patch_type == Some("categorical") || (patch_type.is_none() && matches!(parameter, WorkflowParameter::Categorical { .. }));
    if use_categorical {
        let current = match parameter {
            WorkflowParameter::Categorical { .. } => parameter.clone(),
            _ => create_default_workflow_parameter(&WorkflowParameterType::Categorical, &name, Some(workflow_parameter_id(parameter))).await,
        };
        if let WorkflowParameter::Categorical { id, value: current_value, options: current_options, .. } = current {
            let options = patch.get("options").and_then(Value::as_array).map_or(current_options, |entries| entries.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect::<Vec<_>>());
            let unique_options = if options.is_empty() { vec!["Option A".into()] } else { options };
            let value = patch
                .get("value")
                .and_then(Value::as_str)
                .filter(|v| unique_options.iter().any(|option| option == v))
                .map(str::to_string)
                .or_else(|| unique_options.iter().find(|option| **option == current_value).cloned())
                .unwrap_or_else(|| unique_options[0].clone());
            return WorkflowParameter::Categorical { id, name, options: unique_options, value };
        }
    }
    if patch_type == Some("toggle") || (patch_type.is_none() && matches!(parameter, WorkflowParameter::Toggle { .. })) {
        let current = match parameter {
            WorkflowParameter::Toggle { .. } => parameter.clone(),
            _ => create_default_workflow_parameter(&WorkflowParameterType::Toggle, &name, Some(workflow_parameter_id(parameter))).await,
        };
        if let WorkflowParameter::Toggle { id, value: current_value, .. } = current {
            let value = patch.get("value").and_then(Value::as_bool).unwrap_or(current_value);
            return WorkflowParameter::Toggle { id, name, value };
        }
    }
    let current = match parameter {
        WorkflowParameter::Text { .. } => parameter.clone(),
        _ => create_default_workflow_parameter(&WorkflowParameterType::Text, &name, Some(workflow_parameter_id(parameter))).await,
    };
    if let WorkflowParameter::Text { id, value: current_value, .. } = current {
        let value = patch.get("value").and_then(Value::as_str).map_or(current_value, str::to_string);
        return WorkflowParameter::Text { id, name, value };
    }
    parameter.clone()
}

fn local_clamp_numeric_value(value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64>) -> f64 {
    let mut next = value;
    if let Some(min) = min.filter(|v| v.is_finite()) {
        next = next.max(min);
    }
    if let Some(max) = max.filter(|v| v.is_finite()) {
        next = next.min(max);
    }
    if let Some(step) = step.filter(|v| v.is_finite() && *v > 0.0) {
        let anchor = min.filter(|v| v.is_finite()).unwrap_or(0.0);
        next = anchor + ((next - anchor) / step).round() * step;
        if let Some(min) = min.filter(|v| v.is_finite()) {
            next = next.max(min);
        }
        if let Some(max) = max.filter(|v| v.is_finite()) {
            next = next.min(max);
        }
    }
    next
}

/// @emoji 🩹️ Builds a `ChangeParameter` operation by folding `patch` (a `{field: value}` object) into
/// the current parameter — the store-free operation-builder used in place of os-core's
/// `OsWorkflowStore::patch_parameter`.
pub async fn patch_parameter_operation(projection: &WorkflowSnapshot, parameter_id: &str, patch: &Value) -> Option<WorkflowMutation> {
    let mut current = None;
    for parameter in &projection.parameters {
        if parameter_entity_id(parameter).await == parameter_id {
            current = Some(parameter);
            break;
        }
    }
    let current = current?;
    Some(WorkflowMutation::ChangeParameter(ChangeParameter { parameter_id: parameter_id.into(), parameter: Box::new(local_patch_workflow_parameter(current, patch).await) }))
}
//#endregion 🔖️Parameters

//#region 🔖️WorkflowNodes
/// @emoji ✨️ Builds the `AddNode` operation (minting a fresh node — id, ports, document/config refs,
/// everything — via `workflow_node_for_app`, so replay never re-derives it) plus the new node's id for
/// the caller to focus. The store-free operation-builder the plugin uses in place of os-core's
/// `OsWorkflowStore::add_workflow_node`. A node IS the app instance now, there is no separate
/// `OsAppInstance` to mint alongside it.
pub async fn add_workflow_node_operation(plugin_id: &str, app_id: &str, label: Option<&str>, x: f64, y: f64) -> Option<(WorkflowMutation, String)> {
    let app = resolve_os_app_definition(plugin_id, app_id)?;
    let node_id = create_os_id("node");
    let position = WorkflowPosition { x, y, width: 0.0, height: 0.0 };
    let mut node = workflow_node_for_app(&app, plugin_id, &node_id, &position).await;
    if let Some(label) = label {
        node.label = label.into();
    }
    Some((WorkflowMutation::AddNode(AddNode { node }), node_id))
}
//#endregion 🔖️WorkflowNodes

//#region 🔖️MediaContractConnect
/// @emoji 🤝️ Resolves the source/target `WorkflowMediaPort`s for a proposed connect from the live
/// projection and negotiates their wire contract — shared by both connect entry points
/// (`connections::ConnectMediaPorts` and the `graph_edit::NodeGraphEdit`/`"connect"` fixture edit) so
/// neither can push a `WorkflowMutation::ConnectPorts` for an incompatible or unresolved pair of ports.
/// Operates directly on `WorkflowNode`/`WorkflowMediaPort` — a node's ports are typed `MediaPortSpec`s
/// now, no more string `artifact_kind` join through a separate `OsMediaPort`.
pub async fn negotiate_media_connect(projection: &WorkflowSnapshot, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> Result<MediaContract, String> {
    let source_port = projection.graph.nodes.iter().find(|node| node.id == source_node_id).and_then(|node| node.outputs.iter().find(|port| port.id == source_port_id)).ok_or_else(|| format!("unknown source port {source_node_id}:{source_port_id}"))?;
    let target_port = projection.graph.nodes.iter().find(|node| node.id == target_node_id).and_then(|node| node.inputs.iter().find(|port| port.id == target_port_id)).ok_or_else(|| format!("unknown target port {target_node_id}:{target_port_id}"))?;
    negotiate_media_contract(source_port, target_port)
}
//#endregion 🔖️MediaContractConnect

//#region 🔖️MediaVfs
// 🚧️ `list_os_workflow_vfs_children`/`OsWorkflowVfsNodeRecord` were deleted with os-core's
// `🔖️WorkflowVfs` region — a full collection-browser UI replaces the workflow-instance VFS in a later
// wave. `flatten_media_vfs_rows` is kept as a minimal compiling stub (always empty) so this crate's
// callers still compile; `media_port_label` still has a real body since it doesn't depend on the
// deleted types.
pub async fn flatten_media_vfs_rows(_parent_id: &str, _graph: &semio_framework_os::Workflow, _bindings: &[WorkflowParameterBinding], _parameters: &[WorkflowParameter], _rows: &mut Vec<Value>) {}

pub async fn media_port_label(port_id: &str, parameter_by_id: &HashMap<String, &WorkflowParameter>) -> String {
    if let Some(id) = workflow_parameter_id_from_port_id(port_id) {
        if let Some(row) = parameter_by_id.get(&id) {
            return parameter_name(row).await.to_string();
        }
    }
    media_port_spec_id(port_id).unwrap_or_else(|| port_id.to_string())
}
//#endregion 🔖️MediaVfs

//#region 🔖️CompiledDag
/// @emoji 🕸️ Projects the workflow onto the generic port-directed-DAG fixture the Compiled DAG window
/// renders — every `WorkflowNode` becomes one `DagNodeKind::AppInstance` directly (node IS instance
/// now; no separate join through `OsAppInstance`).
pub async fn workflow_to_dag_fixture(projection: &WorkflowSnapshot) -> DagFixture {
    let mut parameter_by_id: HashMap<String, &WorkflowParameter> = HashMap::new();
    for row in &projection.parameters {
        parameter_by_id.insert(parameter_entity_id(row).await.to_string(), row);
    }
    let mut nodes = Vec::with_capacity(projection.graph.nodes.len());
    for node in &projection.graph.nodes {
        let registration = os_app_registration(&node.plugin_id, &node.app_id);
        let icon = format!("emoji:{}", registration.map_or_else(|| "s".into(), |row| row.component_kind));
        let mut inputs = Vec::with_capacity(node.inputs.len());
        for port in &node.inputs {
            let mut spec = IoPortSpec::simple(&port.id, media_port_label(&port.id, &parameter_by_id).await);
            spec.artifact_kind = port.spec.kind_id.clone();
            inputs.push(spec);
        }
        let mut outputs = Vec::with_capacity(node.outputs.len());
        for port in &node.outputs {
            let mut spec = IoPortSpec::simple(&port.id, media_port_label(&port.id, &parameter_by_id).await);
            spec.artifact_kind = port.spec.kind_id.clone();
            outputs.push(spec);
        }
        nodes.push(DagNodeSpec {
            id: node.id.clone(),
            name: node.label.clone(),
            abbreviation: if node.app_id.chars().count() <= 3 { node.app_id.clone() } else { node.app_id.chars().take(3).collect() },
            icon: icon.clone(),
            x: node.x + node.width / 2.0,
            y: node.y + node.height / 2.0,
            width: node.width,
            height: node.height,
            operator_kind: Some(node.plugin_id.clone()),
            kind: DagNodeKind::AppInstance { instance_id: node.id.clone(), plugin_id: node.plugin_id.clone(), app_id: node.app_id.clone(), icon, inputs, outputs },
            ..Default::default()
        });
    }
    let edges = projection
        .graph
        .edges
        .iter()
        .map(|edge| DagFixtureEdge { id: edge.id.clone(), source: format!("{}@{}", edge.source_node_id, edge.source_port_id), target: format!("{}@{}", edge.target_node_id, edge.target_port_id), ..Default::default() })
        .collect();
    DagFixture { schema: "dag.fixture".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes, edges }
}

pub async fn compiled_dag_wire_literal(projection: &WorkflowSnapshot) -> String {
    let fixture = workflow_to_dag_fixture(projection).await;
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
pub async fn workflow_parameter_type_to_os(kind: &WorkflowParameterType) -> semio_framework_os::OsParameterType {
    match kind {
        WorkflowParameterType::Numeric => semio_framework_os::OsParameterType::Numeric,
        WorkflowParameterType::Categorical => semio_framework_os::OsParameterType::Categorical,
        WorkflowParameterType::Toggle => semio_framework_os::OsParameterType::Toggle,
        WorkflowParameterType::Text => semio_framework_os::OsParameterType::Text,
    }
}

pub async fn workflow_parameter_to_os(parameter: &WorkflowParameter) -> semio_framework_os::OsParameter {
    match parameter {
        WorkflowParameter::Numeric { id, name, value, min, max, step } => semio_framework_os::OsParameter::Numeric { id: id.clone(), name: name.clone(), value: *value, min: *min, max: *max, step: *step },
        WorkflowParameter::Categorical { id, name, value, options } => semio_framework_os::OsParameter::Categorical { id: id.clone(), name: name.clone(), value: value.clone(), options: options.clone() },
        WorkflowParameter::Toggle { id, name, value } => semio_framework_os::OsParameter::Toggle { id: id.clone(), name: name.clone(), value: *value },
        WorkflowParameter::Text { id, name, value } => semio_framework_os::OsParameter::Text { id: id.clone(), name: name.clone(), value: value.clone() },
    }
}

pub async fn workflow_parameters_to_os(parameters: &[WorkflowParameter]) -> Vec<semio_framework_os::OsParameter> {
    let mut out = Vec::with_capacity(parameters.len());
    for parameter in parameters {
        out.push(workflow_parameter_to_os(parameter).await);
    }
    out
}

pub async fn workflow_parameter_binding_to_os(binding: &WorkflowParameterBinding) -> semio_framework_os::OsParameterFieldBinding {
    semio_framework_os::OsParameterFieldBinding { parameter_id: binding.parameter_id.clone(), node_id: binding.node_id.clone(), field_path: binding.field_path.clone() }
}

pub async fn workflow_parameter_bindings_to_os(bindings: &[WorkflowParameterBinding]) -> Vec<semio_framework_os::OsParameterFieldBinding> {
    let mut out = Vec::with_capacity(bindings.len());
    for binding in bindings {
        out.push(workflow_parameter_binding_to_os(binding).await);
    }
    out
}

/// 🌉️ Whether `parameter`'s type is compatible with a registered app parameter-field's declared
/// `OsParameterType` — the inspector panel's single call site for this bridge.
pub async fn os_parameter_types_compatible_shim(parameter: &WorkflowParameter, target: &semio_framework_os::OsParameterType) -> bool {
    let kind = match parameter {
        WorkflowParameter::Numeric { .. } => WorkflowParameterType::Numeric,
        WorkflowParameter::Categorical { .. } => WorkflowParameterType::Categorical,
        WorkflowParameter::Toggle { .. } => WorkflowParameterType::Toggle,
        WorkflowParameter::Text { .. } => WorkflowParameterType::Text,
    };
    semio_framework_os::os_parameter_types_compatible(&workflow_parameter_type_to_os(&kind).await, target)
}
//#endregion 🔖️OsParameterBridge

//#region 🔖️AppRegistrations
/// 🚧️ FRAMEWORK GAP — report, not fixed here: `AppDefinition` (`Modes`/`WindowKinds` carry a
/// hand-rolled `serde(try_from/into = "Vec<T>")` wire shape) still only derives `serde::Deserialize`,
/// never `dsl::FromValue`, and `dsl::from_dsl_value` no longer has a `DeserializeOwned` path (it is
/// `<T: FromValue>` only now) — there is no way left to decode a wire-JSON `AppDefinition` from a
/// crate that no longer depends on `serde_json` in production. Degrades to a no-op per this
/// function's own pre-existing tolerance ("best-effort host hint push, not a user-facing operation
/// with error surfacing") until `AppDefinition: FromValue` lands upstream — see
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
pub async fn apply_app_registrations(_json: &str) {}
//#endregion 🔖️AppRegistrations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo_space_projection;

    #[semio_framework_async_macros::async_test]
    async fn patch_parameter_op_updates_numeric_value() {
        let projection = demo_space_projection().await;
        let patch = pack::json::object([("value".to_string(), Value::from(48.0))]);
        let operation = patch_parameter_operation(&projection, "param-brush-size", &patch).await.expect("operation");
        match operation {
            WorkflowMutation::ChangeParameter(ChangeParameter { parameter, .. }) => match *parameter {
                WorkflowParameter::Numeric { value, .. } => assert_eq!(value, 48.0),
                _ => panic!("expected numeric"),
            },
            _ => panic!("expected change parameter operation"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn compiled_dag_wire_literal_mentions_app_instances() {
        let wire = compiled_dag_wire_literal(&demo_space_projection().await).await;
        assert!(wire.contains("appInstance") || wire.contains("draw"));
    }
}
//#endregion 🧪️Tests
