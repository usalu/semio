//! ✏️ `gif` editor (any) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Emits the frozen `set-pixel-region` action onto the artifact's own frame/image pixel-index replace mutation (index fixed at 0 — a genuine per-region patch is not declared in this format's schema).
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::artifacts::gif::standards::v87a::subsets::any::schema::mutations::{set_image_pixels, GifMutation};
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;
use crate::artifacts::gif::{GIF_87A_DIALECT, STDIO_GIF_DOCUMENT_SCHEMA};
use crate::editor::gif_87a::modes::edit;
use crate::editor::gif_87a::modes::edit::windows::main;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use store::EngineHandles;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Gif87aEditCommand {
    SetPixelRegion { indices: Vec<u8> },
}

impl protocol::OpBinary for Gif87aEditCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Malformed { what: "gif_87a-edit-command", offset: 0, detail: error.to_string() })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "gif_87a-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Gif87aEditor;

impl ArtifactEditor for Gif87aEditor {
    type Snapshot = GifSnapshot;
    type Mutation = GifMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Gif87aEditCommand;

    const DIALECT: Dialect = GIF_87A_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_GIF_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Self::Snapshot {
        GifSnapshot::default()
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
            Gif87aEditCommand::SetPixelRegion { indices } => Ok(Emit::mutations(vec![GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index: 0, indices: indices.clone() })])),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_gif_87a_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(GIF_87A_DIALECT).document(["semio", "gif"]).icon_id("image").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).default_layout(edit::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_editor_builds_a_definition_for_the_editor_role() {
        let def = create_gif_87a_editor();
        assert_eq!(def.role, semio_framework::AppRole::Editor);
        assert_eq!(def.dialect, GIF_87A_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Gif87aEditor as ArtifactEditor>::DIALECT, GIF_87A_DIALECT);
    }
}
//#endregion 🧪️Tests
