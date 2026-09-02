//! 👁️ Raster viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `RasterViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<RasterViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::raster::{RasterSnapshot, RASTER_DIALECT, RASTER_DOCUMENT_SCHEMA};
use crate::viewer::raster::modes::view;
use crate::viewer::raster::modes::view::windows::{composite, navigator};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*` carries
/// them would be pure ceremony for a surface that never dispatches anything through `handle`. Derives
/// `Default` (`#[default]` on the sole variant) — required by `testkit::assert_viewer_never_mutates`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RasterViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for RasterViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(RasterViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct RasterViewer;

impl ArtifactViewer for RasterViewer {
    type Snapshot = RasterSnapshot;
    type Mutation = crate::artifacts::raster::op::RasterMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = RasterViewCommand;

    const DIALECT: Dialect = RASTER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = RASTER_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> RasterSnapshot {
        crate::artifacts::raster::schema::empty_raster_document()
    }

    /// 👁️ Structurally read-only: the sole `RasterViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action is a pure
    /// addition here, never a signature change.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            composite::RASTER_VIEW_BODY_COMPOSITE => composite::render(doc.snapshot),
            navigator::RASTER_VIEW_BODY_NAVIGATOR => navigator::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_raster_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(RASTER_DIALECT)
        .document(["semio", "raster"])
        .icon_id("raster")
        .mode_def(view::definition())
        .default_mode_id(view::RASTER_VIEW_MODE_VIEW)
        .window_kind_def(composite::definition())
        .window_kind_def(navigator::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_raster_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_raster_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, RASTER_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<RasterViewer as ArtifactViewer>::DIALECT, RASTER_DIALECT);
    }
}
//#endregion 🧪️Tests
