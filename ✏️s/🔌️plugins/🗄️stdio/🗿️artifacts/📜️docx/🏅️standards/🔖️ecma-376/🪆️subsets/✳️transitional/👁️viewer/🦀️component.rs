//! 👁️ Docx transitional viewer — the read-only counterpart of the mutation-capable surface for
//! this subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2).
//! `DocxTransitionalViewer` implements `ArtifactViewer`, never `ArtifactApp` — `ViewerApp<
//! DocxTransitionalViewer>` (framework SDK) is the sole runtime adapter, so this file can never
//! structurally emit an artifact mutation. Must not import anything from the sibling
//! mutation-capable surface.

use crate::artifacts::docx::{DocxMutation, DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};
use crate::viewer::docx::standards::v_ecma_376::subsets::transitional::modes::view;
use crate::viewer::docx::standards::v_ecma_376::subsets::transitional::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode, ViewEmit, Viewer};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — `s.stdio.docx@ecma-376/transitional`. Duplicated (not imported) from
/// the sibling mutation-capable surface — never shared through that module.
pub const DOCX_TRANSITIONAL_VIEWER_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DocxTransitionalViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for DocxTransitionalViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(DocxTransitionalViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct DocxTransitionalViewer;

impl ArtifactViewer for DocxTransitionalViewer {
    type Snapshot = DocxSnapshot;
    type Mutation = DocxMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = DocxTransitionalViewCommand;

    const DIALECT: Dialect = DOCX_TRANSITIONAL_VIEWER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_DOCX_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> DocxSnapshot {
        DocxSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `DocxTransitionalViewCommand::Noop` variant never
    /// carries a config change, so this always returns the empty `ViewEmit`.
    async fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _engines: &store::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))).await,
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_docx_transitional_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(DOCX_TRANSITIONAL_VIEWER_DIALECT)
        .document(["semio", "stdio", "docx"])
        .icon_id("file-text")
        .mode_def(view::definition())
        .default_mode_id(view::DOCX_TRANSITIONAL_VIEW_MODE_ID)
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
    async fn create_docx_transitional_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_docx_transitional_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, DOCX_TRANSITIONAL_VIEWER_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<DocxTransitionalViewer as ArtifactViewer>::DIALECT, DOCX_TRANSITIONAL_VIEWER_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_declares_the_document_window() {
        let def = create_docx_transitional_viewer();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
