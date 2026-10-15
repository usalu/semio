//! 🎬️ Workflow, VFS snapshot types, and media export registry.
// #region workflow

// 🧬️ Kernel re-exports — the persisted graph model itself (`Workflow`/`WorkflowNode`/`WorkflowEdge`/
// `WorkflowMediaPort`/`WorkflowPosition`/`MediaContract`/`WorkflowDelivery`/
// `plan_workflow`/`workflow_node_for_app`/`placeholder_media_contract`/`empty_workflow`) lives in
// the `semio-framework-os-kernel-workflow` crate (dependency name `workflow`) — see its
// `🔖️InstanceIdentity` doc. Re-exported here so every existing `crate::workflow::X` call site in
// this file keeps working unchanged. `workflow::validate_workflow` (dangling-edge + cycle checks
// only) is re-exported under a different name because this module's own `validate_workflow` (below)
// wraps it with the contract-renegotiation check that still needs the artifact registry, which only
// exists at this layer.
// 🧬️ `WorkflowSnapshot`/`WorkflowMutation`/`WorkflowParameter*`/`WorkflowInput*`/`WorkflowOutputBinding`
// absorb os-core's dissolved `OsSnapshot`/`OsMutation`/`instance::OsParameter*` (see `## The
// inversion` in the plan) — re-exported here too so every `crate::workflow::X` call site (and every
// downstream crate importing via `semio_framework_os::workflow::X`/`semio_framework_os::X`) keeps a
// single source of truth for the workflow document vocabulary.
pub use crate::workflow_kernel::{
    apply_workflow_operation, create_default_workflow_parameter, empty_workflow, empty_workflow_snapshot, media_port_spec_id, patch_workflow_parameter, placeholder_media_contract, plan_workflow, sync_workflow_parameter_ports,
    validate_workflow as kernel_validate_workflow, validate_workflow_parameter_config_binding, validate_workflow_snapshot, workflow_node_for_app, workflow_parameter_id, workflow_parameter_id_from_port_id, workflow_parameter_name,
    workflow_parameter_types_compatible, workflow_parameter_value, MediaContract, Workflow, WorkflowDelivery, WorkflowEdge, WorkflowInput, WorkflowInputBinding, WorkflowMediaPort, WorkflowMutation, WorkflowNode,
    WorkflowOutputBinding, WorkflowParameter, WorkflowParameterBinding, WorkflowParameterPatch, WorkflowParameterType, WorkflowPosition, WorkflowSnapshot, WorkflowValidation, AddInput, AddNode, AddParameter, BindInput,
    BindOutput, BindParameterField, ChangeParameter, ConnectPorts, DisconnectEdge, MoveNode, RemoveInput, RemoveNode, RemoveParameter, RenameNode, UnbindInput, UnbindOutput, UnbindParameterField, UpdateNodePorts,
    S_WORKFLOW_SCHEMA, WORKFLOW_SCHEMA,
};

use crate::instance::create_os_id;
use crate::registry::{os_app_registration, os_artifact_descriptor, OsArtifactDescriptor};
use {semio_framework::media_types_compatible,semio_framework_artifact_reference::ArtifactDialect,semio_framework::MediaCompat,semio_framework::MediaWireFormat};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};

pub const OS_SPACE_SCHEMA: &str = "os.space";
pub const OS_WORKFLOW_VFS_ROOT_ID: &str = "os-workflow-root";
pub const OS_MEDIA_FLOW_MODULE_ID: &str = "os-media";

//#region 🔖️Workflow
/// 🤝️ Negotiates the wire contract for connecting `source_port` (a producer/output) to
/// `target_port` (a consumer/input). Compatibility itself is decided from the ports' own
/// `MediaPortSpec.media_type` (always accurate, even for the implicit `document:*` ports whose
/// `kind_id` is `None`); the artifact-kind registry is only consulted for wire-encoding details
/// (schema / export / import formats), keyed by `kind_id` when present. `Err` means the connect
/// must be rejected outright (see the `s::plugin` connect handlers).
pub fn negotiate_media_contract(source_port: &WorkflowMediaPort, target_port: &WorkflowMediaPort) -> Result<MediaContract, String> {
    let source_descriptor = os_artifact_descriptor(source_port.spec.kind_id.as_deref().unwrap_or_default());
    let target_descriptor = os_artifact_descriptor(target_port.spec.kind_id.as_deref().unwrap_or_default());
    let conversion = match crate::host::resolve_kernel_future(media_types_compatible(&source_port.spec.media_type, &target_port.spec.media_type)) {
        MediaCompat::Direct => None,
        MediaCompat::Convert { from, to } => Some((from, to)),
        MediaCompat::Reject => {
            return Err(format!(
                "cannot connect `{}` ({:?}/{:?}) to `{}` ({:?}/{:?}): incompatible media types",
                source_port.spec.id, source_port.spec.media_type.class, source_port.spec.media_type.form, target_port.spec.id, target_port.spec.media_type.class, target_port.spec.media_type.form
            ));
        }
    };
    let wire = negotiate_wire_format(&source_descriptor, &target_descriptor)?.ok_or_else(|| format!("cannot connect `{}` to `{}`: no shared wire format", source_port.spec.id, target_port.spec.id))?;
    let kind_id = target_port.spec.kind_id.clone().unwrap_or_else(|| target_descriptor.kind.clone());
    Ok(MediaContract { kind_id, media_type: target_port.spec.media_type, wire, conversion })
}

/// 🔀️ Prefers a shared `Document{schema}` wire (structured payloads round-trip losslessly) over a shared
/// `Binary{format_kind}` wire (the first common format kind id between the two descriptors' export/import
/// lists) — see `MediaWireFormat`. The legacy format enum was retired — ticket 26/08/11/
/// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6.
fn negotiate_wire_format(source: &OsArtifactDescriptor, target: &OsArtifactDescriptor) -> Result<Option<MediaWireFormat>, String> {
    if !source.schema.is_empty() && source.schema == target.schema {
        return Ok(Some(MediaWireFormat::Intrinsic { schema: source.schema.clone() }));
    }
    if !source.export_stdio_kinds.is_empty() && !target.import_stdio_kinds.is_empty() {
        for kind in &source.export_stdio_kinds {
            let format_kind = semio_framework_os_kernel::io::format_descriptor(kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown source stdio format kind `{kind}`"))?.kind_id;
            for other in &target.import_stdio_kinds {
                let target_format_kind = semio_framework_os_kernel::io::format_descriptor(other).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown target stdio format kind `{other}`"))?.kind_id;
                if target_format_kind == format_kind {
                    return Ok(Some(MediaWireFormat::Binary { format_kind }));
                }
            }
        }
    }
    if let Some(format_kind) = registry_shared_stdio_dialect(&native_dialect_kind(&source.kind), &native_dialect_kind(&target.kind))? {
        return Ok(Some(MediaWireFormat::Binary { format_kind }));
    }
    Ok(source.export_formats.iter().find(|format| target.import_formats.contains(format)).map(|format| MediaWireFormat::Binary { format_kind: format.clone() }))
}

/// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: see the os-side twin of this function for
/// the full rationale — consults the live typed IO registry as a supplement to the
/// `export_stdio_kinds`/`import_stdio_kinds` static lists, catching drift between the two.
fn registry_shared_stdio_dialect(source_kind: &str, target_kind: &str) -> Result<Option<String>, String> {
    use semio_framework_os_kernel::io::IoDirection;
    let target_reads: HashSet<&str> = crate::host::resolve_kernel_future(semio_framework_os_kernel::io::dialects_for(target_kind, IoDirection::Import)).map_err(|error| format!("{} registry unavailable", error.registry))?.iter().map(|d| d.artifact_kind).collect();
    if target_reads.contains(source_kind) {
        let descriptor = semio_framework_os_kernel::io::format_descriptor(source_kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown source dialect format kind `{source_kind}`"))?;
        return Ok(Some(descriptor.kind_id));
    }
    let source_reads: HashSet<&str> = crate::host::resolve_kernel_future(semio_framework_os_kernel::io::dialects_for(source_kind, IoDirection::Import)).map_err(|error| format!("{} registry unavailable", error.registry))?.iter().map(|d| d.artifact_kind).collect();
    if let Some(candidate) = target_reads.intersection(&source_reads).next() {
        let descriptor = semio_framework_os_kernel::io::format_descriptor(candidate).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown shared dialect format kind `{candidate}`"))?;
        return Ok(Some(descriptor.kind_id));
    }
    Ok(None)
}

/// ✅️ Validates workflow connectivity, cycle freedom (via `workflow::validate_workflow`,
/// re-exported as `kernel_validate_workflow`), and edge-contract consistency (this layer's own
/// pass, since it needs the artifact registry the kernel crate doesn't have).
pub fn validate_workflow(graph: &Workflow) -> WorkflowValidation {
    let mut validation = crate::host::resolve_kernel_future(kernel_validate_workflow(graph));

    //#region ContractConsistency
    // 🛡️ Defense in depth for merged/imported studio documents: re-negotiate each edge's endpoints
    // against the *current* artifact registry and flag any edge whose stored `contract` no longer
    // matches — a concurrent re-typing or a stale import can leave a wire's contract behind.
    let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    for edge in &graph.edges {
        let Some(source_port) = node_by_id.get(edge.source_node_id.as_str()).and_then(|node| node.outputs.iter().find(|port| port.id == edge.source_port_id)) else { continue };
        let Some(target_port) = node_by_id.get(edge.target_node_id.as_str()).and_then(|node| node.inputs.iter().find(|port| port.id == edge.target_port_id)) else { continue };
        match negotiate_media_contract(source_port, target_port) {
            Ok(contract) if contract == edge.contract => {}
            Ok(_) => validation.errors.push(format!("edge {} contract stale: no longer matches negotiated port types", edge.id)),
            Err(reason) => validation.errors.push(format!("edge {} contract invalid: {reason}", edge.id)),
        }
    }
    //#endregion ContractConsistency

    validation.ok = validation.errors.is_empty();
    validation
}

pub fn os_media_neuron_kind_for_node(node_id: &str) -> String {
    format!("os.media.node.{node_id}")
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsWorkflowCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for OsWorkflowCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

pub fn os_workflow_to_flow_host_snapshot_json(graph: &Workflow, camera: &OsWorkflowCamera) -> Value {
    let widgets: Vec<_> = graph
        .nodes
        .iter()
        .map(|node| {
            json!({
                "kind": "neuron",
                "id": node.id,
                "neuronKind": os_media_neuron_kind_for_node(&node.id),
                "inputPorts": node.inputs.iter().map(|port| &port.id).collect::<Vec<_>>(),
                "outputPorts": node.outputs.iter().map(|port| &port.id).collect::<Vec<_>>(),
                "params": {
                    "nodeId": node.id,
                    "pluginId": node.plugin_id,
                    "appId": node.app_id,
                },
                "preview": true,
            })
        })
        .collect();
    let layout: HashMap<_, _> = graph.nodes.iter().map(|node| (node.id.clone(), json!({ "x": node.x + node.width / 2.0, "y": node.y + node.height / 2.0 }))).collect();
    let synapses: Vec<_> = graph
        .edges
        .iter()
        .map(|edge| {
            json!({
                "id": edge.id,
                "from": edge.source_node_id,
                "to": edge.target_node_id,
                "fromPort": edge.source_port_id,
                "toPort": edge.target_port_id,
            })
        })
        .collect();
    json!({
        "schema": "flow.host_snapshot",
        "camera": { "x": camera.x, "y": camera.y, "zoom": camera.zoom },
        "widgets": widgets,
        "synapses": synapses,
        "layout": layout,
    })
}

/** 🔁️ Diffs a flow snapshot back into workflow operations — inverse of [`os_workflow_to_flow_host_snapshot_json`]. */
pub fn apply_flow_host_snapshot_to_os_workflow(graph: &Workflow, snapshot_json: &str) -> Vec<WorkflowMutation> {
    let Ok(flow_snapshot) = serde_json::from_str::<Value>(snapshot_json) else {
        return Vec::new();
    };
    let mut operations = Vec::new();
    if let Some(layout) = flow_snapshot.get("layout").and_then(Value::as_object) {
        for node in &graph.nodes {
            let Some(position) = layout.get(&node.id) else { continue };
            let (Some(center_x), Some(center_y)) = (position.get("x").and_then(Value::as_f64), position.get("y").and_then(Value::as_f64)) else {
                continue;
            };
            let x = center_x - node.width / 2.0;
            let y = center_y - node.height / 2.0;
            if (x - node.x).abs() > 1e-6 || (y - node.y).abs() > 1e-6 {
                operations.push(WorkflowMutation::MoveNode(MoveNode { node_id: node.id.clone(), x, y }));
            }
        }
    }
    let mut removed_node_ids = HashSet::new();
    if let Some(widgets) = flow_snapshot.get("widgets").and_then(Value::as_array) {
        let widget_ids: HashSet<&str> = widgets.iter().filter_map(|widget| widget.get("id").and_then(Value::as_str)).collect();
        for node in &graph.nodes {
            if !widget_ids.contains(node.id.as_str()) {
                removed_node_ids.insert(node.id.clone());
                operations.push(WorkflowMutation::RemoveNode(RemoveNode { node_id: node.id.clone() }));
            }
        }
    }
    let synapse_endpoints = |synapse: &Value| -> Option<(String, String, String, String)> {
        Some((synapse.get("from").and_then(Value::as_str)?.into(), synapse.get("fromPort").and_then(Value::as_str)?.into(), synapse.get("to").and_then(Value::as_str)?.into(), synapse.get("toPort").and_then(Value::as_str)?.into()))
    };
    let edge_endpoints = |edge: &WorkflowEdge| (edge.source_node_id.clone(), edge.source_port_id.clone(), edge.target_node_id.clone(), edge.target_port_id.clone());
    let synapses = flow_snapshot.get("synapses").and_then(Value::as_array).cloned().unwrap_or_default();
    let snapshot_endpoints: HashSet<_> = synapses.iter().filter_map(synapse_endpoints).collect();
    let graph_endpoints: HashSet<_> = graph.edges.iter().map(edge_endpoints).collect();
    let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    for synapse in &synapses {
        let Some(endpoints) = synapse_endpoints(synapse) else { continue };
        if graph_endpoints.contains(&endpoints) {
            continue;
        }
        let (source_node_id, source_port_id, target_node_id, target_port_id) = endpoints;
        // 🤝️ Only wire the edge if the endpoints still negotiate a valid contract — a stale/hand-edited
        // snapshot referencing an incompatible or now-removed port silently drops the synapse instead of
        // producing an untyped edge (see `negotiate_media_contract`).
        let Some(source_port) = node_by_id.get(source_node_id.as_str()).and_then(|node| node.outputs.iter().find(|port| port.id == source_port_id)) else { continue };
        let Some(target_port) = node_by_id.get(target_node_id.as_str()).and_then(|node| node.inputs.iter().find(|port| port.id == target_port_id)) else { continue };
        let Ok(contract) = negotiate_media_contract(source_port, target_port) else { continue };
        let id = synapse.get("id").and_then(Value::as_str).filter(|value| !value.is_empty()).map_or_else(|| create_os_id("edge"), str::to_string);
        operations.push(WorkflowMutation::ConnectPorts(ConnectPorts { edge: WorkflowEdge { id, source_node_id, source_port_id, target_node_id, target_port_id, contract } }));
    }
    if flow_snapshot.get("synapses").and_then(Value::as_array).is_some() {
        for edge in &graph.edges {
            if snapshot_endpoints.contains(&edge_endpoints(edge)) {
                continue;
            }
            if removed_node_ids.contains(&edge.source_node_id) || removed_node_ids.contains(&edge.target_node_id) {
                continue;
            }
            operations.push(WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id: edge.id.clone() }));
        }
    }
    operations
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsWorkflowNodeGraphPayload {
    pub nodes_json: String,
    pub edges_json: String,
    pub viewport_json: String,
    pub find_items_json: String,
}

/// 🖼️ Presentation-only media projection; it does not define a persisted workflow codec.
fn workflow_media_contract_payload(contract: &MediaContract) -> Value {
    let wire = match &contract.wire {
        MediaWireFormat::Document { schema } => json!({ "kind": "document", "schema": schema }),
        MediaWireFormat::Intrinsic { schema } => json!({ "kind": "intrinsic", "schema": schema }),
        MediaWireFormat::Binary { format_kind } => json!({ "kind": "binary", "formatKind": format_kind }),
    };
    json!({
        "kindId": contract.kind_id,
        "mediaType": { "class": contract.media_type.class, "form": contract.media_type.form },
        "wire": wire,
        "conversion": contract.conversion.map(|(from, to)| [from, to]),
    })
}

#[cfg(test)]
include!("../🧪️tests/🔬️workflow-standalone/🦀️.rs");

/// 🕸️ Serializes an OS workflow into generic node-graph scene payloads.
///
/// 🚧️ TEMP(Wave 3): still emits JSON-string payloads (typed `NodeGraphScene` records land with
/// WP-0.3/WP-3.2) — kept load-bearing for the space plugin ui crate (WP-1.5) until then. Port keys
/// now read `artifactKind` off `WorkflowMediaPort.spec.kind_id` (was the deleted `OsMediaPort`'s
/// `artifact_kind` string field under the stale `resourceKind` key).
pub fn os_workflow_to_node_graph_payload(graph: &Workflow) -> OsWorkflowNodeGraphPayload {
    let nodes: Vec<_> = graph
        .nodes
        .iter()
        .map(|node| {
            json!({
                "id": node.id,
                "instanceId": node.id,
                "label": format!("{} / {}", node.plugin_id, node.app_id),
                "x": node.x,
                "y": node.y,
                "width": node.width,
                "height": node.height,
                "inputs": node.inputs.iter().map(|port| json!({
                    "id": port.id,
                    "artifactKind": port.spec.kind_id.clone().unwrap_or_default(),
                    "direction": "in",
                    "label": port.spec.label,
                })).collect::<Vec<_>>(),
                "outputs": node.outputs.iter().map(|port| json!({
                    "id": port.id,
                    "artifactKind": port.spec.kind_id.clone().unwrap_or_default(),
                    "direction": "out",
                    "label": port.spec.label,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    let edges: Vec<_> = graph
        .edges
        .iter()
        .map(|edge| {
            json!({
                "id": edge.id,
                "sourceNodeId": edge.source_node_id,
                "sourcePortId": edge.source_port_id,
                "targetNodeId": edge.target_node_id,
                "targetPortId": edge.target_port_id,
                "contract": workflow_media_contract_payload(&edge.contract),
                "isConversion": edge.contract.conversion.is_some(),
            })
        })
        .collect();
    let find_items: Vec<_> = graph.nodes.iter().map(|node| json!({ "id": node.id, "label": format!("{} / {}", node.plugin_id, node.app_id), "category": "Workflow" })).collect();
    OsWorkflowNodeGraphPayload {
        nodes_json: serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
        edges_json: serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
        viewport_json: r#"{"x":0,"y":0,"zoom":1}"#.into(),
        find_items_json: serde_json::to_string(&find_items).unwrap_or_else(|_| "[]".into()),
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct OsWorkflowChannelSpec {
    pub name: String,
    pub code: String,
    pub abbreviation: String,
    pub full_name: String,
    pub operators: Vec<String>,
}

/// 🌱️ `ToValue` only (additive, alongside `serde` — this crate is framework, exempt from the
/// serde ban) — `space`'s `json_array_to_node_graph_operators` shim only ENCODES this type
/// (never decodes it back), see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
/// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct OsWorkflowOperatorInfo {
    pub id: String,
    pub module: String,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
    pub inputs: Vec<OsWorkflowChannelSpec>,
    pub outputs: Vec<OsWorkflowChannelSpec>,
}

/// 🎬️ Derives one `OsWorkflowChannelSpec` directly from a `WorkflowMediaPort`'s
/// `MediaPortSpec` — `operators` (accepted schema ids) is the port's own `kind_id` (falling back to
/// its id when unset, e.g. the implicit `document:*` ports).
fn os_workflow_channel_spec(port: &WorkflowMediaPort, label: &str) -> OsWorkflowChannelSpec {
    let code = port.spec.id.chars().next().map_or_else(|| "P".into(), |ch| ch.to_uppercase().collect::<String>());
    let abbreviation = if label.chars().count() <= 3 { label.into() } else { label.chars().take(3).collect() };
    let operator = port.spec.kind_id.clone().unwrap_or_else(|| port.spec.id.clone());
    OsWorkflowChannelSpec { name: port.spec.id.clone(), code, abbreviation, full_name: label.into(), operators: vec![operator] }
}

/// 🧩️ Registers per-node neuron metadata for the OS workflow flow extension — derived
/// directly from each node's `WorkflowMediaPort.spec: MediaPortSpec` (no more stringly synthesis
/// from a separate `OsAppInstance` join). `parameters`/port-id helpers are the kernel `workflow`
/// crate's own (`workflow::WorkflowParameter`/`workflow::media_port_spec_id`/
/// `workflow::workflow_parameter_id_from_port_id`) — `WorkflowSnapshot.parameters` absorbed the
/// dissolved `OsSnapshot.parameters` in W3, see `## The inversion`.
pub fn build_os_workflow_operator_infos(graph: &Workflow, parameters: &[WorkflowParameter]) -> Vec<OsWorkflowOperatorInfo> {
    let parameter_by_id: HashMap<_, _> = parameters.iter().map(|row| (workflow_parameter_id(row).to_string(), row)).collect();
    graph
        .nodes
        .iter()
        .map(|node| {
            let registration = os_app_registration(&node.plugin_id, &node.app_id);
            let neuron_kind = os_media_neuron_kind_for_node(&node.id);
            OsWorkflowOperatorInfo {
                id: neuron_kind,
                module: OS_MEDIA_FLOW_MODULE_ID.into(),
                name: node.label.clone(),
                abbreviation: if node.app_id.chars().count() <= 3 { node.app_id.clone() } else { node.app_id.chars().take(3).collect() },
                icon: format!("emoji:{}", registration.map_or_else(|| "s".into(), |row| row.component_kind)),
                summary: format!("{}/{}", node.plugin_id, node.app_id),
                inputs: node
                    .inputs
                    .iter()
                    .map(|port| {
                        let parameter_id = workflow_parameter_id_from_port_id(&port.id);
                        let label = parameter_id.as_ref().and_then(|id| parameter_by_id.get(id)).map(|parameter| workflow_parameter_name(parameter)).or_else(|| media_port_spec_id(&port.id)).unwrap_or_else(|| port.id.clone());
                        os_workflow_channel_spec(port, &label)
                    })
                    .collect(),
                outputs: node
                    .outputs
                    .iter()
                    .map(|port| {
                        let label = media_port_spec_id(&port.id).unwrap_or_else(|| port.id.clone());
                        os_workflow_channel_spec(port, &label)
                    })
                    .collect(),
            }
        })
        .collect()
}
//#endregion 🔖️Workflow

// 🧷️ `WorkflowDelivery`/`plan_workflow` now live in the kernel `workflow` crate
// (re-exported above, `producer_node_id`/`consumer_node_id` field names). `WorkflowInstanceRegistry`
// was confirmed dead (zero callers) and deleted outright.

//#region 🔖️MediaExport
//#region 🔖️IoDialectBridge
/// 🌉️ Resolves a legacy OS-workflow kind id (`ArtifactKindSpec.id`/`OsArtifactDescriptor.kind`,
/// e.g. `"3d.cad"`, `"3d.puzzle"`, `"2d.map"` — dimension-prefixed, what `WorkflowNode.yields`
/// and every `register_mesh_exporter`/`register_solid_exporter`/`register_dwg_import_handler`
/// call site historically keyed on) to the REAL `Dialect.artifact_kind` its `ComposerEntry` is
/// registered under (`"s." + component_kind`, e.g. `"s.cad"`, `"s.puzzle3d"`, `"s.gismap"`).
/// Ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1 finding:
/// these are two different id namespaces that happen to look similar, so `registry_export_media`/
/// `registry_import_media`/`registry_shared_stdio_dialect` were silently building a `native_kind`
/// (`format!("s.{artifact_kind}")` off the RAW workflow id) that could never match any real
/// composer dialect -- the "try the typed registry first" path was dead for every production
/// caller, always falling through to the legacy `register_os_media_{export,import}_handler_kind`
/// map even for artifacts (cad, puzzle3d/5d, puzzle2d, gismap) that already carry a full
/// `ComposerEntry` roster for the exact same formats. `OsArtifactDescriptor.component_kind` is
/// the one field already carrying the un-prefixed dialect slug, so no new registry is needed --
/// only reading the right field. An unregistered/synthetic kind (e.g. a unit test's throwaway
/// `"3d.__mesh_exporter_test"`) resolves through `os_artifact_descriptor`'s own placeholder
/// fallback to `"s.panel"`, which will not match any registered dialect and therefore still
/// falls through to the legacy handler map exactly as before -- this bridge only changes
/// resolution for kinds that were ever registered via `register_artifact_descriptor(s)`.
/// 🎯️🆕️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 2: no longer
/// re-derives the `"s.{component_kind}"` string on every call -- reads `crate::registry::
/// os_artifact_dialect(workflow_kind).artifact_kind`, the ONE stored `ArtifactDialect` computed
/// once at registration time (`crate::registry::dialect_from_component_kind`). Return type stays
/// `String` (just the `artifact_kind` segment, no `@standard/subset`) because this function's own
/// two remaining callers (`registry_shared_stdio_dialect` below, and the OLD `io_dispatch`
/// fallback paths in `registry_export_media`/`registry_import_media`) both talk to the OLD
/// `semio_framework_os_kernel::io::dialects_for`/`IoKey` registry (debt D2), whose `Dialect.artifact_kind`
/// is a bare `&str` with no standard/subset fields of its own -- the NEW io-mechanism path below
/// builds a full `ArtifactDialect` directly from `os_artifact_dialect`, not from this function.
fn native_dialect_kind(workflow_kind: &str) -> String {
    crate::registry::os_artifact_dialect(workflow_kind).artifact_kind
}
/// 📜️ The JSON standard (RFC 8259) that marks the JSON interchange dialect among a composer's declared reads and an artifact's export dialects — a standard id, never a plugin id.
const JSON_INTERCHANGE_STANDARD: &str = "rfc8259";
/// 🎯️ A format kind id `<plugin>.<artifact>` names the dialect whose artifact kind is `<domain>.<plugin>.<artifact>`, whichever domain owns it.
fn dialect_names_format(dialect: &semio_framework_artifact_reference::Dialect, format_kind: &str) -> bool {
    dialect.artifact_kind.split_once('.').is_some_and(|(_, qualified)| qualified == format_kind)
}
//#endregion 🔖️IoDialectBridge
//#region 🔖️MediaCapability
pub use crate::registry::os_resource_media_capability;
/// 🗂️ Defined in `semio_framework` alongside `ArtifactKindSpec`; re-exported here
/// verbatim. `os_resource_media_capability` is a registry lookup (see `crate::registry`) driven by each
/// app's declared `ArtifactKindSpec.media_capability` instead of a hardcoded per-app match.
pub use semio_framework::OsMediaCapability;
//#endregion 🔖️MediaCapability

#[derive(Clone, Debug, PartialEq)]
pub struct OsMediaExportResult {
    pub data: String,
    pub mime_type: String,
    pub file_name: String,
    pub encoding: Option<String>,
}

impl OsMediaExportResult {
    /// 📤️ Build an export result from raw bytes + stdio format kind id (the legacy format enum was retired —
    /// ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
    pub fn from_format_kind_bytes(bytes: Vec<u8>, format_artifact_kind: &str, file_stem: &str) -> Result<Self, String> {
        let entry = semio_framework_os_kernel::io::format_descriptor(format_artifact_kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown stdio format kind `{format_artifact_kind}`"))?;
        let mime_type = entry.mimes.first().cloned().ok_or_else(|| format!("stdio format kind `{format_artifact_kind}` has no MIME claim"))?;
        let extension = entry.extensions.first().ok_or_else(|| format!("stdio format kind `{format_artifact_kind}` has no extension claim"))?;
        let data = if entry.is_binary { base64_codec::base64_standard_encode(&bytes) } else { String::from_utf8(bytes).map_err(|error| error.to_string())? };
        Ok(Self { data, mime_type, file_name: format!("{file_stem}{extension}"), encoding: if entry.is_binary { Some("base64".into()) } else { None } })
    }
}

type OsMediaExportHandler = Box<dyn Fn(&Value) -> Result<OsMediaExportResult, String> + Send + Sync>;

fn export_handlers() -> &'static Mutex<HashMap<String, OsMediaExportHandler>> {
    static HANDLERS: OnceLock<Mutex<HashMap<String, OsMediaExportHandler>>> = OnceLock::new();
    HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 🗂️ Registry key `(artifact_kind, format_artifact_kind)` — stdio kind ids.
fn os_media_handler_key(artifact_kind: &str, format_artifact_kind: &str) -> String {
    format!("{artifact_kind}:{format_artifact_kind}")
}

/// 🗄️ Registers an export handler keyed by `(artifact_kind, format_artifact_kind)` stdio/kind ids.
pub fn register_os_media_export_handler_kind(artifact_kind: &str, format_artifact_kind: &str, handler: impl Fn(&Value) -> Result<OsMediaExportResult, String> + Send + Sync + 'static) {
    export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(os_media_handler_key(artifact_kind, format_artifact_kind), Box::new(handler));
}

/// 📤️ Export via `(artifact_kind, format_artifact_kind)` stdio kind ids.
pub fn export_os_app_instance_media_kind(node: &WorkflowNode, source_document: &Value, format_artifact_kind: &str) -> Result<OsMediaExportResult, String> {
    export_os_app_instance_media_kind_cancellable(node, source_document, format_artifact_kind, &semio_framework_async::CancelToken::root_now())
}

/// 📤️ Export that stops at its next IO checkpoint once `cancel` fires; the refusal is the exact string `media export was cancelled`.
pub fn export_os_app_instance_media_kind_cancellable(node: &WorkflowNode, source_document: &Value, format_artifact_kind: &str, cancel: &semio_framework_async::CancelToken) -> Result<OsMediaExportResult, String> {
    let format_kind = semio_framework_os_kernel::io::normalize_format_kind(format_artifact_kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown stdio format kind `{format_artifact_kind}`"))?;
    if let Some(result) = registry_export_media(&node.yields, &format_kind, source_document, cancel) {
        return result;
    }
    let handlers = export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let handler = handlers
        .get(&os_media_handler_key(&node.yields, &format_kind))
        .or_else(|| handlers.get(&os_media_handler_key(&node.yields, format_artifact_kind)))
        .ok_or_else(|| format!("no export handler for {}:{}", node.yields, format_artifact_kind))?;
    handler(source_document)
}

/// 🏁️ How one host IO route run ended.
enum HostIoRun {
    Done(semio_framework::io_schema::IoPayload),
    Failed,
    Cancelled,
}

/// 📏️ A hop may expand its payload (base64, pack framing, text to binary); the host funds this many input extents per hop.
const HOST_IO_EXPANSION: usize = 8;
/// 📏️ Smallest and largest physical extent, in bytes, one host IO run may own at once.
const HOST_IO_MINIMUM_BYTES: usize = 1 << 20;
const HOST_IO_MAXIMUM_BYTES: usize = 1 << 30;
/// 📏️ One item (row, span, frame) is funded per this many bytes of the physical extent.
const HOST_IO_BYTES_PER_ITEM: usize = 64;

/// 🫴️ The host's IO route run. The kernel route runner is one call that checkpoints at every hop and inside every native
/// codec span, not a per-grant stepper, so the host cannot slice it into turns; it bounds and interrupts it instead. The five
/// currencies are sized from the operation itself - the input extent times the route's hops times [`HOST_IO_EXPANSION`], clamped
/// to [`HOST_IO_MINIMUM_BYTES`]..[`HOST_IO_MAXIMUM_BYTES`] - and every native decode, encode and SQLite callback observes `cancel`,
/// so a cancelled export or import stops at its next checkpoint with its partial owners returned to the controls.
fn host_io_run(route: &semio_framework::io_schema::IoRoute, payload: semio_framework::io_schema::IoPayload, cancel: &semio_framework_async::CancelToken) -> HostIoRun {
    use semio_framework_os_kernel::io::io_mechanism::{io_run_with_snapshot_control, IoRunControl};
    let input = match &payload {
        semio_framework::io_schema::IoPayload::Text(text) => text.len(),
        semio_framework::io_schema::IoPayload::Binary(bytes) => bytes.len(),
    };
    let bytes = input.saturating_mul(HOST_IO_EXPANSION).saturating_mul(route.hops.len().max(1)).clamp(HOST_IO_MINIMUM_BYTES, HOST_IO_MAXIMUM_BYTES);
    let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: bytes / HOST_IO_BYTES_PER_ITEM, maximum_copy_bytes: bytes, maximum_capacity_bytes: bytes, maximum_release_bytes: bytes, maximum_depth: 64 };
    let mut decode_callback = |_| !cancel.is_cancelled_now();
    let mut encode_callback = |_| !cancel.is_cancelled_now();
    let mut decode = semio_framework_value::NativeDecodeControl::new(grant.maximum_capacity_bytes, &mut decode_callback);
    let mut encode = semio_framework_value::NativeEncodeControl::new(grant.maximum_capacity_bytes, &mut encode_callback);
    let mut control = IoRunControl::new(&mut decode, &mut encode, grant);
    let mut snapshot_callback = |_| !cancel.is_cancelled_now();
    let mut snapshot = semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut snapshot_callback, Default::default());
    let mut input = Some(payload);
    match crate::host::resolve_kernel_future(io_run_with_snapshot_control(route, &mut input, &mut control, &mut snapshot)) {
        Ok(outcome) => HostIoRun::Done(outcome.value),
        Err(_) if cancel.is_cancelled_now() => HostIoRun::Cancelled,
        Err(_) => HostIoRun::Failed,
    }
}

/// 🎯️🆕️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 1: the io-mechanism
/// export path -- **the fix for the bug this whole ticket exists to remove**. The OLD
/// `registry_export_media_via_composer_registry` below built a FAKE `s.stdio.json@rfc8259/*` "bridge" dialect for
/// `source_document` and handed it to the OLD registry's `io_dispatch`, whose `composed.payload`
/// (per `composer_entry_of`) is the artifact's raw internal `ArtifactPack::encode_pack(snapshot)`
/// bytes for any kind that never registered a REAL composer for the requested format -- so "export
/// as .png" wrote an unopenable `.semio` pack container to a `.png` file. This function instead:
/// (1) resolves `artifact_kind`'s REAL `ArtifactDialect` from the catalog (task 2's
/// `crate::registry::os_artifact_dialect`, never a hardcoded `standard: "1"` string literal built
/// fresh here), (2) picks the carrier dialect (`CARRIER_BINARY`/`CARRIER_TEXT`, design.md §3) by
/// whether the target `format_kind` is binary or text, (3) `io_route`s from the artifact's own
/// dialect all the way to that carrier (up to 3 hops -- a route through the requested format's own
/// dialect and then that format's native codec down to raw bytes is exactly the kind of path this
/// graph search finds), (4) `io_run`s it. The resulting carrier payload IS the raw file content --
/// no JSON bridge, no `IoKey`, no `io_dispatch` -- written verbatim by `from_format_kind_bytes`.
/// Returns `None` (not an error) whenever no route exists yet (every real plugin today -- no
/// subset has migrated onto `declare_artifact`/`io_register` yet, W1-D openQuestion #4), so
/// `registry_export_media` below falls through to the OLD path for every production caller until
/// W2+ cuts real subsets over -- this is debt D2's "coexist, do not bridge" shape, not a silent gap.
fn registry_export_media_via_io_mechanism(artifact_dialect: &ArtifactDialect, format_kind: &str, source_document: &Value, file_stem: &str, cancel: &semio_framework_async::CancelToken) -> Option<Result<OsMediaExportResult, String>> {
    use semio_framework_os_kernel::io::io_mechanism::io_route;
    use semio_framework::io_schema::{IoPayload as NewIoPayload, CARRIER_BINARY, CARRIER_TEXT};

    let is_binary = semio_framework_os_kernel::io::format_descriptor(format_kind).ok().flatten()?.is_binary;
    let carrier: ArtifactDialect = (if is_binary { CARRIER_BINARY } else { CARRIER_TEXT }).into();
    let route = crate::host::resolve_kernel_future(io_route(artifact_dialect, &carrier, 3)).ok()?.value;
    // 🌉️ `source_document` is this artifact's own JSON-shaped snapshot as the OS document store
    // already carries it -- a legitimate `IoPayload::Text` reading of `artifact_dialect`'s own
    // native encoding whenever that dialect's `NativeCodecs.snapshot.text` is a plain-serde-json
    // `ArtifactDsl` impl (the common case today, D7), NOT a re-introduction of the deleted
    // `s.stdio.json` bridge dialect -- the dialect claimed here is the artifact's REAL dialect, not
    // a fake stand-in. If a migrated subset's real DSL grammar differs, the first hop's
    // `Deserializer::deserialize` fails to parse and `io_run` returns `Err`, `.ok()?` below yields
    // `None`, and `registry_export_media` safely falls through to the OLD path -- never a silent
    // wrong-content export.
    let json_text = serde_json::to_string(source_document).ok()?;
    let outcome = match host_io_run(&route, NewIoPayload::Text(json_text), cancel) {
        HostIoRun::Done(outcome) => outcome,
        HostIoRun::Failed => return None,
        HostIoRun::Cancelled => return Some(Err("media export was cancelled".into())),
    };
    let bytes = match outcome {
        NewIoPayload::Binary(b) => b,
        NewIoPayload::Text(t) => t.into_bytes(),
    };
    Some(OsMediaExportResult::from_format_kind_bytes(bytes, format_kind, file_stem))
}

/// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: see the os-side twin of this function for
/// the full rationale -- dispatches export via `io_dispatch` (real subset validation + one-hop
/// fallback guard, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT
/// W6) before falling back to the old stringly handler map.
/// 🐛️ Ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1 fix:
/// `format_kind` here is ALWAYS the caller's already-`normalize_format_kind`d value (see both
/// call sites below), which returns the canonical `FormatDescriptor.kind_id` -- already in
/// `"stdio.<format>"` form, e.g. `"stdio.obj"`. Prefixing another literal `"stdio."` on top
/// built `"s.stdio.stdio.obj"`, which can never match a registered `"s.stdio.obj"` dialect --
/// so this lookup silently missed for EVERY artifact/format pair and always fell through to
/// the legacy handler map, regardless of `native_dialect_kind`. Only `"s."` belongs here.
/// 🎯️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 1 (debt D2): kept
/// working, UNCHANGED logic, as the fallback for artifacts that have not migrated onto the new
/// io-mechanism yet -- W6 deletes this function outright. Never merged with the new path above;
/// `registry_export_media` picks ONE or the other per call, never blends their results.
fn registry_export_media_via_composer_registry(artifact_kind: &str, format_kind: &str, source_document: &Value) -> Option<Result<OsMediaExportResult, String>> {
    use {semio_framework_os_kernel::io::ErasedComposeSource,semio_framework_os_kernel::io::IoDirection,semio_framework_os_kernel::io::IoKey,semio_framework_os_kernel::io::IoPayload};
    let native_kind = native_dialect_kind(artifact_kind);
    let target = match crate::host::resolve_kernel_future(semio_framework_os_kernel::io::dialects_for(&native_kind, IoDirection::Export)) {
        Ok(dialects) => dialects.into_iter().find(|dialect| dialect_names_format(dialect, format_kind))?,
        Err(error) => return Some(Err(format!("{} registry unavailable", error.registry))),
    };
    let key = IoKey {
        artifact_kind: native_kind,
        standard: "1".to_string(),
        subset: "*".to_string(),
        direction: IoDirection::Export,
        format_kind: target.artifact_kind.to_string(),
        format_standard: target.standard.0.to_string(),
        format_subset: target.subset.0.to_string(),
    };
    let composer = crate::host::resolve_kernel_future(semio_framework_os_kernel::io::resolve(&key)).ok()?;
    let json_bridge = *composer.reads.iter().find(|dialect| dialect.standard.0 == JSON_INTERCHANGE_STANDARD)?;
    let json_text = serde_json::to_string(source_document).ok()?;
    let sources = [ErasedComposeSource { dialect: json_bridge, payload: IoPayload::Text(json_text) }];
    let composed = crate::host::resolve_kernel_future(semio_framework_os_kernel::io::io_dispatch(&key, &sources)).ok()?;
    let bytes = match composed.payload {
        IoPayload::Binary(b) => b,
        IoPayload::Text(t) => t.into_bytes(),
    };
    Some(OsMediaExportResult::from_format_kind_bytes(bytes, format_kind, artifact_kind))
}

/// 🚪️ Entry point `export_os_app_instance_media_kind` calls: tries the NEW io-mechanism path
/// first (task 1's real fix), falls through to the OLD `io_dispatch` path (debt D2) when no route
/// exists yet, falls through again to the stringly handler map at the call site. One path wins per
/// call -- never merged (design.md's rejected-approaches list; `📌️important.md`).
fn registry_export_media(artifact_kind: &str, format_kind: &str, source_document: &Value, cancel: &semio_framework_async::CancelToken) -> Option<Result<OsMediaExportResult, String>> {
    let dialect = crate::registry::os_artifact_dialect(artifact_kind);
    if let Some(result) = registry_export_media_via_io_mechanism(&dialect, format_kind, source_document, artifact_kind, cancel) {
        return Some(result);
    }
    registry_export_media_via_composer_registry(artifact_kind, format_kind, source_document)
}

pub fn os_media_export_extension_for_format_kind(format_artifact_kind: &str) -> Result<Option<String>, semio_framework_os_kernel::io::FormatRegistryError> {
    Ok(semio_framework_os_kernel::io::format_descriptor(format_artifact_kind)?.and_then(|row| row.extensions.first().cloned()))
}

type OsMediaImportHandler = Box<dyn Fn(&[u8]) -> Result<Value, String> + Send + Sync>;

fn import_handlers() -> &'static Mutex<HashMap<String, OsMediaImportHandler>> {
    static HANDLERS: OnceLock<Mutex<HashMap<String, OsMediaImportHandler>>> = OnceLock::new();
    HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 🗄️ Registers an import handler keyed by `(artifact_kind, format_artifact_kind)` stdio/kind ids.
pub fn register_os_media_import_handler_kind(artifact_kind: &str, format_artifact_kind: &str, handler: impl Fn(&[u8]) -> Result<Value, String> + Send + Sync + 'static) {
    import_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(os_media_handler_key(artifact_kind, format_artifact_kind), Box::new(handler));
}

/// 📥️ Import via `(artifact_kind, format_artifact_kind)` stdio kind ids.
pub fn import_os_app_instance_media_kind(node: &WorkflowNode, data: &[u8], format_artifact_kind: &str) -> Result<Value, String> {
    import_os_app_instance_media_kind_cancellable(node, data, format_artifact_kind, &semio_framework_async::CancelToken::root_now())
}

/// 📥️ Import that stops at its next IO checkpoint once `cancel` fires; the refusal is the exact string `media import was cancelled`.
pub fn import_os_app_instance_media_kind_cancellable(node: &WorkflowNode, data: &[u8], format_artifact_kind: &str, cancel: &semio_framework_async::CancelToken) -> Result<Value, String> {
    let format_kind = semio_framework_os_kernel::io::normalize_format_kind(format_artifact_kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown stdio format kind `{format_artifact_kind}`"))?;
    if let Some(result) = registry_import_media(&node.yields, &format_kind, data, cancel) {
        return result;
    }
    let handlers = import_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let handler = handlers
        .get(&os_media_handler_key(&node.yields, &format_kind))
        .or_else(|| handlers.get(&os_media_handler_key(&node.yields, format_artifact_kind)))
        .ok_or_else(|| format!("no import handler for {}:{}", node.yields, format_artifact_kind))?;
    handler(data)
}

/// 🎯️🆕️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 1: the io-mechanism
/// import path, mirroring `registry_export_media_via_io_mechanism`'s reasoning in reverse. The
/// caller (`registry_import_media`, via `import_os_app_instance_media_kind`) already knows both
/// `artifact_dialect` (the target the bytes should become) and `format_kind` (what the bytes
/// already are), so per design.md §3 ("When the caller already knows the dialect, skip identify")
/// this is a SINGLE `io_route(carrier -> artifact_dialect)` + `io_run`, never `io_identify` --
/// `io_identify` is for the genuinely-unknown-dialect "open this file" case, not this one.
fn registry_import_media_via_io_mechanism(artifact_dialect: &ArtifactDialect, format_kind: &str, data: &[u8], cancel: &semio_framework_async::CancelToken) -> Option<Result<Value, String>> {
    use semio_framework_os_kernel::io::io_mechanism::io_route;
    use semio_framework::io_schema::{IoPayload as NewIoPayload, CARRIER_BINARY, CARRIER_TEXT};

    let is_binary = semio_framework_os_kernel::io::format_descriptor(format_kind).ok().flatten()?.is_binary;
    let carrier: ArtifactDialect = (if is_binary { CARRIER_BINARY } else { CARRIER_TEXT }).into();
    let carrier_payload = if is_binary { NewIoPayload::Binary(data.to_vec()) } else { NewIoPayload::Text(String::from_utf8(data.to_vec()).ok()?) };
    let route = crate::host::resolve_kernel_future(io_route(&carrier, artifact_dialect, 3)).ok()?.value;
    let outcome = match host_io_run(&route, carrier_payload, cancel) {
        HostIoRun::Done(outcome) => outcome,
        HostIoRun::Failed => return None,
        HostIoRun::Cancelled => return Some(Err("media import was cancelled".into())),
    };
    // 🌉️ Mirrors the export side: the JSON text this yields is read back as this artifact's own
    // OS-document-store shape, not re-wrapped through the deleted `s.stdio.json` bridge dialect.
    let json_text = match outcome {
        NewIoPayload::Text(t) => t,
        NewIoPayload::Binary(b) => String::from_utf8(b).ok()?,
    };
    let value: Value = serde_json::from_str(&json_text).ok()?;
    Some(Ok(value))
}

/// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: see the os-side twin of this function for
/// the full rationale -- two-hop dispatch (target bytes -> native pack bytes -> json text)
/// through `io_dispatch` before falling back to the old stringly handler map.
/// 🐛️ Same double-`"stdio."` fix as `registry_export_media` -- see that function's doc comment.
/// 🎯️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 1 (debt D2): kept
/// working, UNCHANGED logic, as the fallback for artifacts that have not migrated onto the new
/// io-mechanism yet -- W6 deletes this function outright.
fn registry_import_media_via_composer_registry(artifact_kind: &str, format_kind: &str, data: &[u8]) -> Option<Result<Value, String>> {
    use semio_framework_os_kernel::io::ErasedComposeSource;
    use semio_framework_os_kernel::io::IoDirection;
    use semio_framework_os_kernel::io::IoKey;
    use semio_framework_os_kernel::io::IoPayload;
    let native_kind = native_dialect_kind(artifact_kind);

    let source_dialect = match crate::host::resolve_kernel_future(semio_framework_os_kernel::io::dialects_for(&native_kind, IoDirection::Import)) {
        Ok(dialects) => dialects.into_iter().find(|dialect| dialect_names_format(dialect, format_kind))?,
        Err(error) => return Some(Err(format!("{} registry unavailable", error.registry))),
    };
    let import_key = IoKey {
        artifact_kind: native_kind.clone(),
        standard: "1".to_string(),
        subset: "*".to_string(),
        direction: IoDirection::Import,
        format_kind: source_dialect.artifact_kind.to_string(),
        format_standard: source_dialect.standard.0.to_string(),
        format_subset: source_dialect.subset.0.to_string(),
    };
    let sources = [ErasedComposeSource { dialect: source_dialect, payload: IoPayload::Binary(data.to_vec()) }];
    let native = crate::host::resolve_kernel_future(semio_framework_os_kernel::io::io_dispatch(&import_key, &sources)).ok()?;

    let export_dialect = match crate::host::resolve_kernel_future(semio_framework_os_kernel::io::dialects_for(&native_kind, IoDirection::Export)) {
        Ok(dialects) => dialects.into_iter().find(|dialect| dialect.standard.0 == JSON_INTERCHANGE_STANDARD)?,
        Err(error) => return Some(Err(format!("{} registry unavailable", error.registry))),
    };
    let export_key = IoKey {
        artifact_kind: native_kind,
        standard: "1".to_string(),
        subset: "*".to_string(),
        direction: IoDirection::Export,
        format_kind: export_dialect.artifact_kind.to_string(),
        format_standard: export_dialect.standard.0.to_string(),
        format_subset: export_dialect.subset.0.to_string(),
    };
    let native_sources = [ErasedComposeSource { dialect: native.dialect, payload: native.payload }];
    let json_out = crate::host::resolve_kernel_future(semio_framework_os_kernel::io::io_dispatch(&export_key, &native_sources)).ok()?;
    let bytes = match json_out.payload {
        IoPayload::Binary(b) => b,
        IoPayload::Text(t) => t.into_bytes(),
    };
    Some(serde_json::from_slice(&bytes).map_err(|e| e.to_string()))
}

/// 🚪️ Entry point `import_os_app_instance_media_kind` calls: tries the NEW io-mechanism path
/// first, falls through to the OLD `io_dispatch` path (debt D2) when no route exists yet, falls
/// through again to the stringly handler map at the call site. One path wins per call.
fn registry_import_media(artifact_kind: &str, format_kind: &str, data: &[u8], cancel: &semio_framework_async::CancelToken) -> Option<Result<Value, String>> {
    let dialect = crate::registry::os_artifact_dialect(artifact_kind);
    if let Some(result) = registry_import_media_via_io_mechanism(&dialect, format_kind, data, cancel) {
        return Some(result);
    }
    registry_import_media_via_composer_registry(artifact_kind, format_kind, data)
}

//#endregion 🔖️MediaExport

//#region 🧪️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️workflow-unit/🦀️.rs");
//#endregion 🧪️Tests
// #endregion workflow
