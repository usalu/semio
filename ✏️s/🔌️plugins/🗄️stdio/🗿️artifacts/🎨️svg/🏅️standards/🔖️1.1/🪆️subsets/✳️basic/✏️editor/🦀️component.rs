//! ✏️ `svg` editor (basic) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! SVG has no pixel buffer: `set-pixel-region` replaces the whole vector snapshot via the artifact's own DSL text round-trip (`parse_dsl`/`SetSnapshot`), the closest real mutation this format declares — not a pixel edit.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::artifacts::svg::standards::v1_1::subsets::basic::schema::mutations::{set_snapshot, SvgBasicMutation};
use crate::artifacts::svg::standards::v1_1::subsets::basic::schema::snapshot::SvgSnapshot;
use crate::artifacts::svg::{STDIO_SVG_DOCUMENT_SCHEMA, SVG_BASIC_DIALECT};
use crate::editor::svg_basic::modes::edit;
use crate::editor::svg_basic::modes::edit::windows::main;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SvgBasicEditCommand {
    SetPixelRegion { source: String },
}

impl protocol::OpBinary for SvgBasicEditCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Malformed { what: "svg_basic-edit-command", offset: 0, detail: error.to_string() })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "svg_basic-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct SvgBasicEditor;

impl ArtifactEditor for SvgBasicEditor {
    type Snapshot = SvgSnapshot;
    type Mutation = SvgBasicMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SvgBasicEditCommand;

    const DIALECT: Dialect = SVG_BASIC_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_SVG_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Self::Snapshot {
        SvgSnapshot::default()
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
            SvgBasicEditCommand::SetPixelRegion { source } => match <SvgSnapshot as store::ArtifactDsl>::parse_dsl(source) {
                Ok(snapshot) => Ok(Emit::mutations(vec![SvgBasicMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot })])),
                Err(_) => Ok(Emit::default()),
            },
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
pub fn create_svg_basic_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(SVG_BASIC_DIALECT).document(["semio", "svg"]).icon_id("image").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).default_layout(edit::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_editor_builds_a_definition_for_the_editor_role() {
        let def = create_svg_basic_editor();
        assert_eq!(def.role, semio_framework::AppRole::Editor);
        assert_eq!(def.dialect, SVG_BASIC_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<SvgBasicEditor as ArtifactEditor>::DIALECT, SVG_BASIC_DIALECT);
    }
}
//#endregion 🧪️Tests
