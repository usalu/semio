//! 👁️ Block 2D viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Block2dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Block2dViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::block2d::{schema, Block2dSnapshot, BLOCK2D_DIALECT, BLOCK_2D_SCHEMA};
use crate::viewer::block2d::modes::view;
use crate::viewer::block2d::modes::view::windows::board;
use semio_framework_plugin::{
    ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer,
};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert, `Default`-deriving variant — real per-command payload modules the way
/// `✏️editor/🎮️commands/*` carries them would be pure ceremony for a surface that never dispatches
/// anything through `handle`. `Default` is required by `testkit::assert_viewer_never_mutates`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Block2dViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Block2dViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Block2dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Block2dViewer;

impl ArtifactViewer for Block2dViewer {
    type Snapshot = Block2dSnapshot;
    type Mutation = crate::artifacts::block2d::op::Block2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Block2dViewCommand;

    const DIALECT: Dialect = BLOCK2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_2D_SCHEMA;

    async fn initial_snapshot() -> Block2dSnapshot {
        schema::empty_block2d_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `Block2dViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action is a pure
    /// addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &semio_framework_plugin::app::InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            board::BODY_KEY => board::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_block2d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(BLOCK2D_DIALECT)
        .document(["semio", "block", "2d"])
        .icon_id("layout-grid")
        .mode_def(view::definition())
        .default_mode_id(view::BLOCK2D_VIEW_MODE_VIEW)
        .window_kind_def(board::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn create_block2d_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_block2d_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, BLOCK2D_DIALECT.into());
    }

    #[test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Block2dViewer as ArtifactViewer>::DIALECT, BLOCK2D_DIALECT);
    }
}
//#endregion 🧪️Tests
