//! ✏️ `avi` editor (any) — `ArtifactEditor` surface built on the frozen
//! `MediaWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! `seek-media` is declared (the frozen `MediaWindowKit` editable action) but intentionally performs no document mutation — playback position is host-side ephemeral transport state, not persisted document content this format's schema models.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::artifacts::avi::{AVI_DIALECT, STDIO_AVI_DOCUMENT_SCHEMA};
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::AviMutation;
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use crate::editor::avi::modes::edit;
use crate::editor::avi::modes::edit::windows::main;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AviEditCommand {
    SeekMedia { position_ms: u64 },
}

impl protocol::OpBinary for AviEditCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Malformed { what: "avi-edit-command", offset: 0, detail: error.to_string() })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "avi-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct AviEditor;

impl ArtifactEditor for AviEditor {
    type Snapshot = AviSnapshot;
    type Mutation = AviMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = AviEditCommand;

    const DIALECT: Dialect = AVI_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_AVI_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> Self::Snapshot {
        AviSnapshot::default()
    }

    async fn handle(
        command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            AviEditCommand::SeekMedia { position_ms: _ } => Ok(Emit::default()),
        }
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_avi_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(AVI_DIALECT)
        .document(["semio", "avi"])
        .icon_id("play")
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

    #[semio_framework_async_macros::async_test]
    async fn create_editor_builds_a_definition_for_the_editor_role() {
        let def = create_avi_editor();
        assert_eq!(def.role, semio_framework::AppRole::Editor);
        assert_eq!(def.dialect, AVI_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<AviEditor as ArtifactEditor>::DIALECT, AVI_DIALECT);
    }
}
//#endregion 🧪️Tests
