//! 📝️ Imperative play app — the script window: the compiled, read-only textual form of the document.

use crate::artifacts::imperative::ImperativeSnapshot;
use crate::editor::imperative::engine::ImperativeHost;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_WINDOW_SCRIPT: &str = "imperative-script";
pub const IMPERATIVE_PLAY_BODY_SCRIPT: &str = "imperative.play.script";
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
pub fn render(document: &ImperativeSnapshot) -> BuiltNode {
    let host = ImperativeHost::from_snapshot(document.clone());
    TextWindowKit::render(&TextView { text: host.compile_text(), language: Some("imperative".into()), read_only: true })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::imperative::testkit::{imperative_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_script_editor() {
        let mut app = imperative_app().await;
        assert!(render_body(&mut app, IMPERATIVE_PLAY_BODY_SCRIPT).await.contains("text-editor"));
    }
}
//#endregion 🧪️Tests
