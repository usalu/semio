//! 👁️ Puzzle 5D viewer — the read-only counterpart of the sibling editor module for this subset
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Puzzle5dViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Puzzle5dViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! or draft mutation. MUST NOT import anything from the sibling editor module
//! (`policyViewerPurityBreaches`).
//!
//! 🧬️ `Snapshot`/`Mutation` are the real ARTIFACT-level types (`Puzzle5dSnapshot`/`Puzzle5dMutation`,
//! shared with the editor per contract §2.2's decode-only rule) — NOT the editor's own
//! `Puzzle5dPlaySnapshot` newtype (a `serde_json::Value` wrapper the play app's command layer
//! mutates); that distinction is this artifact's own pre-existing quirk, not introduced here.

use crate::artifacts::puzzle5d::{Puzzle5dMutation, Puzzle5dSnapshot, PUZZLE5D_DIALECT, PUZZLE_5D_SCHEMA};
use crate::viewer::puzzle5d::modes::view;
use crate::viewer::puzzle5d::modes::view::windows::world3d;
use semio_framework_plugin::app::Dialect;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way the sibling editor's
/// `🎮️commands/*` carries them would be pure ceremony for a surface that never dispatches anything
/// through `handle`.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle5dViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Puzzle5dViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Puzzle5dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Puzzle5dViewer;

impl ArtifactViewer for Puzzle5dViewer {
    type Snapshot = Puzzle5dSnapshot;
    type Mutation = Puzzle5dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Puzzle5dViewCommand;

    const DIALECT: Dialect = PUZZLE5D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE_5D_SCHEMA;

    fn initial_snapshot() -> Puzzle5dSnapshot {
        Puzzle5dSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `Puzzle5dViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// orbit, "jump to part") is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &semio_framework_plugin::app::InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            world3d::BODY_KEY => world3d::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_puzzle5d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(PUZZLE5D_DIALECT)
        .document(["semio", "puzzle", "5d"])
        .icon_id("puzzle")
        .mode_def(view::definition())
        .default_mode_id(view::PUZZLE5D_VIEW_MODE_VIEW)
        .window_kind_def(world3d::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_puzzle5d_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_puzzle5d_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, PUZZLE5D_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Puzzle5dViewer as ArtifactViewer>::DIALECT, PUZZLE5D_DIALECT);
    }
}
//#endregion 🧪️Tests
