//! ✏️ `html` editor (any) — `ArtifactEditor` surface built on the frozen
//! `TextWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Emits the frozen `replace-text` action: the incoming text is the artifact's own DSL text envelope (`print_dsl`/`parse_dsl`), round-tripped into a whole-document `SetSnapshot`.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::artifacts::html::{HTML_DIALECT, STDIO_HTML_DOCUMENT_SCHEMA};
use crate::artifacts::html::standards::v5::subsets::any::schema::mutations::HtmlMutation;
use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
use crate::editor::html::modes::edit;
use crate::editor::html::modes::edit::windows::main;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HtmlEditCommand {
    ReplaceText { text: String },
}

impl protocol::OpBinary for HtmlEditCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Malformed { what: "html-edit-command", offset: 0, detail: error.to_string() })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "html-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct HtmlEditor;

impl ArtifactEditor for HtmlEditor {
    type Snapshot = HtmlSnapshot;
    type Mutation = HtmlMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = HtmlEditCommand;

    const DIALECT: Dialect = HTML_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_HTML_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Self::Snapshot {
        HtmlSnapshot::default()
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
            HtmlEditCommand::ReplaceText { text } => match <HtmlSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                Ok(snapshot) => Ok(Emit::mutations(vec![HtmlMutation::SetSnapshot { snapshot }])),
                Err(_) => Ok(Emit::default()),
            },
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
pub fn create_html_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(HTML_DIALECT)
        .document(["semio", "html"])
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

    #[test]
    fn create_editor_builds_a_definition_for_the_editor_role() {
        let def = create_html_editor();
        assert_eq!(def.role, semio_framework::AppRole::Editor);
        assert_eq!(def.dialect, HTML_DIALECT.into());
    }

    #[test]
    fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<HtmlEditor as ArtifactEditor>::DIALECT, HTML_DIALECT);
    }
}
//#endregion 🧪️Tests
