//! 👁️ Tsv viewer — the read-only counterpart of `✏️editor` for `s.stdio.tsv@iana/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `TsvViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<TsvViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact mutation. Must
//! not import anything from the sibling mutation-capable surface.

use crate::artifacts::tsv::{TsvMutation, TsvSnapshot, STDIO_TSV_DOCUMENT_SCHEMA};
use crate::viewer::tsv::modes::view;
use crate::viewer::tsv::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode, ViewEmit, Viewer};

//#region 🔖️Dialect
/// 🪪️ Same coordinate as the sibling editor surface's own `TSV_EDITOR_DIALECT` — duplicated here
/// on purpose (never imported through the editor module).
pub const TSV_VIEWER_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tsv", standard: StandardId("iana"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TsvViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for TsvViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(TsvViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct TsvViewer;

impl ArtifactViewer for TsvViewer {
    type Snapshot = TsvSnapshot;
    type Mutation = TsvMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TsvViewCommand;

    const DIALECT: Dialect = TSV_VIEWER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TSV_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> TsvSnapshot {
        TsvSnapshot::default()
    }

    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &semio_framework_plugin::app::InteractionView<'_>, _engines: &store::EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_tsv_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(TSV_VIEWER_DIALECT)
        .document(["semio", "stdio", "tsv"])
        .icon_id("table-2")
        .mode_def(view::definition())
        .default_mode_id(view::TSV_VIEW_MODE_ID)
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
    async fn create_tsv_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_tsv_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, TSV_VIEWER_DIALECT.into());
    }

    #[test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<TsvViewer as ArtifactViewer>::DIALECT, TSV_VIEWER_DIALECT);
    }

    #[test]
    async fn viewer_declares_the_table_window() {
        let def = create_tsv_viewer();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
