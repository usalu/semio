//! 👁️ Draw viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `DrawViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<DrawViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::draw::schema::default_draw_document;
use crate::artifacts::draw::{DrawSnapshot, DRAW_DIALECT, DRAW_DOCUMENT_SCHEMA};
use crate::viewer::draw::modes::view;
use crate::viewer::draw::modes::view::windows::canvas;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
/// `Default` is required by `testkit::assert_viewer_never_mutates::<DrawViewer>()`'s `V::Command:
/// Default` bound (contract §2.5) — the single variant is trivially its own default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for DrawViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(DrawViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct DrawViewer;

impl ArtifactViewer for DrawViewer {
    type Snapshot = DrawSnapshot;
    type Mutation = crate::artifacts::draw::op::DrawMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = DrawViewCommand;

    const DIALECT: semio_framework::Dialect = DRAW_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DRAW_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> DrawSnapshot {
        default_draw_document("empty", None)
    }

    /// 👁️ Structurally read-only: the sole `DrawViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// pan/zoom) is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            canvas::BODY_KEY => canvas::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_draw_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(DRAW_DIALECT)
        .document(["semio", "draw"])
        .icon_id("draw")
        .mode_def(view::definition())
        .default_mode_id(view::DRAW_VIEW_MODE_VIEW)
        .window_kind_def(canvas::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_draw_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_draw_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, DRAW_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<DrawViewer as ArtifactViewer>::DIALECT, DRAW_DIALECT);
    }
}
//#endregion 🧪️Tests
