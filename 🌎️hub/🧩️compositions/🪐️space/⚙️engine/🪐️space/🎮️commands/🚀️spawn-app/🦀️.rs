//! 🧩️ 🧩️ S Studio app command — `spawn-app`.

use semio_hub_space_foundation::config::{SpaceConfig, SpaceConfigMutation, ActiveNodeSetting};
use semio_os::{WorkflowMutation, WorkflowSnapshot};
use semio_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "spawn-app")]
pub struct SpawnApp {
    pub plugin_id: String,
    pub app_id: String,
    pub x: f64,
    pub y: f64,
}

pub fn handle(payload: &SpawnApp, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let Some(app) = semio_os::resolve_os_app_definition(&payload.plugin_id, &payload.app_id) else {
        return Ok(Emit::default());
    };
    if !payload.x.is_finite() || !payload.y.is_finite() {
        return Err(Fault::from("s.space.invalid-workflow-node-position"));
    }
    if let Some(workflow_artifact_id) = cfg.snapshot.workflow_artifact_id.as_deref().filter(|_| !app.io.artifact_schema.is_empty()) {
        let operation = doc.operation_optional().ok_or_else(|| Fault::from("s.space.workflow-authoring-operation-required"))?;
        if operation.parent_document_id != workflow_artifact_id {
            return Err(Fault::from("s.space.workflow-artifact-scope-changed"));
        }
        let node_id = operation.entity_id("node", "spawnApp")?;
        return Ok(Emit::effect(Effect::ReplayShellCommand {
            action_id: "os.create-workflow-artifact".into(),
            args: Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({
                "pluginId": payload.plugin_id.as_str(),
                "appId": app.id.as_str(),
                "nodeId": node_id.as_str(),
                "x": payload.x,
                "y": payload.y,
            }))),
        }));
    }
    match semio_hub_space_operations::resolve_future(semio_hub_space_operations::add_workflow_node_operation(&payload.plugin_id, &payload.app_id, None, payload.x, payload.y)) {
        Some((operation, node_id)) => Ok(Emit { artifact_mutations: vec![operation], config_mutations: vec![SpaceConfigMutation::SetActiveNode(ActiveNodeSetting { node_id: Some(node_id) })], ..Default::default() }),
        None => Ok(Emit::default()),
    }
}


pub mod finish_spawn_app {
    use super::*;
    use semio_hub_space_foundation::config::FocusedNodeSetting;
    use semio_hub_space_operations::{resolve_future, workflow_artifact_reference};
    use semio_os::{WorkflowPosition, workflow_node_for_app};
    use semio_plugin::kernel::PluginDocumentTarget;

    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "finish-spawn-app")]
    pub struct FinishSpawnApp {
        pub plugin_id: String,
        pub app_id: String,
        pub node_id: String,
        pub x: f64,
        pub y: f64,
        pub artifact_id: String,
        pub artifact_schema: String,
        pub space_id: String,
        pub name: String,
    }

    pub fn handle(payload: &FinishSpawnApp, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        let workflow_artifact_id = cfg.snapshot.workflow_artifact_id.as_deref();
        if cfg.snapshot.space_id.as_deref() != Some(payload.space_id.as_str())
            || workflow_artifact_id.is_none()
            || doc.operation_optional().is_some_and(|operation| Some(operation.parent_document_id.as_str()) != workflow_artifact_id)
        {
            return Err(Fault::from("s.space.workflow-artifact-scope-changed"));
        }
        if !payload.x.is_finite() || !payload.y.is_finite() || payload.name.trim().is_empty() {
            return Err(Fault::from("s.space.invalid-workflow-artifact-completion"));
        }
        let app = semio_os::resolve_os_app_definition(&payload.plugin_id, &payload.app_id)
            .filter(|app| app.id == payload.app_id && app.io.artifact_schema == payload.artifact_schema)
            .ok_or_else(|| Fault::from("s.space.workflow-artifact-schema-mismatch"))?;
        let document = PluginDocumentTarget {
            document_id: payload.artifact_id.clone(),
            schema: payload.artifact_schema.clone(),
            space_id: Some(payload.space_id.clone()),
        };
        let artifact_ref = workflow_artifact_reference(&payload.node_id, &document)?;
        if let Some(current) = doc.snapshot.graph.nodes.iter().find(|node| node.id == payload.node_id) {
            if current.plugin_id == payload.plugin_id && current.app_id == payload.app_id && current.artifact_ref == artifact_ref {
                return Ok(Emit::default());
            }
            return Err(Fault::from("s.space.workflow-artifact-node-already-bound"));
        }
        let position = WorkflowPosition { x: payload.x, y: payload.y, width: 0.0, height: 0.0 };
        let mut node = resolve_future(workflow_node_for_app(&app, &payload.plugin_id, &payload.node_id, &position, &payload.name));
        node.artifact_ref = artifact_ref;
        Ok(Emit {
            effects: vec![Effect::OpenPluginInstance { plugin_id: node.plugin_id.clone(), app_id: node.app_id.clone(), os_instance_id: Some(node.id.clone()), document: Some(document) }],
            config_mutations: vec![
                SpaceConfigMutation::SetActiveNode(ActiveNodeSetting { node_id: Some(node.id.clone()) }),
                SpaceConfigMutation::SetFocusedNode(FocusedNodeSetting { node_id: Some(node.id.clone()) }),
            ],
            artifact_mutations: vec![WorkflowMutation::AddNode(semio_os::workflow::AddNode { node })],
            ..Default::default()
        })
    }
}
