//! 👁️ Puzzle 3d viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Puzzle3dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Puzzle3dViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::puzzle3d::{Puzzle3dMutation, Puzzle3dSnapshot, PUZZLE3D_DIALECT, PUZZLE_3D_SCHEMA};
use crate::viewer::puzzle3d::modes::view;
use crate::viewer::puzzle3d::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
/// `Default` is required by `testkit::assert_viewer_never_mutates::<V>() where V::Command: Default`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Puzzle3dViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Puzzle3dViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Puzzle3dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Puzzle3dViewer;

impl ArtifactViewer for Puzzle3dViewer {
    type Snapshot = Puzzle3dSnapshot;
    type Mutation = Puzzle3dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Puzzle3dViewCommand;

    const DIALECT: Dialect = PUZZLE3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE_3D_SCHEMA;

    async fn initial_snapshot() -> Puzzle3dSnapshot {
        Puzzle3dSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `Puzzle3dViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// orbit, "focus object") is a pure addition here, never a signature change.
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
pub async fn create_puzzle3d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(PUZZLE3D_DIALECT)
        .document(["semio", "puzzle", "3d"])
        .icon_id("puzzle")
        .mode_def(view::definition())
        .default_mode_id(view::PUZZLE3D_VIEW_MODE_VIEW)
        .window_kind_def(main::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn create_puzzle3d_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_puzzle3d_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, PUZZLE3D_DIALECT.into());
    }

    #[test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Puzzle3dViewer as ArtifactViewer>::DIALECT, PUZZLE3D_DIALECT);
    }
}
//#endregion 🧪️Tests
