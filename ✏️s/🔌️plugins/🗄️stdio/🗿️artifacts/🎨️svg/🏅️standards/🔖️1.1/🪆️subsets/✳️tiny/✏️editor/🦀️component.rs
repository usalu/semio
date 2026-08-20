//! ✏️ `svg` editor (tiny) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! SVG has no pixel buffer: `set-pixel-region` replaces the whole vector snapshot via the artifact's own DSL text round-trip (`parse_dsl`/`SetSnapshot`), the closest real mutation this format declares — not a pixel edit.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::artifacts::svg::{SVG_TINY_DIALECT, STDIO_SVG_DOCUMENT_SCHEMA};
use crate::artifacts::svg::standards::v1_1::subsets::tiny::schema::mutations::SvgMutation;
use crate::artifacts::svg::standards::v1_1::subsets::tiny::schema::snapshot::SvgSnapshot;
use crate::editor::svg_tiny::modes::edit;
use crate::editor::svg_tiny::modes::edit::windows::main;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SvgTinyEditCommand {
    SetPixelRegion { source: String },
}

impl protocol::OpBinary for SvgTinyEditCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Malformed { what: "svg_tiny-edit-command", offset: 0, detail: error.to_string() })
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "svg_tiny-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct SvgTinyEditor;

impl ArtifactEditor for SvgTinyEditor {
    type Snapshot = SvgSnapshot;
    type Mutation = SvgMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SvgTinyEditCommand;

    const DIALECT: Dialect = SVG_TINY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_SVG_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> Self::Snapshot {
        SvgSnapshot::default()
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
            SvgTinyEditCommand::SetPixelRegion { source } => match <SvgSnapshot as store::ArtifactDsl>::parse_dsl(source).await {
                Ok(snapshot) => Ok(Emit::mutations(vec![SvgMutation::SetSnapshot { snapshot }]).await),
                Err(_) => Ok(Emit::default()),
            },
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
pub fn create_svg_tiny_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(SVG_TINY_DIALECT)
        .document(["semio", "svg"])
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
        let def = create_svg_tiny_editor();
        assert_eq!(def.role, semio_framework::AppRole::Editor);
        assert_eq!(def.dialect, SVG_TINY_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<SvgTinyEditor as ArtifactEditor>::DIALECT, SVG_TINY_DIALECT);
    }
}
//#endregion 🧪️Tests
