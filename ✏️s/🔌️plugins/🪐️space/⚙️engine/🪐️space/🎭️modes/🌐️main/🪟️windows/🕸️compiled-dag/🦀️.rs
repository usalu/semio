//! 🕸️ S Studio app — Compiled DAG window: definition + render (constitutional: ui/WindowKind + Render).
//! A read-only text view of the workflow projected onto the generic port-directed-DAG wire literal.

use crate::demo_space_projection;
use crate::engine::space::engine::compiled_dag_wire_literal;
use semio_framework_os::WorkflowSnapshot;
use semio_framework_plugin::{InteractionRef, LocalizedLabel, SurfaceKind, WindowEngagement, WindowEngagementSlot, WindowEngagementStatus, WindowKindDefinition, WindowOptions};
use semio_framework_ui_scene::TextEditorScene;

//#region 🔖️Constants
pub const S_PLAY_WINDOW_COMPILED_DAG: &str = "s-compiled-dag";
pub const S_PLAY_BODY_COMPILED_DAG: &str = "s.play.compiled-dag";
pub const S_PLAY_SURFACE_COMPILED_DAG: &str = "s.play.compiled-dag";
//#endregion 🔖️Constants

//#region 🔖️Manifest
async fn compiled_dag_engagement(projection: &WorkflowSnapshot) -> WindowEngagement {
    let wire = compiled_dag_wire_literal(projection).await;
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: None,
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "s-compiled-dag-status".into(), text: if wire.trim().is_empty() { "Empty".into() } else { "Compiled".into() } }]),
        possible_engagements: None,
    }
}

pub async fn definition() -> WindowKindDefinition {
    let engagement = compiled_dag_engagement(&demo_space_projection().await).await;
    WindowKindDefinition {
        id: S_PLAY_WINDOW_COMPILED_DAG.into(),
        label: LocalizedLabel::native("Compiled DAG", "Kompilierter DAG"),
        body_key: S_PLAY_BODY_COMPILED_DAG.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "git-merge".into(),
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::Some(engagement) },
        actions: Vec::new(),
        utilities: Vec::new(),
        // 🕹️ Read-only text projection of the SAME `graph` node graph the Workflow window edits — hosts
        // the "graph" domain too (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
        interactions: vec![InteractionRef::new(crate::engine::space::S_PLAY_INTERACTION_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Manifest

//#region 🔖️Render
pub async fn render(projection: &WorkflowSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let wire = compiled_dag_wire_literal(projection).await;
    semio_framework_plugin::scene_surface(S_PLAY_SURFACE_COMPILED_DAG, semio_framework_ui_contract::SurfaceKind::TextEditor, &TextEditorScene::base(wire, Some("wire".into()), None))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn renders_compiled_dag_editor() {
        use semio_framework_plugin::{PluginApp, VcsArtifactApp, ViewModel};
        let mut app = VcsArtifactApp::new(crate::engine::space::SpaceApp::default());
        let node = app.render(S_PLAY_BODY_COMPILED_DAG, None, &ViewModel::default()).expect("render");
        assert!(pack::to_json_string(&node).contains("text-editor"));
        let wire = compiled_dag_wire_literal(&demo_space_projection().await).await;
        assert!(wire.contains("appInstance") || wire.contains("draw"));
    }
}
//#endregion 🧪️Tests
