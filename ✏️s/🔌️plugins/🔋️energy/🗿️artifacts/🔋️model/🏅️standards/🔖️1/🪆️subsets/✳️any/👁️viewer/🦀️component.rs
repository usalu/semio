//! 👁️ Energy model viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `EnergyModelViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<EnergyModelViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact mutation.
//! Must not import anything from the sibling mutation-capable surface (`policyViewerPurityBreaches`).

use crate::artifacts::model::{EnergyModelMutation, EnergyModelSnapshot, ENERGY_MODEL_DOCUMENT_SCHEMA, MODEL_DIALECT};
use crate::viewer::model::modes::view;
use crate::viewer::model::modes::view::windows::{structure, zones};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor`'s typed command
/// enum carries them would be pure ceremony for a surface that never dispatches anything through
/// `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EnergyModelViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for EnergyModelViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(EnergyModelViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct EnergyModelViewer;

impl ArtifactViewer for EnergyModelViewer {
    type Snapshot = EnergyModelSnapshot;
    type Mutation = EnergyModelMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = EnergyModelViewCommand;

    const DIALECT: Dialect = MODEL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = ENERGY_MODEL_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> EnergyModelSnapshot {
        EnergyModelSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `EnergyModelViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit`. Kept as a real dispatch (not
    /// `unreachable!()`) so a future view-only action is a pure addition, never a signature change.
    async fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _engines: &store::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            structure::BODY_KEY => structure::render(doc.snapshot),
            zones::BODY_KEY => zones::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_energy_model_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(MODEL_DIALECT)
        .document(["semio", "energy", "model"])
        .icon_id("battery")
        .mode_def(view::definition())
        .default_mode_id(view::ENERGY_MODEL_VIEW_MODE_ID)
        .window_kind_def(structure::definition())
        .window_kind_def(zones::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_energy_model_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_energy_model_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, MODEL_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<EnergyModelViewer as ArtifactViewer>::DIALECT, MODEL_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_declares_both_windows() {
        let def = create_energy_model_viewer();
        assert!(def.window_kinds.iter().any(|w| w.id == structure::WINDOW_KIND_ID));
        assert!(def.window_kinds.iter().any(|w| w.id == zones::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
