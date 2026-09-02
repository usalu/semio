//! 👁️ Remodeling viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `RemodelingViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<RemodelingViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::remodeling::{default_remodeling_scene, RemodelingSnapshot, REMODELING_DIALECT, REMODELING_DOCUMENT_SCHEMA};
use crate::viewer::remodeling::modes::view;
use crate::viewer::remodeling::modes::view::windows::model;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RemodelingViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for RemodelingViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(RemodelingViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct RemodelingViewer;

impl ArtifactViewer for RemodelingViewer {
    type Snapshot = RemodelingSnapshot;
    type Mutation = crate::artifacts::remodeling::op::RemodelingMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = RemodelingViewCommand;

    const DIALECT: Dialect = REMODELING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = REMODELING_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> RemodelingSnapshot {
        default_remodeling_scene()
    }

    /// 👁️ Structurally read-only: the sole `RemodelingViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// orbit, "jump to frame") is a pure addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            model::BODY_KEY => model::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_remodeling_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(REMODELING_DIALECT)
        .document(["semio", "remodeling"])
        .icon_id("remodeling-app")
        .mode_def(view::definition())
        .default_mode_id(view::REMODELING_VIEW_MODE_VIEW)
        .window_kind_def(model::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_remodeling_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_remodeling_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, REMODELING_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<RemodelingViewer as ArtifactViewer>::DIALECT, REMODELING_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_renders_the_model_window_body_and_falls_back_by_name_otherwise() {
        let scene = default_remodeling_scene();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&scene, &history);
        let cfg = ConfigView { snapshot: &NoConfig::default() };
        let _rendered = <RemodelingViewer as ArtifactViewer>::render(model::BODY_KEY, &doc, &cfg);
        let fallback = <RemodelingViewer as ArtifactViewer>::render("nonsense", &doc, &cfg);
        assert!(format!("{fallback:?}").contains("nonsense"));
    }
}
//#endregion 🧪️Tests
