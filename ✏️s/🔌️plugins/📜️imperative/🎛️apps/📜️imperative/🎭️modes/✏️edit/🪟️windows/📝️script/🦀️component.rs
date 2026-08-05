//! 📝️ Imperative play app — the script window: the compiled, read-only textual form of the document.

use crate::artifacts::imperative::engine::ImperativeHost;
use crate::artifacts::imperative::ImperativeDocument;
use semio_framework_plugin::{build_text_editor_scene, LocalizedLabel, SurfaceKind, TextEditorScene, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_WINDOW_SCRIPT: &str = "imperative-script";
pub const IMPERATIVE_PLAY_BODY_SCRIPT: &str = "imperative.play.script";
const IMPERATIVE_PLAY_SURFACE_SCRIPT: &str = "imperative.play.script";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: IMPERATIVE_PLAY_WINDOW_SCRIPT.into(),
        label: LocalizedLabel::native("Script", "Skript"),
        body_key: IMPERATIVE_PLAY_BODY_SCRIPT.into(),
        surface_kind: SurfaceKind::TextEditor,
        icon_id: "file-code".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        document_projection_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &ImperativeDocument) -> UiNode {
    let host = ImperativeHost::from_document(document.clone());
    build_text_editor_scene(IMPERATIVE_PLAY_SURFACE_SCRIPT, crate::apps::imperative::IMPERATIVE_PLAY_APP_ID, TextEditorScene::base(host.compile_text(), Some("imperative".into()), None))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::imperative::testkit::{imperative_app, render as render_body};

    #[test]
    fn renders_script_editor() {
        let mut app = imperative_app();
        assert!(render_body(&mut app, IMPERATIVE_PLAY_BODY_SCRIPT).contains("text-editor"));
    }
}
//#endregion 🧪️Tests
