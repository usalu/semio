//! 👁️ Block 5D viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Block5dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Block5dViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::block5d::{Block5dSnapshot, BLOCK5D_DIALECT, BLOCK_5D_SCHEMA};
use crate::viewer::block5d::modes::view;
use crate::viewer::block5d::modes::view::windows::world;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode};
// 🚧️ SDK GAP: `ArtifactViewer`/`Viewer`/`ViewEmit` are in the crate-root re-export list now
// (ticket 26/08/16 W0-F closed that gap), but `Dialect` itself is not — only reachable through
// `app`. Flagged in this packet's migration report, not fixable here (`🧰️framework/**` is
// outside this packet's lease).
use semio_framework_plugin::app::Dialect;
use semio_framework_plugin::{ArtifactViewer, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Block5dViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Block5dViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Block5dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Block5dViewer;

impl ArtifactViewer for Block5dViewer {
    type Snapshot = Block5dSnapshot;
    type Mutation = crate::artifacts::block5d::op::Block5dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Block5dViewCommand;

    const DIALECT: Dialect = BLOCK5D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_5D_SCHEMA;

    async fn initial_snapshot() -> Block5dSnapshot {
        crate::artifacts::block5d::schema::empty_block5d_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `Block5dViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// orbit, "jump to representation") is a pure addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &semio_framework_plugin::app::InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            world::BODY_KEY => world::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_block5d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(BLOCK5D_DIALECT)
        .document(["semio", "block", "5d"])
        .icon_id("layers")
        .mode_def(view::definition())
        .default_mode_id(view::BLOCK5D_VIEW_MODE_VIEW)
        .window_kind_def(world::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn create_block5d_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_block5d_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, BLOCK5D_DIALECT.into());
    }

    #[test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Block5dViewer as ArtifactViewer>::DIALECT, BLOCK5D_DIALECT);
    }
}
//#endregion 🧪️Tests
