//! 🧱️ Graph model of the workflow graph: positions, nodes, edges and the workflow itself with its constructors. See [`super`].

use super::*;

#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct WorkflowPosition {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// 🧷️ A node IS the app-instance now — see the `🔖️InstanceIdentity` region at the top of this file.
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct WorkflowNode {
    pub id: String,
    pub plugin_id: String,
    pub app_id: String,
    pub label: String,
    pub yields: String,
    pub artifact_ref: String,
    pub config_ref: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub inputs: Vec<WorkflowMediaPort>,
    pub outputs: Vec<WorkflowMediaPort>,
}

#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct WorkflowEdge {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
    #[dsl(block)]
    pub contract: MediaContract,
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(extension = "workflow")]
pub struct Workflow {
    pub schema: String,
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
}

pub async fn empty_workflow() -> Workflow {
    Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() }
}

/// 🔌️ Instantiates one `WorkflowMediaPort` from an app-level `MediaPortSpec`, scoping its wire id to
/// this node (`"{node_id}:{spec.id}:{in|out}"`) so the same port declaration produces distinct wire
/// endpoints per node instance.
// 🚫️async: E1 transitive — only consumed by an `Iterator::map` (external trait) closure; pure
// field assembly, no I/O (R9).
pub fn workflow_media_port(node_id: &str, spec: &MediaPortSpec) -> WorkflowMediaPort {
    let direction_word = match spec.direction {
        MediaPortDirection::In => "in",
        MediaPortDirection::Out => "out",
    };
    WorkflowMediaPort { id: format!("{node_id}:{}:{}", spec.id, direction_word), spec: spec.clone() }
}

/// 🧩️ Builds a workflow node shell from a manifest app definition so every app is instantiable as a node.
pub async fn workflow_node_for_app(app: &AppDefinition, plugin_id: &str, node_id: &str, position: &WorkflowPosition) -> WorkflowNode {
    let all_ports = app.io.all_ports().await;
    let inputs: Vec<WorkflowMediaPort> = all_ports.iter().filter(|spec| spec.direction == MediaPortDirection::In).map(|spec| workflow_media_port(node_id, spec)).collect();
    let outputs: Vec<WorkflowMediaPort> = all_ports.iter().filter(|spec| spec.direction == MediaPortDirection::Out).map(|spec| workflow_media_port(node_id, spec)).collect();
    let yields = outputs.first().and_then(|port| port.spec.kind_id.clone()).unwrap_or_default();
    let port_count = inputs.len().max(outputs.len()).max(1);
    let height = position.height.max(56.0 + port_count as f64 * 18.0);
    WorkflowNode {
        id: node_id.into(),
        plugin_id: plugin_id.into(),
        app_id: app.id.clone(),
        // 🚧️ `workflow_node_for_app` carries no locale/terminology context (this node-graph node
        // builder isn't rendered per-request the way UiNode trees are); resolves native/English
        // pending a locale plumbed through the workflow canvas API.
        label: app.label.resolve(Terminology::Native, Locale::En).to_string(),
        yields,
        artifact_ref: format!("artifacts/{node_id}"),
        config_ref: format!("config/{node_id}"),
        x: position.x,
        y: position.y,
        width: position.width.max(220.0),
        height,
        inputs,
        outputs,
    }
}
