//! 👁️ Txt viewer — the read-only counterpart of `✏️editor` for `s.stdio.txt@utf-8/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `TxtViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<TxtViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact mutation. Must
//! not import anything from the sibling mutation-capable surface.

use crate::artifacts::txt::{TxtMutation, TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
use crate::viewer::txt::modes::view;
use crate::viewer::txt::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode, ViewEmit, Viewer};

//#region 🔖️Dialect
/// 🪪️ Same coordinate as the sibling editor surface's own `TXT_EDITOR_DIALECT` — duplicated here
/// on purpose (never imported through the editor module).
pub const TXT_VIEWER_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TxtViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for TxtViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(TxtViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct TxtViewer;

impl ArtifactViewer for TxtViewer {
    type Snapshot = TxtSnapshot;
    type Mutation = TxtMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TxtViewCommand;

    const DIALECT: Dialect = TXT_VIEWER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TXT_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> TxtSnapshot {
        TxtSnapshot::default()
    }

    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &semio_framework_plugin::app::InteractionView<'_>, _engines: &store::EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_txt_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(TXT_VIEWER_DIALECT)
        .document(["semio", "stdio", "txt"])
        .icon_id("type")
        .mode_def(view::definition())
        .default_mode_id(view::TXT_VIEW_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_txt_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_txt_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, TXT_VIEWER_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<TxtViewer as ArtifactViewer>::DIALECT, TXT_VIEWER_DIALECT);
    }

    #[test]
    fn viewer_declares_the_text_window() {
        let def = create_txt_viewer();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
