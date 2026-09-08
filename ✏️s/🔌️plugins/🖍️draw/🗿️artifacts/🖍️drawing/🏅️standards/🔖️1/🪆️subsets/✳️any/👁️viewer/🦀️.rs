//! 👁️ Drawing viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `DrawingViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<DrawingViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::schema::default_drawing_document;
use crate::{DrawingSnapshot, DRAWING_DIALECT, DRAWING_DOCUMENT_SCHEMA};
use crate::viewer::drawing::modes::view;
use crate::viewer::drawing::modes::view::windows::canvas;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
/// `Default` is required by `testkit::assert_viewer_never_mutates::<DrawingViewer>()`'s `V::Command:
/// Default` bound (contract §2.5) — the single variant is trivially its own default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawingViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for DrawingViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(DrawingViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct DrawingViewer;

impl ArtifactViewer for DrawingViewer {
    type Snapshot = DrawingSnapshot;
    type Mutation = crate::op::DrawingMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = DrawingViewCommand;

    const DIALECT: semio_framework::Dialect = DRAWING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> DrawingSnapshot {
        default_drawing_document("empty", None)
    }

    /// 👁️ Structurally read-only: the sole `DrawingViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// pan/zoom) is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let root = match body_key {
            canvas::BODY_KEY => canvas::render(doc.snapshot),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}")))
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("drawing.viewer.body.label", "the fixed Drawing viewer label exceeds its UI bound")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(root))
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_drawing_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(DRAWING_DIALECT).document(["semio", "drawing"]).icon_id("drawing").mode_def(view::definition()).default_mode_id(view::DRAWING_VIEW_MODE_VIEW).window_kind_def(canvas::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
