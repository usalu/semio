//! 🕸️ S Studio app — Workflow window: definition + render (constitutional: ui/WindowKind + Render).
//! The primary node-graph canvas: spawn/move/connect/select app instances, edit parameters.

use crate::apps::space::config::SpaceConfig;
use crate::apps::space::terminology::SStudioLabels;
use crate::demo_space_projection;
use semio_framework_os::{build_os_workflow_operator_infos, os_workflow_to_flow_fixture, os_workflow_to_node_graph_payload, OsWorkflowCamera, WorkflowSnapshot};
use semio_framework_plugin::{
    build_node_graph_scene, resolve_labels_for_locale, LocalizedLabel, NodeGraphEdgeRecord, NodeGraphFindItem, NodeGraphHover, NodeGraphNodeRecord, NodeGraphOperatorRecord, NodeGraphScene, NodeGraphViewport, SurfaceKind, UiNode, WindowEngagement,
    WindowEngagementInput, WindowEngagementSlot, WindowEngagementStatus, WindowKindDefinition, WindowOptions,
};
use serde::Serialize;

//#region 🔖️Constants
pub const S_PLAY_WINDOW_WORKFLOW: &str = "s-workflow";
pub const S_PLAY_BODY_WORKFLOW: &str = "s.play.workflow";
pub const S_PLAY_SURFACE_WORKFLOW: &str = "s.play.workflow";
//#endregion 🔖️Constants

//#region 🔖️Manifest
fn workflow_engagement(config: &SpaceConfig, node_count: usize) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("s-media-catalogue-hint".into()),
            value: Some(config.workflow_engagement_input.clone()),
            placeholder: Some("Drag apps from Catalogue workbench tab".into()),
            on_change: Some(crate::apps::space::s_play_action("workflowEngagementInput", None)),
            on_submit: Some(crate::apps::space::s_play_action("workflowEngagementSubmit", None)),
            disabled: None,
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "s-media-count".into(), text: format!("{node_count} nodes") }]),
        possible_engagements: None,
    }
}

pub fn definition() -> WindowKindDefinition {
    let projection = demo_space_projection();
    let config = SpaceConfig::default();
    let engagement = workflow_engagement(&config, projection.graph.nodes.len());
    WindowKindDefinition {
        id: S_PLAY_WINDOW_WORKFLOW.into(),
        label: LocalizedLabel::native("Workflow", "Workflow"),
        body_key: S_PLAY_BODY_WORKFLOW.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "graph-media".into(),
        // 📇️ `options.measures` stays EMPTY here — measures are config-derived per frame by
        // `DocumentApp::window_measures`, never frozen into the manifest (see the `🎚️options` node's
        // `measure(config, labels)` this window collects from). `options.engagement` DOES get a static
        // initial value baked in (there is no per-frame `DocumentApp::engagement()` trait method).
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::Some(engagement) },
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        document_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Manifest

//#region 🔖️Render
// TEMP(Wave 3): space producer flip pending — `os_workflow_to_node_graph_payload`/
// `build_os_workflow_operator_infos` still emit JSON-string payloads, while `NodeGraphScene` is now
// typed. These shims JSON-decode the existing wire shape into the new typed records so this one call
// site compiles without doing the real space producer cutover. Delete once the space producer is
// flipped to build the typed records directly.
fn json_array_to_node_graph_nodes(json: &str) -> Vec<NodeGraphNodeRecord> {
    serde_json::from_str(json).unwrap_or_default()
}

fn json_array_to_node_graph_edges(json: &str) -> Vec<NodeGraphEdgeRecord> {
    serde_json::from_str(json).unwrap_or_default()
}

fn json_array_to_node_graph_find_items(json: &str) -> Vec<NodeGraphFindItem> {
    serde_json::from_str(json).unwrap_or_default()
}

fn json_array_to_node_graph_operators<T: Serialize>(operators: &[T]) -> Vec<NodeGraphOperatorRecord> {
    serde_json::to_string(operators).ok().and_then(|json| serde_json::from_str(&json).ok()).unwrap_or_default()
}
// TEMP(Wave 3) end

fn workflow_camera(config: &SpaceConfig) -> OsWorkflowCamera {
    config.camera.get(S_PLAY_WINDOW_WORKFLOW).copied().map(Into::into).unwrap_or_default()
}

pub fn render(app: &crate::apps::space::SpaceApp, projection: &WorkflowSnapshot, config: &SpaceConfig) -> UiNode {
    let graph_payload = os_workflow_to_node_graph_payload(&projection.graph);
    let camera = workflow_camera(config);
    let fixture = os_workflow_to_flow_fixture(&projection.graph, &camera);
    let operators = build_os_workflow_operator_infos(&projection.graph, &projection.parameters);
    let selection = config.selected_node_ids.clone();
    let hover = config.hovered_node_id.as_ref().map(|id| NodeGraphHover { node_id: Some(id.clone()) });
    build_node_graph_scene(
        S_PLAY_SURFACE_WORKFLOW,
        crate::apps::space::S_PLAY_CONTROLLER_ID,
        NodeGraphScene {
            editable: Some(true),
            operators: json_array_to_node_graph_operators(&operators),
            find_items: json_array_to_node_graph_find_items(&graph_payload.find_items_json),
            selection,
            hover,
            capabilities_json: Some(r#"{"engine":"flow","spotlight":false,"noteEdit":false,"clusters":false}"#.into()),
            fixture_json: Some(fixture.to_string()),
            presence_peers_json: Some(crate::apps::space::presence_peers_json(app, config)),
            ..NodeGraphScene::base(json_array_to_node_graph_nodes(&graph_payload.nodes_json), json_array_to_node_graph_edges(&graph_payload.edges_json), NodeGraphViewport { x: camera.x, y: camera.y, zoom: camera.zoom })
        },
    )
}
//#endregion 🔖️Render

//#region 🔖️Measures
pub fn window_measures(config: &SpaceConfig, nodes: &[semio_framework_os::WorkflowNode]) -> Vec<semio_framework_plugin::WindowMeasure> {
    let labels = resolve_labels_for_locale::<SStudioLabels>(&config.locale);
    vec![crate::apps::space::modes::main::windows::workflow::options::active_instance::measure(config, nodes, labels)]
}
//#endregion 🔖️Measures

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_workflow_scene() {
        use semio_framework_plugin::{PluginApp, ViewModel, VcsDocumentApp};
        let mut app = VcsDocumentApp::new(crate::apps::space::SpaceApp::default());
        let node = app.render(S_PLAY_BODY_WORKFLOW, None, &ViewModel::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("node-graph"));
    }

    #[test]
    fn workflow_scene_uses_flow_engine_with_fixture() {
        use semio_framework_plugin::{PluginApp, ViewModel, VcsDocumentApp};
        let mut app = VcsDocumentApp::new(crate::apps::space::SpaceApp::default());
        let node = app.render(S_PLAY_BODY_WORKFLOW, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(r#"\"engine\":\"flow\""#));
        assert!(json.contains("fixtureJson"));
        assert!(json.contains(r#"\"schema\":\"flow.fixture\""#));
    }
}
//#endregion 🧪️Tests
