//! 🕸️ Persisted app-node workflow graph — nodes reference plugin apps plus document/config artifact refs.

use semio_framework_core::{AppDefinition, MediaPortDirection, MediaPortSpec};
use serde::{Deserialize, Serialize};

pub const WORKFLOW_SCHEMA: &str = "workflow.graph";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowMediaPort {
    pub id: String,
    pub artifact_kind: String,
    pub direction: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowPosition {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowNode {
    pub id: String,
    pub plugin_id: String,
    pub app_id: String,
    pub document_ref: String,
    pub config_ref: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub inputs: Vec<WorkflowMediaPort>,
    pub outputs: Vec<WorkflowMediaPort>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowEdge {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "workflow", layout = "lines")]
pub struct Workflow {
    pub schema: String,
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
}

pub fn empty_workflow() -> Workflow {
    Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() }
}

fn port_from_spec(spec: &MediaPortSpec) -> WorkflowMediaPort {
    let artifact_kind = spec
        .kind_id
        .clone()
        .unwrap_or_else(|| "media".into());
    let direction = match spec.direction {
        MediaPortDirection::In => "in",
        MediaPortDirection::Out => "out",
    };
    WorkflowMediaPort { id: spec.id.clone(), artifact_kind, direction: direction.into() }
}

/// 🧩️ Builds a workflow node shell from a manifest app definition so every app is instantiable as a node.
pub fn workflow_node_for_app(app: &AppDefinition, plugin_id: &str, node_id: &str, position: &WorkflowPosition) -> WorkflowNode {
    let inputs: Vec<WorkflowMediaPort> = app.media_inputs.iter().map(port_from_spec).collect();
    let outputs: Vec<WorkflowMediaPort> = app.media_outputs.iter().map(port_from_spec).collect();
    let port_count = inputs.len().max(outputs.len()).max(1);
    let height = position.height.max(56.0 + port_count as f64 * 18.0);
    WorkflowNode {
        id: node_id.into(),
        plugin_id: plugin_id.into(),
        app_id: app.id.clone(),
        document_ref: format!("documents/{node_id}"),
        config_ref: format!("config/{node_id}"),
        x: position.x,
        y: position.y,
        width: position.width.max(220.0),
        height,
        inputs,
        outputs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_workflow_default() {
        let workflow = empty_workflow();
        assert_eq!(workflow.schema, WORKFLOW_SCHEMA);
        assert!(workflow.nodes.is_empty());
    }
}
