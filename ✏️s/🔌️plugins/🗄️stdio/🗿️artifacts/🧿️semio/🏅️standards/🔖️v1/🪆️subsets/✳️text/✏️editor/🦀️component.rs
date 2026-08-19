//! ✏️ Semio Text editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `SemioTextEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::editor::semio_text::modes::edit;
use crate::editor::semio_text::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation,
    StandardId, SubsetId, UiNode,
};
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;

//#region 🔖️Dialect
pub const SEMIO_TEXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("text") };
pub const SEMIO_TEXT_DOCUMENT_SCHEMA: &str = "s.stdio.semio.text";
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The Main window declares the shared `MeshWindowKit::editable_window_kind()`'s
/// `set-vertex` action (contract §2.6), but this subset's own `🧬️schema/🧬️mutations` declares
/// no by-index "replace"/"set" op that action could honestly back today (only insert/remove and
/// whole-document `SetSnapshot`) — per this ticket's explicit allowance, the editor still exists
/// with a MINIMAL command set: the window really advertises the action, `handle` is a real dispatch
/// (not `unreachable!()`) that is a no-op today, rather than inventing a mutation the schema does
/// not have. Report, don't invent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SemioTextEditCommand {
    #[default]
    SetVertex,
}

impl protocol::OpBinary for SemioTextEditCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(SemioTextEditCommand::SetVertex)
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct SemioTextEditor;

impl ArtifactEditor for SemioTextEditor {
    type Snapshot = SemioTextSnapshot;
    type Mutation = SemioTextMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SemioTextEditCommand;

    const DIALECT: Dialect = SEMIO_TEXT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SEMIO_TEXT_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> SemioTextSnapshot {
        SemioTextSnapshot::default()
    }

    async fn handle(command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        let _ = command;
        Ok(Emit::default())
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
pub async fn create_semio_text_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(SEMIO_TEXT_DIALECT)
        .document(["stdio", "semio"])
        .icon_id("box")
        .mode_def(edit::definition())
        .default_mode_id(edit::SEMIO_TEXT_EDIT_MODE_ID)
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
    async fn create_editor_builds_a_definition_for_the_editor_role() {
        let def = create_semio_text_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, SEMIO_TEXT_DIALECT.into());
    }

    #[test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<SemioTextEditor as ArtifactEditor>::DIALECT, SEMIO_TEXT_DIALECT);
    }

    #[test]
    async fn editor_and_viewer_share_one_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<SemioTextEditor, crate::viewer::semio_text::SemioTextViewer>();
    }
}
//#endregion 🧪️Tests
