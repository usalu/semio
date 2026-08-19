//! 📝️ Imperative play app — the script window: the compiled, read-only textual form of the document.

use crate::editor::imperative::engine::ImperativeHost;
use crate::artifacts::imperative::ImperativeSnapshot;
use semio_framework_plugin::{build_text_editor_scene, LocalizedLabel, SurfaceKind, TextEditorScene, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_WINDOW_SCRIPT: &str = "imperative-script";
pub const IMPERATIVE_PLAY_BODY_SCRIPT: &str = "imperative.play.script";
const IMPERATIVE_PLAY_SURFACE_SCRIPT: &str = "imperative.play.script";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: IMPERATIVE_PLAY_WINDOW_SCRIPT.into(),
        label: LocalizedLabel::native("Script", "Skript"),
        body_key: IMPERATIVE_PLAY_BODY_SCRIPT.into(),
        surface_kind: SurfaceKind::TextEditor,
        icon_id: "file-code".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(document: &ImperativeSnapshot) -> UiNode {
    let host = ImperativeHost::from_snapshot(document.clone());
    build_text_editor_scene(IMPERATIVE_PLAY_SURFACE_SCRIPT, crate::editor::imperative::IMPERATIVE_PLAY_APP_ID, TextEditorScene::base(host.compile_text(), Some("imperative".into()), None))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::imperative::testkit::{imperative_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_script_editor() {
        let mut app = imperative_app();
        assert!(render_body(&mut app, IMPERATIVE_PLAY_BODY_SCRIPT).contains("text-editor"));
    }
}
//#endregion 🧪️Tests
