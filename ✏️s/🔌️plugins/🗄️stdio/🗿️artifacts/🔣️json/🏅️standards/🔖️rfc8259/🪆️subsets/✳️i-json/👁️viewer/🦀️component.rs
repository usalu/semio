//! 👁️ Json viewer — the read-only counterpart of `✏️editor` for `s.stdio.json@rfc8259/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `JsonIJsonViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<JsonIJsonViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact mutation. Must
//! not import anything from the sibling mutation-capable surface.

use crate::artifacts::json::{JsonMutation, JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use crate::viewer::json_i_json::modes::view;
use crate::viewer::json_i_json::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, ViewEmit, Viewer};

//#region 🔖️Dialect
/// 🪪️ Same coordinate as the sibling editor surface's own `JSON_I_JSON_EDITOR_DIALECT` — duplicated here
/// on purpose (never imported through the editor module).
pub const JSON_I_JSON_VIEWER_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("i-json") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum JsonIJsonViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for JsonIJsonViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(JsonIJsonViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct JsonIJsonViewer;

impl ArtifactViewer for JsonIJsonViewer {
    type Snapshot = JsonSnapshot;
    type Mutation = JsonMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = JsonIJsonViewCommand;

    const DIALECT: Dialect = JSON_I_JSON_VIEWER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JSON_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> JsonSnapshot {
        JsonSnapshot::default()
    }

    async fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _engines: &store::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::ComponentTree {
        semio_framework_plugin::built_to_component_tree(match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))),
        })
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_json_i_json_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(JSON_I_JSON_VIEWER_DIALECT)
        .document(["semio", "stdio", "json"])
        .icon_id("list-tree")
        .mode_def(view::definition())
        .default_mode_id(view::JSON_I_JSON_VIEW_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_json_i_json_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_json_i_json_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, JSON_I_JSON_VIEWER_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<JsonIJsonViewer as ArtifactViewer>::DIALECT, JSON_I_JSON_VIEWER_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_declares_the_tree_window() {
        let def = create_json_i_json_viewer();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
