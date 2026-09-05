//! 👁️ Block 3D viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Block3dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Block3dViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::block3d::{Block3dSnapshot, BLOCK3D_DIALECT, BLOCK_3D_SCHEMA};
use crate::viewer::block3d::modes::view;
use crate::viewer::block3d::modes::view::windows::world;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
// 🚧️ SDK GAP: see the identical note in `✏️editor/🦀️.rs` — `Dialect`/`InteractionView` are
// only reachable through `app`, not yet in the crate-root re-export list (`ArtifactViewer`/`Viewer`/
// `ViewEmit` are — closed by W0-F).
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::{ArtifactViewer, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Block3dViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Block3dViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Block3dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Block3dViewer;

impl ArtifactViewer for Block3dViewer {
    type Snapshot = Block3dSnapshot;
    type Mutation = crate::artifacts::block3d::op::Block3dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Block3dViewCommand;

    const DIALECT: Dialect = BLOCK3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_3D_SCHEMA;

    /// 🚀️ Boots on the `hexagonal-cut-concrete-forest-left` fixture rather than the empty document —
    /// a viewer has no `setActiveExample` action at all (its sole command is `Noop`), so an empty boot
    /// document made this surface permanently blank. See `dsl::block3d_boot_snapshot`.
    fn initial_snapshot() -> Block3dSnapshot {
        crate::artifacts::block3d::dsl::block3d_boot_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `Block3dViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// orbit, "jump to representation") is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let node = match body_key {
            world::BODY_KEY => world::render(doc.snapshot)?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "block3d viewer unknown-body label admission failed"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_block3d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(BLOCK3D_DIALECT).icon_id("box").mode_def(view::definition()).default_mode_id(view::BLOCK3D_VIEW_MODE_VIEW).window_kind_def(world::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_block3d_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_block3d_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, BLOCK3D_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Block3dViewer as ArtifactViewer>::DIALECT, BLOCK3D_DIALECT);
    }

    /// ⚖️ LAW: the viewer boots non-empty — it can never load an example itself, so an empty initial
    /// snapshot would leave its `World3d` window blank forever.
    #[semio_framework_async_macros::async_test]
    async fn viewer_boots_with_at_least_one_representation() {
        let mut app = semio_framework_plugin::testkit::new_app::<semio_framework_plugin::ViewerApp<Block3dViewer>>();
        let snapshot = app.snapshot().expect("snapshot");
        assert!(!snapshot.representations.is_empty(), "the viewer must boot with a renderable document");
        assert!(snapshot.representations.iter().all(|representation| representation.mesh_url.is_some()));
    }

    #[semio_framework_async_macros::async_test]
    async fn noop_command_round_trips_and_never_mutates() {
        let mut app = semio_framework_plugin::testkit::new_app::<semio_framework_plugin::ViewerApp<Block3dViewer>>();
        let before = app.snapshot().expect("snapshot");
        app.dispatch_typed(Block3dViewCommand::Noop, &semio_framework_plugin::testkit::meta("local")).expect("dispatch");
        let after = app.snapshot().expect("snapshot");
        assert_eq!(before, after, "the viewer's sole command must never change the document");
    }
}
//#endregion 🧪️Tests
