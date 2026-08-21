//! 👁️ Zip viewer (2.0/✳️any) — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `ZipAnyViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<ZipAnyViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact mutation. Must
//! not import anything from the sibling mutation-capable surface (policy forbids the substring
//! outright, including inside comments).

use crate::artifacts::zip::{STDIO_ZIP_DOCUMENT_SCHEMA, ZipMutation, ZipSnapshot};
use crate::viewer::zip::any::modes::view;
use crate::viewer::zip::any::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode, ViewEmit, Viewer};

//#region 🔖️Dialect
/// 🎯️ This surface's dialect coordinate — `s.stdio.zip@2.0/*`. Kept as its own independent const
/// (never imported from the sibling authoring surface) so this file can never reach the
/// mutation-capable module even transitively.
pub const ZIP_ANY_VIEWER_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ZipAnyViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for ZipAnyViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(ZipAnyViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct ZipAnyViewer;

impl ArtifactViewer for ZipAnyViewer {
    type Snapshot = ZipSnapshot;
    type Mutation = ZipMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = ZipAnyViewCommand;

    const DIALECT: Dialect = ZIP_ANY_VIEWER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_ZIP_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> ZipSnapshot {
        ZipSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `ZipAnyViewCommand::Noop` variant never carries a config
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
pub fn create_zip_any_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(ZIP_ANY_VIEWER_DIALECT)
        .document(["stdio", "zip", "any"])
        .icon_id("archive")
        .mode_def(view::definition())
        .default_mode_id(view::ZIP_ANY_VIEW_MODE_ID)
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
    async fn create_zip_any_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_zip_any_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, ZIP_ANY_VIEWER_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<ZipAnyViewer as ArtifactViewer>::DIALECT, ZIP_ANY_VIEWER_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_declares_the_main_window() {
        let def = create_zip_any_viewer();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
