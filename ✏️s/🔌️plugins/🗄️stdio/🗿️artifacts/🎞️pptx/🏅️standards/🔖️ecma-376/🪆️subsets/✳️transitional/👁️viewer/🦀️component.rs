//! 👁️ Pptx transitional viewer — the read-only counterpart of the mutation-capable surface for
//! this subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2).
//! `PptxTransitionalViewer` implements `ArtifactViewer`, never `ArtifactApp` — `ViewerApp<
//! PptxTransitionalViewer>` (framework SDK) is the sole runtime adapter, so this file can never
//! structurally emit an artifact mutation. Must not import anything from the sibling
//! mutation-capable surface.

use crate::artifacts::pptx::{PptxMutation, PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};
use crate::viewer::pptx::standards::v_ecma_376::subsets::transitional::modes::view;
use crate::viewer::pptx::standards::v_ecma_376::subsets::transitional::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode, ViewEmit, Viewer};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — `s.stdio.pptx@ecma-376/transitional`. Duplicated (not imported) from
/// the sibling mutation-capable surface — never shared through that module.
pub const PPTX_TRANSITIONAL_VIEWER_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PptxTransitionalViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for PptxTransitionalViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(PptxTransitionalViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct PptxTransitionalViewer;

impl ArtifactViewer for PptxTransitionalViewer {
    type Snapshot = PptxSnapshot;
    type Mutation = PptxMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = PptxTransitionalViewCommand;

    const DIALECT: Dialect = PPTX_TRANSITIONAL_VIEWER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PPTX_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> PptxSnapshot {
        PptxSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `PptxTransitionalViewCommand::Noop` variant never
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
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_pptx_transitional_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(PPTX_TRANSITIONAL_VIEWER_DIALECT)
        .await.document(["semio", "stdio", "pptx"])
        .icon_id("presentation")
        .mode_def(view::definition())
        .default_mode_id(view::PPTX_TRANSITIONAL_VIEW_MODE_ID)
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
    async fn create_pptx_transitional_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_pptx_transitional_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, PPTX_TRANSITIONAL_VIEWER_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<PptxTransitionalViewer as ArtifactViewer>::DIALECT, PPTX_TRANSITIONAL_VIEWER_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_declares_the_document_window() {
        let def = create_pptx_transitional_viewer();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
