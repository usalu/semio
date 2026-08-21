//! ✏️ Semio Image editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `SemioImageEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use crate::editor::semio_image::modes::edit;
use crate::editor::semio_image::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation,
    StandardId, SubsetId, UiNode,
};
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;

//#region 🔖️Dialect
pub const SEMIO_IMAGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
pub const SEMIO_IMAGE_DOCUMENT_SCHEMA: &str = "s.stdio.semio.image";
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
pub enum SemioImageEditCommand {
    #[default]
    SetVertex,
}

impl protocol::OpBinary for SemioImageEditCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(SemioImageEditCommand::SetVertex)
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct SemioImageEditor;

impl ArtifactEditor for SemioImageEditor {
    type Snapshot = SemioImageSnapshot;
    type Mutation = SemioImageMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SemioImageEditCommand;

    const DIALECT: Dialect = SEMIO_IMAGE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SEMIO_IMAGE_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> SemioImageSnapshot {
        SemioImageSnapshot::default()
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_semio_image_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(SEMIO_IMAGE_DIALECT)
        .document(["stdio", "semio"])
        .icon_id("box")
        .mode_def(edit::definition())
        .default_mode_id(edit::SEMIO_IMAGE_EDIT_MODE_ID)
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
        let def = create_semio_image_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, SEMIO_IMAGE_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<SemioImageEditor as ArtifactEditor>::DIALECT, SEMIO_IMAGE_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_and_viewer_share_one_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<SemioImageEditor, crate::viewer::semio_image::SemioImageViewer>();
    }
}
//#endregion 🧪️Tests
