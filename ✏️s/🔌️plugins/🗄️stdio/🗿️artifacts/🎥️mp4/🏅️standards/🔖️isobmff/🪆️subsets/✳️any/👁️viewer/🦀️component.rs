//! 👁️ `mp4` viewer (any) — the read-only counterpart of `✏️editor` for this
//! subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Mp4Viewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Mp4Viewer>` is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling `editor` module (`policyViewerPurityBreaches`).

use crate::artifacts::mp4::{MP4_DIALECT, STDIO_MP4_DOCUMENT_SCHEMA};
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::mutations::Mp4Mutation;
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
use crate::viewer::mp4::modes::view;
use crate::viewer::mp4::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mp4ViewCommand {
    Noop,
}

impl protocol::OpBinary for Mp4ViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Mp4ViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Mp4Viewer;

impl ArtifactViewer for Mp4Viewer {
    type Snapshot = Mp4Snapshot;
    type Mutation = Mp4Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Mp4ViewCommand;

    const DIALECT: Dialect = MP4_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_MP4_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> Self::Snapshot {
        Mp4Snapshot::default()
    }

    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &semio_framework_plugin::app::InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
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
pub async fn create_mp4_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(MP4_DIALECT)
        .document(["semio", "mp4"])
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
    async fn create_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_mp4_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, MP4_DIALECT.into());
    }

    #[test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Mp4Viewer as ArtifactViewer>::DIALECT, MP4_DIALECT);
    }
}
//#endregion 🧪️Tests
