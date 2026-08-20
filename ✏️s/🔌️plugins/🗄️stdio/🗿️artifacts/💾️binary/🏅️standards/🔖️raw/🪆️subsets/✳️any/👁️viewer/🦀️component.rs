//! 👁️ Binary viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `BinaryViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<BinaryViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact mutation. Must
//! not import anything from the sibling mutation-capable surface (policy forbids the substring
//! outright, including inside comments).

use crate::artifacts::binary::{BinaryMutation, BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::viewer::binary::modes::view;
use crate::viewer::binary::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode, ViewEmit, Viewer};

//#region 🔖️Dialect
/// 🎯️ This surface's dialect coordinate — `s.stdio.binary@raw/*`. Kept as its own independent const
/// (never imported from the sibling authoring surface) so this file can never reach the
/// mutation-capable module even transitively.
pub const BINARY_VIEWER_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BinaryViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for BinaryViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(BinaryViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct BinaryViewer;

impl ArtifactViewer for BinaryViewer {
    type Snapshot = BinarySnapshot;
    type Mutation = BinaryMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = BinaryViewCommand;

    const DIALECT: Dialect = BINARY_VIEWER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_BINARY_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> BinarySnapshot {
        BinarySnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `BinaryViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit`.
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_binary_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(BINARY_VIEWER_DIALECT)
        .await.document(["stdio", "binary"])
        .icon_id("binary")
        .mode_def(view::definition())
        .default_mode_id(view::BINARY_VIEW_MODE_ID)
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
    async fn create_binary_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_binary_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, BINARY_VIEWER_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<BinaryViewer as ArtifactViewer>::DIALECT, BINARY_VIEWER_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_declares_the_main_window() {
        let def = create_binary_viewer();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
