//! 🖱️ S Studio app — graph hover + viewport/camera commands.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{OsWorkflowCamera, WorkflowSnapshot, WorkflowMutation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};

//#region 🔖️Hover
/// 🔁️ Shared body for `node_graph_hover` and `text_hover` — both decode an optional `{nodeId}` JSON
/// blob (or accept the raw string as-is) into a `SetHover` config operation.
fn hover_operation(hover_json: &Option<String>) -> Vec<SpaceConfigMutation> {
    let node_id = hover_json.as_deref().and_then(|text| serde_json::from_str::<serde_json::Value>(text).ok().and_then(|parsed| parsed.get("nodeId").and_then(|id| id.as_str().map(str::to_string))).or_else(|| Some(text.to_string())));
    vec![SpaceConfigMutation::SetHover { node_id }]
}

pub mod node_graph_hover {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-hover")]
    pub struct NodeGraphHover {
        pub hover_json: Option<String>,
    }

    pub fn handle(payload: &NodeGraphHover, _doc: &DocumentView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        Ok(Emit::config(hover_operation(&payload.hover_json)))
    }
}

pub mod text_hover {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "text-hover")]
    pub struct TextHover {
        pub hover_json: Option<String>,
    }

    pub fn handle(payload: &TextHover, _doc: &DocumentView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        Ok(Emit::config(hover_operation(&payload.hover_json)))
    }
}
//#endregion 🔖️Hover

//#region 🔖️NodeGraphViewport
pub mod node_graph_viewport {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-viewport")]
    pub struct NodeGraphViewport {
        pub viewport_json: String,
    }

    pub fn handle(payload: &NodeGraphViewport, _doc: &DocumentView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        match serde_json::from_str::<OsWorkflowCamera>(&payload.viewport_json) {
            Ok(camera) => Ok(Emit::config(vec![SpaceConfigMutation::SetCamera { window_id: crate::apps::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera: camera.into() }])),
            Err(_) => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️NodeGraphViewport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        use crate::apps::space::SpaceCommand;
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::NodeGraphHover(node_graph_hover::NodeGraphHover { hover_json: Some("{\"nodeId\":\"n1\"}".into()) }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::TextHover(text_hover::TextHover { hover_json: None }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: "{\"x\":0,\"y\":0,\"zoom\":1}".into() }));
    }

    #[test]
    fn node_graph_viewport_persists_camera() {
        use crate::apps::space::testkit::studio_emit;
        use crate::apps::space::SpaceCommand;
        use crate::demo_space_projection;
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, &SpaceCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: r#"{"x":7.0,"y":9.0,"zoom":0.5}"#.into() })).expect("handle");
        assert_eq!(
            emit.config_mutations,
            vec![SpaceConfigMutation::SetCamera { window_id: crate::apps::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera: OsWorkflowCamera { x: 7.0, y: 9.0, zoom: 0.5 }.into() }]
        );
    }
}
//#endregion 🧪️Tests
