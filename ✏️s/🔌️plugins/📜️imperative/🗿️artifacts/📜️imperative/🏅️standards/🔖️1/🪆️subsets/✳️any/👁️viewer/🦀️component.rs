//! 👁️ Imperative viewer — the read-only counterpart of the sibling editor surface for this subset
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `ImperativeViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<ImperativeViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! or draft mutation. MUST NOT import anything from the sibling editor module
//! (`policyViewerPurityBreaches` forbids it outright, including the substring in comments).

use crate::artifacts::imperative::schema::default_snapshot;
use crate::artifacts::imperative::{ImperativeSnapshot, IMPERATIVE_DIALECT, IMPERATIVE_DOCUMENT_SCHEMA};
use crate::viewer::imperative::modes::view;
use crate::viewer::imperative::modes::view::windows::{main, script};
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode};
// 🚧️ SDK GAP: `ArtifactViewer`/`Viewer`/`ViewEmit`/`Dialect` were closed by w0-f (bare-importable
// from the crate root now); `Dialect`/`StandardId`/`SubsetId` and the window-kit types (contract
// §2.6) are still only reachable through `app` — see the identical note in `✏️editor`'s own file.
use semio_framework_plugin::app::{ArtifactViewer, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor`'s
/// `🎮️commands/*` carries them would be pure ceremony for a surface that never dispatches anything
/// through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ImperativeViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for ImperativeViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(ImperativeViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct ImperativeViewer;

impl ArtifactViewer for ImperativeViewer {
    type Snapshot = ImperativeSnapshot;
    type Mutation = crate::artifacts::imperative::mutations::ImperativeMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = ImperativeViewCommand;

    const DIALECT: semio_framework_plugin::app::Dialect = IMPERATIVE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = IMPERATIVE_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> ImperativeSnapshot {
        default_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `ImperativeViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no
    /// dirty scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action
    /// (e.g. "jump to step") is a pure addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &semio_framework_plugin::app::InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            script::BODY_KEY => script::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_imperative_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(IMPERATIVE_DIALECT)
        .document(["semio", "imperative"])
        .icon_id("imperative")
        .mode_def(view::definition())
        .default_mode_id(view::IMPERATIVE_VIEW_MODE_VIEW)
        .window_kind_def(main::definition())
        .window_kind_def(script::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn create_imperative_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_imperative_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, IMPERATIVE_DIALECT.into());
    }

    #[test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<ImperativeViewer as ArtifactViewer>::DIALECT, IMPERATIVE_DIALECT);
    }
}
//#endregion 🧪️Tests
