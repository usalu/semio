//! 👁️ Playbook viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `PlaybookViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<PlaybookViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::playbook::{PlaybookSnapshot, PLAYBOOK_DIALECT, PLAYBOOK_DOCUMENT_SCHEMA};
use crate::viewer::playbook::modes::view;
use crate::viewer::playbook::modes::view::windows::steps;
use semio_framework_plugin::{ArtifactView, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
// 🚧️ SDK GAP: `ArtifactViewer` is reachable bare (w0-f gap 1 closed it), but this trait's own
// `InteractionView` parameter type is NOT in the crate-root curated re-export list yet — only
// reachable through `app`. Flagged for the coordinator (see this packet's notes file).
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PlaybookViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for PlaybookViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(PlaybookViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct PlaybookViewer;

impl semio_framework_plugin::ArtifactViewer for PlaybookViewer {
    type Snapshot = PlaybookSnapshot;
    type Mutation = crate::artifacts::playbook::op::PlaybookMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = PlaybookViewCommand;

    const DIALECT: Dialect = PLAYBOOK_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PLAYBOOK_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> PlaybookSnapshot {
        crate::artifacts::playbook::empty_playbook_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `PlaybookViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (e.g.
    /// "jump to step") is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            steps::PLAYBOOK_VIEW_BODY_STEPS => steps::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_playbook_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(PLAYBOOK_DIALECT)
        .document(["semio", "playbook"])
        .icon_id("eye")
        .mode_def(view::definition())
        .default_mode_id(view::PLAYBOOK_VIEW_MODE_VIEW)
        .window_kind_def(steps::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::ArtifactViewer;

    #[test]
    fn create_playbook_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_playbook_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, PLAYBOOK_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<PlaybookViewer as ArtifactViewer>::DIALECT, PLAYBOOK_DIALECT);
    }
}
//#endregion 🧪️Tests
