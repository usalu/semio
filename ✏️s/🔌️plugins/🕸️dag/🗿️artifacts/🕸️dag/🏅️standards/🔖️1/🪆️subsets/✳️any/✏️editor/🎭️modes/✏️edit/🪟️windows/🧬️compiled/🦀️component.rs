//! 🧬️ DAG play app — the compiled-DAG window: the read-only wire literal of the current fixture.
//!
//! `surface_kind` stays `SurfaceKind::NodeGraph` even though `render` builds a text-editor scene —
//! verbatim from the pre-migration manifest, matching the identical precedent in
//! `sequence_ui`'s own compiled window (`🎬️sequence/🎛️apps/🎬️sequence/🎭️modes/✏️edit/🪟️windows/🧬️compiled`).

use crate::editor::dag::DAG_PLAY_APP_ID;
use crate::artifacts::dag::DagSnapshot;
use infinite_board_port_directed_dag::{dag_fixture_from_document, dag_fixture_to_wire_literal, DagCamera};
use semio_framework_plugin::{build_text_editor_scene, LocalizedLabel, SurfaceKind, TextEditorScene, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const DAG_PLAY_WINDOW_COMPILED: &str = "dag-compiled-dag";
pub const DAG_PLAY_BODY_COMPILED: &str = "dag.play.compiled-dag";
const DAG_PLAY_SURFACE_COMPILED: &str = "dag.play.compiled-dag";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: DAG_PLAY_WINDOW_COMPILED.into(),
        label: LocalizedLabel::native("DSL", "DSL"),
        body_key: DAG_PLAY_BODY_COMPILED.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "code".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 🕹️ The compiled-DSL window is read-only text — no interaction domain scoped to it.
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(document: &DagSnapshot, camera: &DagCamera) -> UiNode {
    let fixture = dag_fixture_from_document(&infinite_board_port_directed_dag::DagSnapshot::from(document), camera.clone());
    build_text_editor_scene(DAG_PLAY_SURFACE_COMPILED, DAG_PLAY_APP_ID, TextEditorScene::base(dag_fixture_to_wire_literal(&fixture), Some("wire".into()), None))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::dag::testkit::{new_app, render as render_body};

    #[test]
    async fn renders_compiled_dag_text_editor() {
        let mut app = new_app();
        assert!(render_body(&mut app, DAG_PLAY_BODY_COMPILED).contains("text-editor"));
    }
}
//#endregion 🧪️Tests
