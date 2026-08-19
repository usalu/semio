//! ✏️ `md` editor (any) — `ArtifactEditor` surface built on the frozen
//! `TextWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Emits the frozen `replace-text` action: the incoming text is the artifact's own DSL text envelope (`print_dsl`/`parse_dsl`), round-tripped into a whole-document `SetSnapshot`.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::artifacts::md::{MD_DIALECT, STDIO_MD_DOCUMENT_SCHEMA};
use crate::artifacts::md::standards::v_commonmark::subsets::any::schema::mutations::MdMutation;
use crate::artifacts::md::standards::v_commonmark::subsets::any::schema::snapshot::MdSnapshot;
use crate::editor::md::modes::edit;
use crate::editor::md::modes::edit::windows::main;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MdEditCommand {
    ReplaceText { text: String },
}

impl protocol::OpBinary for MdEditCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Malformed { what: "md-edit-command", offset: 0, detail: error.to_string() })
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "md-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct MdEditor;

impl ArtifactEditor for MdEditor {
    type Snapshot = MdSnapshot;
    type Mutation = MdMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = MdEditCommand;

    const DIALECT: Dialect = MD_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_MD_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> Self::Snapshot {
        MdSnapshot::default()
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
            MdEditCommand::ReplaceText { text } => match <MdSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                Ok(snapshot) => Ok(Emit::mutations(vec![MdMutation::SetSnapshot { snapshot }])),
                Err(_) => Ok(Emit::default()),
            },
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
pub async fn create_md_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(MD_DIALECT)
        .document(["semio", "md"])
        .icon_id("file-text")
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
        let def = create_md_editor();
        assert_eq!(def.role, semio_framework::AppRole::Editor);
        assert_eq!(def.dialect, MD_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<MdEditor as ArtifactEditor>::DIALECT, MD_DIALECT);
    }
}
//#endregion 🧪️Tests
