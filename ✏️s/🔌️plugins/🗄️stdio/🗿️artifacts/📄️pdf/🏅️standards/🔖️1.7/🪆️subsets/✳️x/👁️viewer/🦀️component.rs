//! 👁️ PDF/X Document (1.7) viewer -- the read-only counterpart of the mutation-capable surface for this
//! subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Pdf17XViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` -- `ViewerApp<Pdf17XViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! mutation. Must not import anything from the sibling mutation-capable surface (viewer purity).

use crate::artifacts::pdf::{PdfMutation, PdfSnapshot, PDF_ARTIFACT_SCHEMA_ID, STDIO_PDF_DOCUMENT_SCHEMA};
use crate::viewer::pdf17x::modes::view;
use crate::viewer::pdf17x::modes::view::windows::main;
use semio_framework_plugin::{
    built_to_component_tree, ArtifactView, ArtifactViewer, ComponentTree, ConfigView, Dialect, Fault, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, ViewEmit, Viewer,
};
use semio_framework_ui_contract::Buildable;

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract: this file's own surface-id
/// coordinate -- `s.stdio.pdf@1.7/x` -- measured directly against `PDF_ARTIFACT_SCHEMA_ID`
/// and this file's own on-disk standard/subset location. Own copy -- never imported from the mutation-capable surface, per this file's own purity rule.
pub const PDF17X_DIALECT: Dialect = Dialect { artifact_kind: PDF_ARTIFACT_SCHEMA_ID, standard: StandardId("1.7"), subset: SubsetId("x") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Pdf17XViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Pdf17XViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Pdf17XViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Pdf17XViewer;

impl ArtifactViewer for Pdf17XViewer {
    type Snapshot = PdfSnapshot;
    type Mutation = PdfMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Pdf17XViewCommand;

    const DIALECT: Dialect = PDF17X_DIALECT;
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

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> ComponentTree {
        match body_key {
            main::BODY_KEY => built_to_component_tree(main::render(doc.snapshot)),
            _ => built_to_component_tree(semio_framework_ui_contract::text(format!("Unknown body: {body_key}")).build()),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_pdf17_x_viewer() -> semio_framework_plugin::AppDefinition {
    let builder = Viewer::builder(PDF17X_DIALECT);
    let builder = semio_framework_plugin::resolve_ready(builder.document(["stdio", "pdf", "1.7", "x"]));
    let builder = semio_framework_plugin::resolve_ready(builder.icon_id("file-text"));
    let builder = semio_framework_plugin::resolve_ready(builder.mode_def(view::definition()));
    let builder = semio_framework_plugin::resolve_ready(builder.default_mode_id(view::PDF17X_VIEW_MODE_ID));
    let builder = semio_framework_plugin::resolve_ready(builder.window_kind_def(main::definition()));
    let builder = semio_framework_plugin::resolve_ready(builder.default_layout(view::layout()));
    semio_framework_plugin::resolve_ready(builder.build_definition())
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_pdf17_x_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_pdf17_x_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, PDF17X_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Pdf17XViewer as ArtifactViewer>::DIALECT, PDF17X_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_declares_the_main_window() {
        let def = create_pdf17_x_viewer();
        assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
