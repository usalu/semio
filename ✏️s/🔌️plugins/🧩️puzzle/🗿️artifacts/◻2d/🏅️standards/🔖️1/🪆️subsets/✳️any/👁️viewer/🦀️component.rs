//! 👁️ Puzzle 2d viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Puzzle2dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Puzzle2dViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::puzzle2d::{Puzzle2dSnapshot, PUZZLE2D_DIALECT, PUZZLE_2D_SCHEMA};
use crate::viewer::puzzle2d::modes::view;
use crate::viewer::puzzle2d::modes::view::windows::board;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
// 🕹️ `InteractionView` — see `✏️editor/🦀️component.rs`'s identical import comment (missing top-level
// re-export from `semio_framework_plugin`, flagged to the coordinator, not fixed here).
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
/// `#[derive(Default)]` is required by `testkit::assert_viewer_never_mutates::<V>() where V::Command:
/// Default` (contract §2.5).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Puzzle2dViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Puzzle2dViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Puzzle2dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Puzzle2dViewer;

impl ArtifactViewer for Puzzle2dViewer {
    type Snapshot = Puzzle2dSnapshot;
    type Mutation = crate::artifacts::puzzle2d::op::Puzzle2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Puzzle2dViewCommand;

    const DIALECT: Dialect = PUZZLE2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE_2D_SCHEMA;

    async fn initial_snapshot() -> Puzzle2dSnapshot {
        Puzzle2dSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `Puzzle2dViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// pan/zoom) is a pure addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let node = match body_key {
            board::BODY_KEY => board::render(doc.snapshot)?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}")))
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d viewer unknown-body label admission failed"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_puzzle2d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(PUZZLE2D_DIALECT)
        .document(["semio", "puzzle", "2d"])
        .icon_id("puzzle")
        .mode_def(view::definition())
        .default_mode_id(view::PUZZLE2D_VIEW_MODE_VIEW)
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
    fn create_puzzle2d_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_puzzle2d_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, PUZZLE2D_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Puzzle2dViewer as ArtifactViewer>::DIALECT, PUZZLE2D_DIALECT);
    }

    #[test]
    fn viewer_command_default_is_noop() {
        assert_eq!(Puzzle2dViewCommand::default(), Puzzle2dViewCommand::Noop);
    }
}
//#endregion 🧪️Tests
