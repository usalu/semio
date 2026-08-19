//! ✏️ IFC 2x3 Cobie editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `Ifc2x3CobieEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::artifacts::ifc::standards::v2x3::subsets::cobie::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::ifc::standards::v2x3::subsets::cobie::schema::mutations::Ifc2x3Mutation;
use crate::editor::ifc2x3_cobie::modes::edit;
use crate::editor::ifc2x3_cobie::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation,
    StandardId, SubsetId, UiNode,
};
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;

//#region 🔖️Dialect
pub const IFC2X3_COBIE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("cobie") };
pub const IFC2X3_COBIE_DOCUMENT_SCHEMA: &str = "stdio.ifc.2x3";
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
pub enum Ifc2x3CobieEditCommand {
    #[default]
    SetVertex,
}

impl protocol::OpBinary for Ifc2x3CobieEditCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Ifc2x3CobieEditCommand::SetVertex)
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Ifc2x3CobieEditor;

impl ArtifactEditor for Ifc2x3CobieEditor {
    type Snapshot = Ifc2x3Snapshot;
    type Mutation = Ifc2x3Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Ifc2x3CobieEditCommand;

    const DIALECT: Dialect = IFC2X3_COBIE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = IFC2X3_COBIE_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> Ifc2x3Snapshot {
        Ifc2x3Snapshot::default()
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
pub async fn create_ifc2x3_cobie_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(IFC2X3_COBIE_DIALECT)
        .document(["stdio", "ifc2x3"])
        .icon_id("box")
        .mode_def(edit::definition())
        .default_mode_id(edit::IFC2X3_COBIE_EDIT_MODE_ID)
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
        let def = create_ifc2x3_cobie_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, IFC2X3_COBIE_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Ifc2x3CobieEditor as ArtifactEditor>::DIALECT, IFC2X3_COBIE_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_and_viewer_share_one_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<Ifc2x3CobieEditor, crate::viewer::ifc2x3_cobie::Ifc2x3CobieViewer>();
    }
}
//#endregion 🧪️Tests
