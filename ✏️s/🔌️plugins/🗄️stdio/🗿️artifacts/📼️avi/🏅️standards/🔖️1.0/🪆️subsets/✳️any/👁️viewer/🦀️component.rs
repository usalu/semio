//! 👁️ `avi` viewer (any) — the read-only counterpart of `✏️editor` for this
//! subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `AviViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<AviViewer>` is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling `editor` module (`policyViewerPurityBreaches`).

use crate::artifacts::avi::{AVI_DIALECT, STDIO_AVI_DOCUMENT_SCHEMA};
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::AviMutation;
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use crate::viewer::avi::modes::view;
use crate::viewer::avi::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AviViewCommand {
    Noop,
}

impl protocol::OpBinary for AviViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(AviViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct AviViewer;

impl ArtifactViewer for AviViewer {
    type Snapshot = AviSnapshot;
    type Mutation = AviMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = AviViewCommand;

    const DIALECT: Dialect = AVI_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_AVI_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Self::Snapshot {
        AviSnapshot::default()
    }

    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &semio_framework_plugin::app::InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_avi_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(AVI_DIALECT)
        .document(["semio", "avi"])
        .icon_id("play")
        .mode_def(view::definition())
        .default_mode_id(view::MODE_ID)
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
    fn create_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_avi_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, AVI_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<AviViewer as ArtifactViewer>::DIALECT, AVI_DIALECT);
    }
}
//#endregion 🧪️Tests
