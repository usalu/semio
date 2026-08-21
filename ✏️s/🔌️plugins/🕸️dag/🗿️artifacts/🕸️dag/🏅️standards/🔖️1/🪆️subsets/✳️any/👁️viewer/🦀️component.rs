//! 👁️ DAG viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `DagViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<DagViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::dag::{default_snapshot, DagSnapshot, DAG_DIALECT, DAG_DOCUMENT_SCHEMA};
use crate::viewer::dag::modes::view;
use crate::viewer::dag::modes::view::windows::main;
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DagViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for DagViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(DagViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct DagViewer;

impl ArtifactViewer for DagViewer {
    type Snapshot = DagSnapshot;
    type Mutation = crate::artifacts::dag::op::DagMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = DagViewCommand;

    const DIALECT: Dialect = DAG_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DAG_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> DagSnapshot {
        default_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `DagViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// pan) is a pure addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
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
pub async fn create_dag_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(DAG_DIALECT)
        .document(["semio", "mathematical", "graph", "port", "directed", "dag"])
        .icon_id("dag")
        .mode_def(view::definition())
        .default_mode_id(view::DAG_VIEW_MODE_VIEW)
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
    async fn create_dag_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_dag_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, DAG_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<DagViewer as ArtifactViewer>::DIALECT, DAG_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_the_main_body_key_for_the_default_snapshot() {
        let snapshot = default_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = NoConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let _node = <DagViewer as ArtifactViewer>::render(main::BODY_KEY, &doc, &cfg);
    }
}
//#endregion 🧪️Tests
