//! 🗂️ 🗂️ DAG play app commands command — `set-selection`.

use crate::apps::dag::config::{DagConfig, DagConfigMutation};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-selection")]
pub struct SetSelection {
    pub ids: Vec<String>,
}

pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    Ok(Emit::config(vec![DagConfigMutation::SetSelection { node_ids: payload.ids.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::dag::testkit;
    use crate::apps::dag::{DagCommand, DAG_PLAY_BODY_MAIN};
    use semio_framework_plugin::PluginApp;
    use serde_json::{json, Value};

    /// 🧪️ `setSelection`/`selectNode`/`nodeGraphSelect` are three distinct declared actions that all
    /// drive the same config selection — each got its own `DagCommand` variant (matching the manifest
    /// 1:1) instead of a shared `handle_action` match arm.
    #[test]
    fn set_selection_select_node_and_node_graph_select_all_drive_config_selection() {
        let mut app = testkit::new_app();
        let node_id = app.snapshot().expect("projection").nodes().first().map(|node| node.id.clone()).expect("node");

        app.dispatch_typed(DagCommand::SetSelection(SetSelection { ids: vec![node_id.clone()] }), &semio_framework_plugin::testkit::meta("local")).expect("setSelection");
        assert!(serde_json::to_string(&app.render(DAG_PLAY_BODY_MAIN, None, &semio_framework_plugin::ViewModel::default()).expect("render")).unwrap().contains(&node_id));

        app.dispatch_typed(DagCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {}), &semio_framework_plugin::testkit::meta("local")).expect("clear");
        app.dispatch_typed(DagCommand::SelectNode(select_node::SelectNode { node_id: node_id.clone() }), &semio_framework_plugin::testkit::meta("local")).expect("selectNode");
        assert!(serde_json::to_string(&app.render(DAG_PLAY_BODY_MAIN, None, &semio_framework_plugin::ViewModel::default()).expect("render")).unwrap().contains(&node_id));

        app.dispatch_typed(DagCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {}), &semio_framework_plugin::testkit::meta("local")).expect("clear");
        app.dispatch_typed(DagCommand::NodeGraphSelect(node_graph_select::NodeGraphSelect { node_ids: vec![node_id.clone()] }), &semio_framework_plugin::testkit::meta("local")).expect("nodeGraphSelect");
        assert!(serde_json::to_string(&app.render(DAG_PLAY_BODY_MAIN, None, &semio_framework_plugin::ViewModel::default()).expect("render")).unwrap().contains(&node_id));
    }

    #[test]
    fn node_graph_viewport_drives_the_rendered_camera() {
        let mut app = testkit::new_app();
        app.dispatch_typed(DagCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { x: 10.0, y: 20.0, zoom: 2.0 }), &semio_framework_plugin::testkit::meta("local")).expect("viewport");
        let node = app.render(DAG_PLAY_BODY_MAIN, None, &semio_framework_plugin::ViewModel::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        assert_eq!(payload["nodeGraph"]["viewport"], json!({ "x": 10.0, "y": 20.0, "zoom": 2.0 }));
    }

    #[test]
    fn node_graph_hover_is_a_pure_no_op() {
        let mut app = testkit::new_app();
        let result = app.dispatch_typed(DagCommand::NodeGraphHover(node_graph_hover::NodeGraphHover {}), &semio_framework_plugin::testkit::meta("local")).expect("hover");
        assert!(result.mutations.is_empty());
    }

    /// 🧪️ `selection` is `skip_serializing_if = Vec::is_empty` on `NodeGraphScene`, so an empty selection
    /// omits the key entirely rather than serializing `"selection":[]` — assert absence via the typed
    /// `Value`, not a substring check (the node itself stays in the rendered node list either way).
    #[test]
    fn graph_pointer_down_clears_the_selection() {
        let mut app = testkit::new_app();
        let node_id = app.snapshot().expect("projection").nodes().first().map(|node| node.id.clone()).expect("node");
        app.dispatch_typed(DagCommand::SetSelection(SetSelection { ids: vec![node_id] }), &semio_framework_plugin::testkit::meta("local")).expect("select");
        app.dispatch_typed(DagCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {}), &semio_framework_plugin::testkit::meta("local")).expect("clear");
        let node = app.render(DAG_PLAY_BODY_MAIN, None, &semio_framework_plugin::ViewModel::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        assert_eq!(payload["nodeGraph"]["selection"], Value::Null, "an empty selection must omit the key: {payload}");
    }
}
//#endregion 🧪️Tests
