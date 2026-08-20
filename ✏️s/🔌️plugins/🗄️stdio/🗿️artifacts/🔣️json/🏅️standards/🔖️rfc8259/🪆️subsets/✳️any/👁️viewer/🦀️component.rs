//! 👁️ Json viewer — the read-only counterpart of `✏️editor` for `s.stdio.json@rfc8259/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `JsonAnyViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<JsonAnyViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact mutation. Must
//! not import anything from the sibling mutation-capable surface.

use crate::artifacts::json::{JsonMutation, JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use crate::viewer::json_any::modes::view;
use crate::viewer::json_any::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode, ViewEmit, Viewer};

//#region 🔖️Dialect
/// 🪪️ Same coordinate as the sibling editor surface's own `JSON_EDITOR_DIALECT` — duplicated here
/// on purpose (never imported through the editor module).
pub const JSON_VIEWER_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum JsonViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for JsonViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(JsonViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct JsonAnyViewer;

impl ArtifactViewer for JsonAnyViewer {
    type Snapshot = JsonSnapshot;
    type Mutation = JsonMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = JsonViewCommand;

    const DIALECT: Dialect = JSON_VIEWER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JSON_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> JsonSnapshot {
        JsonSnapshot::default()
    }

    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &semio_framework_plugin::app::InteractionView<'_>, _engines: &store::EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).await,
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))).await,
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_json_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(JSON_VIEWER_DIALECT)
        .await.document(["semio", "stdio", "json"])
        .await.icon_id("list-tree")
        .await.mode_def(view::definition().await)
        .await.default_mode_id(view::JSON_VIEW_MODE_ID)
        .await.window_kind_def(main::definition().await)
        .await.default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_json_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_json_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, JSON_VIEWER_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<JsonAnyViewer as ArtifactViewer>::DIALECT, JSON_VIEWER_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_declares_the_tree_window() {
        let def = create_json_viewer();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
