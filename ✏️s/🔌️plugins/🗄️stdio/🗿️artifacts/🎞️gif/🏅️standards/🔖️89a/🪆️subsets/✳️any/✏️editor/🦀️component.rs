//! ✏️ `gif` editor (any) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Emits the frozen `set-pixel-region` action onto the artifact's own frame/image pixel-index replace mutation (index fixed at 0 — a genuine per-region patch is not declared in this format's schema).
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::artifacts::gif::{GIF_89A_DIALECT, STDIO_GIF_DOCUMENT_SCHEMA};
use crate::artifacts::gif::standards::v89a::subsets::any::schema::mutations::GifMutation;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;
use crate::editor::gif_89a::modes::edit;
use crate::editor::gif_89a::modes::edit::windows::main;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Gif89aEditCommand {
    SetPixelRegion { indices: Vec<u8> },
}

impl protocol::OpBinary for Gif89aEditCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Malformed { what: "gif_89a-edit-command", offset: 0, detail: error.to_string() })
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "gif_89a-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Gif89aEditor;

impl ArtifactEditor for Gif89aEditor {
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
    type Command = Gif89aEditCommand;

    const DIALECT: Dialect = GIF_89A_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_GIF_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> Self::Snapshot {
        GifSnapshot::default()
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
            Gif89aEditCommand::SetPixelRegion { indices } => Ok(Emit::mutations(vec![GifMutation::SetFramePixels { index: 0, indices: indices.clone() }]).await),
        }
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))).await,
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_gif_89a_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(GIF_89A_DIALECT)
        .document(["semio", "gif"])
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

    #[semio_framework_async_macros::async_test]
    async fn create_editor_builds_a_definition_for_the_editor_role() {
        let def = create_gif_89a_editor();
        assert_eq!(def.role, semio_framework::AppRole::Editor);
        assert_eq!(def.dialect, GIF_89A_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Gif89aEditor as ArtifactEditor>::DIALECT, GIF_89A_DIALECT);
    }
}
//#endregion 🧪️Tests
