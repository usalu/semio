//! 👁️ PDF/A Document (1.4) viewer -- the read-only counterpart of the mutation-capable surface for this
//! subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Pdf14AViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` -- `ViewerApp<Pdf14AViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! mutation. Must not import anything from the sibling mutation-capable surface (viewer purity).

use crate::artifacts::pdf::{PdfMutation, PdfSnapshot, PDF_ARTIFACT_SCHEMA_ID, STDIO_PDF_DOCUMENT_SCHEMA};
use crate::viewer::pdf14a::modes::view;
use crate::viewer::pdf14a::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode, ViewEmit, Viewer};

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract: this file's own surface-id
/// coordinate -- `s.stdio.pdf@1.4/a` -- measured directly against `PDF_ARTIFACT_SCHEMA_ID`
/// and this file's own on-disk standard/subset location. Own copy -- never imported from the mutation-capable surface, per this file's own purity rule.
pub const PDF14A_DIALECT: Dialect = Dialect { artifact_kind: PDF_ARTIFACT_SCHEMA_ID, standard: StandardId("1.4"), subset: SubsetId("a") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Pdf14AViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Pdf14AViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Pdf14AViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Pdf14AViewer;

impl ArtifactViewer for Pdf14AViewer {
    type Snapshot = PdfSnapshot;
    type Mutation = PdfMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Pdf14AViewCommand;

    const DIALECT: Dialect = PDF14A_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PDF_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> PdfSnapshot {
        PdfSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `Noop` variant never carries a config change. Kept as a
    /// real dispatch (not `unreachable!()`) so a future view-only action is a pure addition.
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
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_pdf14_a_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(PDF14A_DIALECT)
        .await.document(["stdio", "pdf", "1.4", "a"])
        .icon_id("file-text")
        .mode_def(view::definition())
        .default_mode_id(view::PDF14A_VIEW_MODE_ID)
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
    async fn create_pdf14_a_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_pdf14_a_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, PDF14A_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Pdf14AViewer as ArtifactViewer>::DIALECT, PDF14A_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_declares_the_main_window() {
        let def = create_pdf14_a_viewer();
        assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
