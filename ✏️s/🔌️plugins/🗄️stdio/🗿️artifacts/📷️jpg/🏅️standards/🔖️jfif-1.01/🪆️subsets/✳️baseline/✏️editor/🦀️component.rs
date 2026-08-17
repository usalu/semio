//! ✏️ `jpg` editor (baseline) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Emits the frozen `set-pixel-region` action onto the artifact's own whole-raster replace mutation.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::artifacts::jpg::{JPG_BASELINE_DIALECT, STDIO_JPG_DOCUMENT_SCHEMA};
use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::JpgMutation;
use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::snapshot::JpgSnapshot;
use crate::editor::jpg_baseline::modes::edit;
use crate::editor::jpg_baseline::modes::edit::windows::main;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JpgBaselineEditCommand {
    SetPixelRegion { pixels: Vec<u8> },
}

impl protocol::OpBinary for JpgBaselineEditCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Malformed { what: "jpg_baseline-edit-command", offset: 0, detail: error.to_string() })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "jpg_baseline-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct JpgBaselineEditor;

impl ArtifactEditor for JpgBaselineEditor {
    type Snapshot = JpgSnapshot;
    type Mutation = JpgMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = JpgBaselineEditCommand;

    const DIALECT: Dialect = JPG_BASELINE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JPG_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Self::Snapshot {
        JpgSnapshot::default()
    }

    fn handle(
        command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            JpgBaselineEditCommand::SetPixelRegion { pixels } => Ok(Emit::mutations(vec![JpgMutation::SetPixels { pixels: pixels.clone() }])),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
pub fn create_jpg_baseline_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(JPG_BASELINE_DIALECT)
        .document(["semio", "jpg"])
        .icon_id("image")
        .mode_def(edit::definition())
        .default_mode_id(edit::MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(edit::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_editor_builds_a_definition_for_the_editor_role() {
        let def = create_jpg_baseline_editor();
        assert_eq!(def.role, semio_framework::AppRole::Editor);
        assert_eq!(def.dialect, JPG_BASELINE_DIALECT.into());
    }

    #[test]
    fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<JpgBaselineEditor as ArtifactEditor>::DIALECT, JPG_BASELINE_DIALECT);
    }
}
//#endregion 🧪️Tests
